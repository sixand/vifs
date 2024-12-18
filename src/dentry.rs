use std::hash::{Hash, Hasher};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, Weak},
};

#[derive(Debug)]
struct Dentry {
    endpoint: String,
    parent: Option<Weak<Mutex<Dentry>>>,
    children: Mutex<HashSet<DentryWrapper>>,
}

#[derive(Debug, Clone)]
struct DentryWrapper(Arc<Mutex<Dentry>>);

impl PartialEq for DentryWrapper {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.0.lock().unwrap();
        let other_lock = other.0.lock().unwrap();
        self_lock.endpoint == other_lock.endpoint
    }
}

impl Eq for DentryWrapper {}

impl Hash for DentryWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lock = self.0.lock().unwrap();
        lock.endpoint.hash(state);
    }
}

impl Dentry {
    fn new(endpoint: &str, parent: Option<Arc<Mutex<Dentry>>>) -> Arc<Mutex<Dentry>> {
        let parent_weak = parent.as_ref().map(|p| Arc::downgrade(&p));
        let dentry = Dentry {
            endpoint: endpoint.to_string(),
            parent: parent_weak,
            children: Mutex::new(HashSet::new()),
        };
        Arc::new(Mutex::new(dentry))
    }
}

#[derive(Debug)]
struct DentryTable {
    root: Arc<Mutex<Dentry>>,
    cursor: Weak<Mutex<Dentry>>,
}

impl DentryTable {
    fn init_root() -> Self {
        let root = Dentry::new("/", None);
        let cursor = Arc::downgrade(&root);
        DentryTable { root, cursor }
    }

    fn insert(&mut self, endpoint: &str) -> Arc<Mutex<Dentry>> {
        let parent_dentry = if let Some(cursor_arc) = self.cursor.upgrade() {
            Some(cursor_arc)
        } else {
            Some(self.root.clone())
        };

        let new_dentry = Dentry::new(endpoint, parent_dentry);
        let new_wrapper = DentryWrapper(new_dentry.clone());
        let root_lock = self.root.lock().unwrap();
        let mut children_lock = root_lock.children.lock().unwrap();
        children_lock.insert(new_wrapper);
        new_dentry
    }

    fn change_cursor(&mut self, endpoint: &str) -> Result<(), String> {
        let root_lock = self.root.lock().unwrap();
        let children_lock = root_lock.children.lock().unwrap();
        for child in children_lock.iter() {
            let child_lock = child.0.lock().unwrap();
            if child_lock.endpoint == endpoint {
                self.cursor = Arc::downgrade(&child.0);
                return Ok(());
            }
        }
        Err(format!("No such endpoint: {}", endpoint))
    }

    fn remove(&mut self, endpoint: &str) -> Result<(), String> {
        // 尝试从当前光标指向的目录中删除指定的目录项
        let cursor_arc = self
            .cursor
            .upgrade()
            .ok_or_else(|| format!("Cursor is invalid"))?;

        let cursor_lock = cursor_arc.lock().unwrap();
        let children_lock = cursor_lock.children.lock().unwrap();

        // 查找要删除的子项
        let child_option = children_lock.iter().find(|c| {
            let child_lock = c.0.lock().unwrap();
            child_lock.endpoint == endpoint
        });

        // 如果没有找到子项，返回错误
        let child = match child_option {
            Some(child) => child,
            None => return Err(format!("No such endpoint: {}", endpoint)),
        };

        // 锁定子项以进行操作
        let child_lock = child.0.lock().unwrap();
        if child_lock.children.lock().unwrap().is_empty() {
            // 重新锁定父目录以删除当前目录项
            let mut children_lock = cursor_lock.children.lock().unwrap();
            children_lock.remove(child);
            return Ok(());
        } else {
            return Err(format!("Cannot remove non-empty directory: {}", endpoint));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dentry_construction() {
        let endpoint = "test_endpoint";
        let dentry = Dentry::new(endpoint, None);
        let dentry_lock = dentry.lock().unwrap();

        assert_eq!(dentry_lock.endpoint, endpoint);
        assert!(dentry_lock.parent.is_none());
    }

    #[test]
    fn test_dentry_partial_eq() {
        let endpoint1 = "endpoint1";
        let endpoint2 = "endpoint2";

        let dentry1 = Dentry::new(endpoint1, None);
        let dentry2 = Dentry::new(endpoint2, None);

        let wrapper1 = DentryWrapper(dentry1.clone());
        let wrapper2 = DentryWrapper(dentry2.clone());

        assert_ne!(wrapper1, wrapper2);
    }

    #[test]
    fn test_dentry_table_insert_subdir() {
        let mut dentry_table = DentryTable::init_root();
        let _ = dentry_table.insert("subdir");

        assert!(dentry_table.change_cursor("subdir").is_ok());
        assert!(dentry_table.change_cursor("nonexistent").is_err());

        // Upgrade the cursor and check the endpoint
        if let Some(cursor_arc) = dentry_table.cursor.upgrade() {
            // Check the endpoint
            let cursor_lock = cursor_arc.lock().unwrap();
            assert_eq!(cursor_lock.endpoint, "subdir");

            // Check the parent
            if let Some(parent) = &cursor_lock.parent {
                // Upgrade the parent
                if let Some(parent_arc) = parent.upgrade() {
                    // Check the parent endpoint
                    let parent_lock = parent_arc.lock().unwrap();
                    assert_eq!(parent_lock.endpoint, "/");
                } else {
                    panic!("Parent should be valid.");
                }
            }
        } else {
            panic!("Cursor should be valid.");
        }
    }

    #[test]
    fn test_dentry_table_into_insert() {
        let mut dentry_table = DentryTable::init_root();
        let _ = dentry_table.insert("subdir");

        assert!(dentry_table.change_cursor("subdir").is_ok());

        let _ = dentry_table.insert("subdir2");

        assert!(dentry_table.change_cursor("subdir2").is_ok());
        // Upgrade the cursor and check the endpoint
        if let Some(cursor_arc) = dentry_table.cursor.upgrade() {
            // Check the endpoint
            let cursor_lock = cursor_arc.lock().unwrap();
            assert_eq!(cursor_lock.endpoint, "subdir2");

            // Check the parent
            if let Some(parent) = &cursor_lock.parent {
                // Upgrade the parent
                if let Some(parent_arc) = parent.upgrade() {
                    // Check the parent endpoint
                    let parent_lock = parent_arc.lock().unwrap();
                    assert_eq!(parent_lock.endpoint, "subdir");
                } else {
                    panic!("Parent should be valid.");
                }
            }
        } else {
            panic!("Cursor should be valid.");
        }

        let _ = dentry_table.insert("subdir3");

        assert!(dentry_table.change_cursor("subdir3").is_ok());
        // Upgrade the cursor and check the endpoint
        if let Some(cursor_arc) = dentry_table.cursor.upgrade() {
            // Check the endpoint
            let cursor_lock = cursor_arc.lock().unwrap();
            assert_eq!(cursor_lock.endpoint, "subdir3");

            // Check the parent
            if let Some(parent) = &cursor_lock.parent {
                // Upgrade the parent
                if let Some(parent_arc) = parent.upgrade() {
                    // Check the parent endpoint
                    let parent_lock = parent_arc.lock().unwrap();
                    assert_eq!(parent_lock.endpoint, "subdir2");
                } else {
                    panic!("Parent should be valid.");
                }
            }
        } else {
            panic!("Cursor should be valid.");
        }
    }

    #[test]
    fn test_dentry_table_remove() {
        let mut dentry_table = DentryTable::init_root();
        let subdir1 = dentry_table.insert("subdir1");
        let _ = dentry_table.insert("subdir2");

        // 在 subdir1 中插入一个子目录
        {
            let subdir1_lock = subdir1.lock().unwrap();
            let mut children_lock = subdir1_lock.children.lock().unwrap();
            children_lock.insert(DentryWrapper(dentry_table.insert("subdir1_child")));
        }

        // 尝试删除 subdir1，应该返回错误，因为它不为空
        assert!(dentry_table.remove("subdir1").is_err());

        // 尝试删除 subdir2，应该成功
        assert!(dentry_table.remove("subdir2").is_ok());

        // 删除 subdir1_child
        {
            let subdir1_lock = subdir1.lock().unwrap();
            let mut children_lock = subdir1_lock.children.lock().unwrap();
            children_lock.remove(&DentryWrapper(dentry_table.insert("subdir1_child")));
        }

        // 现在可以删除 subdir1
        assert!(dentry_table.remove("subdir1").is_ok());
    }
}
