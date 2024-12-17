use std::{borrow::Borrow, rc::Rc};

#[derive(Debug, Clone)]
struct Dentry {
    endpoint: String,
    parent: Option<Rc<Dentry>>, // 使用 Rc
    children: Vec<Rc<Dentry>>,  // 使用 Rc
    is_leaf: bool,
}

impl Dentry {
    fn new(
        endpoint: String,
        parent: Option<Rc<Dentry>>,
        is_leaf: bool,
    ) -> Result<Rc<Dentry>, &'static str> {
        if endpoint.len() > 4096 {
            return Err("endpoint length exceeds 4096 characters");
        }
        let new_dentry = Dentry {
            endpoint,
            parent,
            children: Vec::new(),
            is_leaf,
        };

        Ok(Rc::new(new_dentry))
    }

    fn insert(&mut self, endpoint: String) -> Result<Rc<Dentry>, &'static str> {
        println!("Inserting directory: {}", endpoint);
        println!("Current children count: {}", self.children.len());

        if endpoint.is_empty() {
            return Err("endpoint cannot be empty");
        }
        if self.children.iter().any(|child| child.endpoint == endpoint) {
            return Err("endpoint already exists");
        }
        if self.is_leaf {
            self.is_leaf = false;
        }

        // 处理根目录的情况
        // let parent = if let Some(ref p) = self.parent {
        //     Some(Rc::clone(&p))
        // } else {
        //     None
        // };

        // let new_node = Dentry::new(endpoint.clone(), Some(Rc::new(self.clone())), true)?;
        // let new_node = Dentry::new(
        //     endpoint.clone(),
        //     Some(Rc::clone(&self.parent.as_ref().unwrap())),
        //     true,
        // )?;
        // self.children.push(new_node.clone());
        // Ok(new_node)

        // let new_node = Dentry::new(endpoint.clone(), parent, true)?;

        // let new_node = Dentry::new(endpoint.clone(), Some(Rc::new(self.clone())), true)?;

        // 处理根目录的情况
        // let parent = Some(Rc::new(self.clone()));
        let parent = Some(Rc::new(self.clone())); // 这里需要确保 self 是 Rc<Dentry>

        let new_node = Dentry::new(endpoint.clone(), parent, true)?;
        self.children.push(new_node.clone());
        Ok(new_node)
    }

    fn delete(&mut self, endpoint: &str) -> Result<(), &'static str> {
        if let Some(index) = self
            .children
            .iter()
            .position(|child| child.endpoint == endpoint)
        {
            let child = &self.children[index];
            if child.children.is_empty() {
                self.children.remove(index);
                return Ok(());
            } else {
                return Err("endpoint has children, cannot delete");
            }
        }
        Err("endpoint not found")
    }

    // fn get_absolute_path(&self) -> String {
    //     let mut path = String::new();
    //     let mut current = self;
    //     while let Some(parent) = current.parent.as_ref() {
    //         path = format!("/{}{}", current.endpoint, path);
    //         current = parent.borrow();
    //     }
    //     path
    // }
    fn get_absolute_path(&self) -> String {
        let mut path = String::new();
        let mut current = self;

        // 先构建路径，最后再加上当前节点的 endpoint
        while let Some(parent) = current.parent.as_ref() {
            path = format!("/{}", current.endpoint) + &path; // 先加当前节点
            current = parent.borrow(); // 使用 borrow() 来获取父目录
        }
        // path = format!("/{}", current.endpoint); // 加上根目录
        path
    }
}

#[derive(Debug)]
struct DentryTable {
    root: Rc<Dentry>,
    degree: usize,
    load_factor: f64,
    threshold: f64,
}

impl DentryTable {
    fn new(root: Rc<Dentry>, degree: usize, load_factor: f64) -> DentryTable {
        let threshold = load_factor * degree as f64;
        DentryTable {
            root,
            degree,
            load_factor,
            threshold,
        }
    }

    fn init_root() -> DentryTable {
        let root = Dentry::new(String::from("/"), None, false).unwrap();
        DentryTable {
            root,
            degree: 2,
            load_factor: 0.75,
            threshold: 2.0,
        }
    }

    fn insert(&mut self, endpoint: String) -> Result<Rc<Dentry>, &'static str> {
        Rc::get_mut(&mut self.root).unwrap().insert(endpoint)
    }

    fn delete(&mut self, endpoint: String) -> Result<(), &'static str> {
        Rc::get_mut(&mut self.root).unwrap().delete(&endpoint)
    }

    fn search(&self, endpoint: String) -> Result<Vec<Rc<Dentry>>, &str> {
        let mut results = Vec::new();
        self.search_recursive(&self.root, endpoint, &mut results);
        Ok(results)
    }

    fn search_recursive(
        &self,
        current: &Rc<Dentry>,
        endpoint: String,
        results: &mut Vec<Rc<Dentry>>,
    ) {
        if current.endpoint.starts_with(&endpoint) {
            // 使用 starts_with 进行部分匹配
            results.push(current.clone());
        }
        for child in &current.children {
            self.search_recursive(child, endpoint.clone(), results);
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::borrow::{Borrow, BorrowMut};
    use std::io::{self, Write};

    use super::*;

    #[test]
    fn test_new_dir() {
        let mut root = DentryTable::init_root();

        assert_eq!(root.root.endpoint, "/");

        let dir1 = root.insert("dir1".to_string()).unwrap();
        let search_dir1 = root.search("dir1".to_string()).unwrap();

        assert_eq!(search_dir1.len(), 1);
        assert_eq!(search_dir1[0].endpoint, dir1.endpoint);
        assert_eq!(search_dir1[0].get_absolute_path(), "/dir1");
    }

    #[test]
    fn test_insert_existing_dir() {
        let mut root = DentryTable::init_root();

        let _ = root.insert("dir1".to_string());
        let dir1_again = root.insert("dir1".to_string());

        assert!(dir1_again.is_err());
    }

    #[test]
    fn test_insert_and_search() {
        let mut root = DentryTable::init_root();

        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir2".to_string());
        let _ = root.insert("dir3".to_string());
        let _ = root.insert("dir4".to_string());

        let search_dir3 = root.search("dir3".to_string()).unwrap();
        assert_eq!(search_dir3.len(), 1);
        assert_eq!(search_dir3[0].endpoint, "dir3");
    }

    #[test]
    fn test_search_non_existent_dir() {
        let mut root = DentryTable::init_root();
        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir2".to_string());

        let result = root.search("dir3".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty()); // dir3 不存在，结果应该为空
    }

    #[test]
    fn test_search_existing_dir() {
        let mut root = DentryTable::init_root();
        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir2".to_string());

        let result = root.search("dir1".to_string()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].endpoint, "dir1");
    }

    #[test]
    fn test_search_partial_match() {
        let mut root = DentryTable::init_root();
        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir12".to_string());
        let _ = root.insert("dir2".to_string());

        let result = root.search("dir1".to_string()).unwrap();
        assert_eq!(result.len(), 2); // 应该返回两个匹配项
    }

    #[test]
    fn test_search_no_match() {
        let mut root = DentryTable::init_root();
        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir2".to_string());

        let result = root.search("dir3".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty()); // 没有匹配项，结果应该为空
    }

    #[test]
    fn test_delete_existing_dir() {
        let mut root = DentryTable::init_root();
        let _ = root.insert("dir1".to_string());
        let _ = root.insert("dir2".to_string());

        let result = root.delete("dir1".to_string());
        assert!(result.is_ok());
        let search_dir1 = root.search("dir1".to_string());
        assert!(search_dir1.is_ok());
        assert!(search_dir1.unwrap().is_empty()); // dir1 已被删除，结果应该为空
    }

    #[test]
    fn test_search_empty_table() {
        let root = DentryTable::init_root();
        let result = root.search("dir1".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty()); // 空表应该返回空结果
    }

    #[test]
    fn test_deep_directory_structure() {
        let root = DentryTable::init_root();
        let mut current_dir = Rc::clone(&root.root); // 使用 Rc<Dentry> 来保持当前目录

        // 创建 100 层深的目录
        for i in 1..=100 {
            let dir_name = format!("dir{}", i);
            let new_dir = Rc::make_mut(&mut current_dir)
                .insert(dir_name.clone())
                .unwrap();
            println!("{:?}", new_dir.clone());
            std::io::stdout().flush().unwrap(); // 强制刷新输出
            current_dir = new_dir; // 更新当前目录为新插入的目录
        }

        // 验证每一层的绝对路径
        for i in 1..=100 {
            let expected_path = format!(
                "/{}",
                (1..=i)
                    .map(|j| format!("dir{}", j))
                    .collect::<Vec<_>>()
                    .join("/")
            );
            let dir_name = format!("dir{}", i);

            println!("{:?}", expected_path);
            println!("{:?}", dir_name);
            std::io::stdout().flush().unwrap(); // 强制刷新输出
                                                // assert_eq!(search_result.len(), 1); // 应该找到一个匹配项
                                                // assert_eq!(search_result[0].get_absolute_path(), expected_path); // 验证路径是否一致
        }
    }

    #[test]
    fn test_create_directory() {
        // 初始化 DentryTable
        let dentry_table = DentryTable::init_root();

        // 创建目录 `/plasma/test/dir1/dir2`
        let paths = vec!["plasma", "test", "dir1", "dir2"];
        let mut current = Rc::clone(&dentry_table.root);

        for path in paths {
            let current_mut = Rc::make_mut(&mut current); // 使用 make_mut
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
        let mut current = Rc::clone(&dentry_table.root);

        for path in expected_paths {
            let found = current.children.iter().find(|child| child.endpoint == path);
            assert!(found.is_some(), "Directory '{}' was not found", path);
            current = found.unwrap().clone(); // 更新当前目录为找到的子目录
        }
    }
}
