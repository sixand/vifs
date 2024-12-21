#[allow(dead_code)]
use std::hash::{Hash, Hasher};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, Weak},
};

// 定义 DentryWrapper 类型，包含 Arc<Mutex<Dentry>> 以简化线程安全操作
#[allow(dead_code)]
type DentryGuard = Arc<Mutex<Dentry>>;

// 定义 Children 类型，使用 HashSet 存储 DentryWrapper
#[allow(dead_code)]
type HashsetChildren = Arc<Mutex<HashSet<DentryWrapper>>>;

// 定义 WeakDentry 类型，简化 Weak<Mutex<Dentry>> 的使用
#[allow(dead_code)]
type WeakDentry = Weak<Mutex<Dentry>>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DentryWrapper(DentryGuard);

#[derive(Debug)]
#[allow(dead_code)]
struct DentryGuardWrapper(DentryGuard);

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Dentry {
    endpoint: String,
    parent: Option<WeakDentry>,
    children: HashsetChildren,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DentryTable {
    root: DentryGuard,
    cursor: WeakDentry,
}

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
    #[allow(dead_code)]
    fn new(endpoint: &str, parent: Option<DentryGuard>) -> DentryGuard {
        let parent_weak = parent.as_ref().map(|p| Arc::downgrade(&p));
        let dentry = Dentry {
            endpoint: endpoint.to_string(),
            parent: parent_weak,
            children: Arc::new(Mutex::new(HashSet::new())),
        };
        Arc::new(Mutex::new(dentry))
    }

    #[allow(dead_code)]
    fn do_add(&mut self, endpoint: &str) -> Result<DentryGuardWrapper, &str> {
        let mut children_lock = self.children.lock().unwrap();

        for child in children_lock.iter() {
            let child_lock = child.0.lock().unwrap();
            if child_lock.endpoint == endpoint {
                return Err("Endpoint already exists");
            }
        }
        let parent_guard = self.get_parent();
        let new_dentry = Dentry::new(endpoint, parent_guard);
        children_lock.insert(DentryWrapper(new_dentry.clone()));

        Ok(DentryGuardWrapper(new_dentry))
    }

    #[allow(dead_code)]
    fn get_parent(&self) -> Option<DentryGuard> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    #[allow(dead_code)]
    fn get_children(&self) -> HashsetChildren {
        // 锁定 Mutex，获取 HashSet 的克隆
        let children_lock = self.children.lock().unwrap();
        let cloned_children = children_lock.clone();
        Arc::new(Mutex::new(cloned_children)) // 这里克隆 HashSet
    }

    #[allow(dead_code)]
    fn get_endpoint(&self) -> &str {
        &self.endpoint
    }

    #[allow(dead_code)]
    fn get_children_count(&self) -> usize {
        self.children.lock().unwrap().len()
    }

    #[allow(dead_code)]
    fn do_remove(&mut self, endpoint: &str) -> Result<(), String> {
        let mut children_lock = self.children.lock().unwrap();

        // 记录原始子元素数量
        let original_count = children_lock.len();

        // 使用 retain 来保留符合条件的元素
        children_lock.retain(|child| {
            let child_lock = child.0.lock().unwrap();
            if child_lock.endpoint == endpoint {
                // 如果子目录不为空，返回 true，表示保留
                if child_lock.get_children_count() > 0 {
                    return true; // 保留不删除
                }
                // 返回 false，表示要删除这个子目录
                false
            } else {
                // 返回 true，表示保留这个子目录
                true
            }
        });

        // 检查是否有元素被删除
        if original_count != children_lock.len() {
            Ok(())
        } else {
            Err(format!(
                "Child with endpoint '{}' not found or not empty.",
                endpoint
            ))
        }
    }

    #[allow(dead_code)]
    fn get_abspath(&self) -> String {
        println!("there is dentry pathabs: {:?} ", self);
        if let Some(ref parent) = self.parent {
            // 如果存在父目录，尝试将弱引用升级为强引用
            if parent.upgrade().is_none() {
                println!("parent is None",);
            } else {
                println!("parent is not None",);
            }
            if let Some(parent_arc) = parent.upgrade() {
                // 如果升级成功，锁定父目录并递归调用 get_abspath 方法
                let parent_path = parent_arc.lock().unwrap().get_abspath();
                // 将当前目录的 endpoint 添加到父目录路径之后
                format!("{}/{}", parent_path, self.endpoint)
            } else {
                // 如果升级失败，说明父目录已经被删除，这里可以处理错误或返回一个默认值
                "/".to_string() // 作为错误处理的示例，返回根目录路径
            }
        } else {
            // 如果当前目录没有父目录，说明当前目录是根目录
            "/".to_string()
        }
    }
}

#[allow(dead_code)]
trait DentryTableOptrator {
    fn init_root() -> Self;
    fn create(&mut self, endpoint: &str) -> Result<(), &str>;
    fn change(&mut self, endpoint: &str) -> Result<(), &str>;
    fn remove(&mut self, endpoint: &str) -> Result<(), &str>;
    fn list(&self) -> Vec<&str>;
    fn get_abspath(&self) -> String;
}

impl DentryTable {
    #[allow(dead_code)]
    fn init_root() -> Self {
        let root = Dentry::new("/", None);
        let cursor = Arc::downgrade(&root);
        DentryTable { root, cursor }
    }

    #[allow(dead_code)]
    fn create(&mut self, endpoint: &str) -> Result<(), &str> {
        // 尝试升级光标，获取当前工作目录的 Arc
        let work_arc = self.cursor.upgrade().ok_or("Failed to upgrade cursor")?;

        // 锁定工作目录并调用 do_add 方法
        let mut work_lock = work_arc.lock().unwrap();
        match work_lock.do_add(endpoint) {
            Ok(_) => {
                // 在这里我们将光标更新为新创建的目录
                // let new_cursor = Arc::downgrade(&new_dentry.0);
                Ok(())
            }
            Err(_) => Err("Failed to create"),
        }
    }

    #[allow(dead_code)]
    fn change(&mut self, endpoint: &str) -> Result<(), &str> {
        // 尝试升级光标，获取当前工作目录的 Arc
        let work_arc = self.cursor.upgrade().unwrap();

        // 如果目标是根目录
        if endpoint == "/" {
            self.cursor = Arc::downgrade(&self.root); // 使用 self.root
            return Ok(());
        }

        // 如果目标是 ".."
        if endpoint == ".." {
            // 尝试获取父目录的 Arc
            let parent_arc = work_arc.lock().unwrap().get_parent();

            if let Some(parent_arc) = parent_arc {
                self.cursor = Arc::downgrade(&parent_arc);
                return Ok(());
            } else {
                // 如果没有父目录，返回错误
                return Err("No parent directory or file");
            }
        }

        // 查找子目录
        let children = work_arc.lock().unwrap().get_children();
        let children_lock = children.lock().unwrap();

        for child in children_lock.iter() {
            let child_lock = child.0.lock().unwrap();
            if child_lock.endpoint == endpoint {
                self.cursor = Arc::downgrade(&child.0);
                return Ok(());
            }
        }

        Err("Not found directory or file")
    }

    #[allow(dead_code)]
    fn remove(&mut self, endpoint: &str) -> Result<(), &str> {
        // 尝试升级光标，获取当前工作目录的 Arc
        let work_arc = self.cursor.upgrade().unwrap();
        // 锁定工作目录并调用 do_remove 方法
        let work_lock = work_arc.lock().unwrap().do_remove(endpoint);

        // 如果删除成功，返回新的 DentryTable，否则返回错误
        if work_lock.is_ok() {
            return Ok(());
        } else {
            return Err("Failed to remove");
        }
    }

    #[allow(dead_code)]
    fn list(&self) -> Vec<String> {
        // 尝试升级光标，获取当前工作目录的 Arc
        let work_lock = self.cursor.upgrade().unwrap();

        // 锁定工作目录并获取子目录
        let children_lock = work_lock.lock().unwrap().get_children();

        let mut endpoints = Vec::new();
        for child in children_lock.lock().unwrap().iter() {
            let child_lock = child.0.lock().unwrap();
            endpoints.push(child_lock.endpoint.clone());
        }
        endpoints
    }

    #[allow(dead_code)]
    fn get_abspath(&self) -> String {
        // 尝试升级光标，获取当前工作目录的 Arc
        let workdir = self.cursor.upgrade().unwrap();
        let binding = workdir.lock().unwrap();
        let x = binding.get_abspath();
        println!("here is dentryTable abspath: {:?}", x);
        x.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dentry_table() {
        let mut dentry_table = DentryTable::init_root();

        let _ = dentry_table.create("dir1");
        let _ = dentry_table.create("dir2");
        let _ = dentry_table.create("dir3");
        let dir_dup = dentry_table.create("dir1");

        assert!(dir_dup.is_err());
        assert_eq!(dentry_table.list().len(), 3);
        dentry_table.change("dir1").unwrap();
        assert_eq!(dentry_table.list().len(), 0);
    }

    #[test]
    fn test_depth_table() {
        // 初始化 DentryTable
        let mut dentry_table = DentryTable::init_root();

        // 创建目录
        let _ = dentry_table.create("dir1");

        // 切换到 dir1
        let _ = dentry_table.change("dir1");

        // 创建子目录
        let _ = dentry_table.create("dir2");

        // 切换到 dir2
        let _ = dentry_table.change("dir2");

        let dir2_abspath = dentry_table.get_abspath();

        assert_eq!(dir2_abspath, "/dir1/dir2");

        // 切换回根目录
        let _ = dentry_table.change("..");

        let dir1_abspath = dentry_table.get_abspath();

        assert_eq!(dir1_abspath, "/dir1");

        // 切换回根目录
        let _ = dentry_table.change("..");

        let abspath = dentry_table.get_abspath();

        // 检查结果
        assert_eq!(abspath, "/");
    }

    #[test]
    fn test_list_dir() {
        // 初始化 DentryTable
        let mut dentry_table = DentryTable::init_root();

        // 创建目录
        let _ = dentry_table.create("dir1");
        let _ = dentry_table.create("dir2");
        let _ = dentry_table.create("dir3");

        // 列出根目录下的目录
        let dirs = dentry_table.list();

        // 检查结果
        assert_eq!(dirs.len(), 3);

        // 切换到 dir1
        let _ = dentry_table
            .change("dir1")
            .expect("Failed to change directory");

        let _ = dentry_table.create("dir4");

        let _ = dentry_table
            .change("dir4")
            .expect("Failed to change directory");

        assert_eq!(dentry_table.list().len(), 0);

        // assert_eq!(dentry_table.get_abspath(), "/dir1");

        println!("here is test path: {:?}", dentry_table.get_abspath())

        // 切换回根目录
        // let _ = dentry_table
        //     .create("dir4")
        //     .expect("Failed to change directory");

        // assert_eq!(dentry_table.list().len(), 1);

        // assert_eq!(dentry_table.get_abspath(), "/dir1");

        // let _ = dentry_table
        //     .change("/")
        //     .expect("Failed to change directory");

        // assert_eq!(dentry_table.get_abspath(), "/");

        // assert_eq!(dentry_table.list().len(), 3);
    }

    fn test_remove_exist_subdir_dir() {
        // 初始化 DentryTable
        let mut dentry_table = DentryTable::init_root();

        // 创建目录
        let _ = dentry_table.create("dir1");

        // 切换到 dir1
        let _ = dentry_table
            .change("dir1")
            .expect("Failed to change directory");

        // 创建子目录
        let _ = dentry_table
            .create("dir2")
            .expect("Failed to create directory");

        // 切换回根目录
        let result = dentry_table.change("..");

        // 检查结果
        assert!(result.is_ok());

        // 尝试删除 dir1
        let result = dentry_table.remove("dir1");

        // 检查结果
        assert!(result.is_err());
    }
}
