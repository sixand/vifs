use std::{borrow::Borrow, cell::RefCell, rc::Rc};

type DentryType = Rc<RefCell<Dentry>>;

#[derive(Debug, Clone)]
struct Dentry {
    endpoint: String,
    parent: Option<DentryType>, // 使用 Rc
    children: Vec<DentryType>,  // 使用 Rc
}

#[derive(Debug)]
struct DentryTable {
    root: DentryType,
    degree: usize,
    load_factor: f64,
    threshold: f64,
}

impl Dentry {
    fn new(endpoint: String, parent: Option<DentryType>) -> Dentry {
        let new_dentry = Dentry {
            endpoint,
            parent,
            children: Vec::new(),
        };

        new_dentry
    }

    fn insert(&mut self, endpoint: String) -> Result<DentryType, &'static str> {
        println!("Inserting directory: {}", endpoint);
        println!("Current children count: {}", self.children.len());

        if endpoint.is_empty() {
            return Err("endpoint cannot be empty");
        }
        if self
            .children
            .iter()
            .any(|child| child.try_borrow().unwrap().endpoint == endpoint)
        {
            return Err("endpoint already exists");
        }

        let parent = Rc::new(RefCell::new(self.clone())); // 这里需要确保 self 是 Rc<Dentry>

        let new_dir = Rc::new(RefCell::new(Dentry::new(endpoint.clone(), Some(parent))));

        self.children.push(new_dir.clone());
        Ok(new_dir)
    }

    fn delete(&mut self, endpoint: &str) -> Result<(), &'static str> {
        if let Some(index) = self
            .children
            .iter()
            .position(|child| child.try_borrow().unwrap().endpoint == endpoint)
        {
            let child = &self.children[index];
            if child.try_borrow().unwrap().children.is_empty() {
                self.children.remove(index);
                return Ok(());
            } else {
                return Err("endpoint has children, cannot delete");
            }
        }
        Err("endpoint not found")
    }

    fn get_absolute_path(&self) -> String {
        let mut path = String::from("/"); // 从根路径开始
        let mut current: &Dentry = self;

        // 构建路径
        loop {
            path = format!("{}/{}", path, current.endpoint); // 将当前节点的 endpoint 加到路径的末尾
            if let Some(parent) = &current.parent {
                let parent_borrow = parent ; // 尝试借用父节点
                current = &parent_borrow; // 更新 current 为父节点
            } else {
                break; // 如果没有父节点，即到达根节点，退出循环
            }
        }

        path
    }

    fn search(&self, endpoint: String) -> Result<Vec<DentryType>, &str> {
        let mut results = Vec::new();
        self.search_recursive(&self.children, endpoint, &mut results);
        Ok(results)
    }

    fn search_recursive(
        &self,
        children: &Vec<DentryType>,
        endpoint: String,
        results: &mut Vec<DentryType>,
    ) {
        for child in children {
            if child.endpoint == endpoint {
                results.push(child.clone());
            }
            self.search_recursive(&child.children, endpoint.clone(), results);
        }
    }
}

impl DentryTable {
    fn init_root() -> Rc<RefCell<Dentry>> {
        let degree: usize = 2;
        let load_factor = 0.75;
        let threshold = load_factor * degree as f64;
        let root = Dentry::new(String::from("/"), None).unwrap();
        let root_table = DentryTable {
            root: Rc::new(RefCell::new(root.clone())),
            degree,
            load_factor,
            threshold,
        };
        root_table.root
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn test_new_dir() {
        let mut mount = DentryTable::init_root();
        let root_table = mount.get_mut();

        // 插入新目录并处理 Result
        let result = root_table.insert("dir1".to_string()).unwrap();

        // 验证根目录的 endpoint
        assert_eq!(result.endpoint, "/");

        // 搜索新插入的目录
        let search_result = root_table.search("dir1".to_string()).unwrap();

        // 验证搜索结果
        assert_eq!(result.children.len(), 1);

        assert_eq!(search_result[0].endpoint, result.endpoint);

        assert_eq!(search_result[0].get_absolute_path(), "/dir1");
    }

    #[test]
    fn test_insert_existing_dir() {
        let mount = DentryTable::init_root();
        let mut root_table = mount.clone(); // 克隆 Rc 以便后续使用

        {
            let root = root_table.borrow_mut(); // 获取可变借用
                                                // 第一次插入 "dir1"
            let _ = root
                .insert("dir1".to_string())
                .expect("Failed to insert dir1");
        }

        {
            let root = root_table.borrow_mut(); // 再次借用
            let dir1_again = root.get_mut().insert("dir1".to_string());

            // 验证插入失败
            assert!(dir1_again.is_err());
        }
    }

    #[test]
    fn test_insert_and_search() {
        let mount = DentryTable::init_root();
        let mut root_table = mount.borrow_mut();

        let _ = root_table.insert("dir1".to_string());
        let _ = root_table.insert("dir2".to_string());
        let _ = root_table.insert("dir3".to_string());
        let _ = root_table.insert("dir4".to_string());
        let _ = root_table.insert("dir22".to_string());

        let search_dir1 = root_table.search("dir1".to_string()).unwrap();
        let search_dir2 = root_table.search("dir2".to_string()).unwrap();
        let search_dir3 = root_table.search("dir3".to_string()).unwrap();
        let search_dir5 = root_table.search("dir5".to_string()).unwrap();

        assert_eq!(search_dir3.len(), 1);

        assert_eq!(search_dir3[0].endpoint, "dir3");

        assert_eq!(search_dir1.len(), 1);

        assert_eq!(search_dir1[0].endpoint, "dir1");

        assert_eq!(search_dir2.len(), 2); // 应该返回两个匹配项

        assert!(search_dir5.is_empty()); // dir3 不存在，结果应该为空
    }

    #[test]
    fn test_delete_existing_dir() {
        let mount = DentryTable::init_root();
        let mut root_table = mount.borrow_mut();

        let _ = root_table.get_mut().insert("dir1".to_string());
        let _ = root_table.get_mut().insert("dir2".to_string());

        let result = root_table.get_mut().delete("dir1");
        let search_dir1 = root_table.as_ref().search("dir1".to_string());

        assert!(result.is_ok());
        assert!(search_dir1.is_ok());
        assert!(search_dir1.unwrap().is_empty()); // dir1 已被删除，结果应该为空
    }

    #[test]
    fn test_search_empty_table() {
        let mount = DentryTable::init_root();
        let root_table = mount.borrow_mut();

        let result = root_table.search("dir1".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty()); // 空表应该返回空结果
    }

    #[test]
    fn test_deep_directory_structure() {
        let mount = DentryTable::init_root();
        let mut root_table = mount.borrow_mut(); // 初始化根目录, 这里假设已经实现了 DentryTable 的初始化和 insert 方法

        let mut current_dir = root_table.clone();

        for i in 0..=100 {
            let dir_name = format!("dir_{}", i);
            match current_dir.borrow_mut().insert(dir_name) {
                Ok(new_dir) => {
                    println!("New directory: {}", new_dir.get_absolute_path());
                    current_dir = Rc::new(new_dir.borrow_mut());
                }
                Err(_) => todo!(),
            }
        }
    }

    #[test]
    fn test_create_directory() {
        // 初始化 DentryTable
        let mount = DentryTable::init_root();
        let mut root_table = mount.borrow_mut();

        // 创建目录 `/plasma/test/dir1/dir2`
        let paths = vec!["plasma", "test", "dir1", "dir2"];
        let mut current = root_table;

        for path in paths {
            let current_mut = root_table; // 使用 make_mut
            match current_mut.insert(path.to_string()) {
                Ok(new_dir) => {
                    println!(
                        "Current children count after insertion: {}",
                        current_mut.children.len()
                    );
                    // 打印当前子目录的名称
                    for child in &current_mut.children {
                        println!("Child directory: {}", child.endpoint);
                    }
                    current = new_dir; // 更新当前目录为新创建的目录
                }
                Err(e) => {
                    panic!("Failed to create directory '{}': {}", path, e);
                }
            }
        }

        // 验证目录是否创建成功
        let expected_paths = vec!["plasma", "test", "dir1", "dir2"];
        let mut current = root_table;

        for path in expected_paths {
            let found = current.children.iter().find(|child| child.endpoint == path);
            assert!(found.is_some(), "Directory '{}' was not found", path);
            current = found; // 更新当前目录为找到的子目录
        }
    }
}
