use std::borrow::BorrowMut;


#[derive(Debug, Clone)]
struct Dentry<'a> {
    endpoint: String, // 目录名或文件名
    parent: Option<Box<Dentry<'a>>>,
    children: Vec<Box<Dentry<'a>>>,
    is_leaf: bool,
}

#[derive(Debug, Clone)]
struct DentryTable {
    root: Dentry<'static>,
    degree: usize,
    load_factor: f64, // 负载因子
    threshold: f64,   // 阈值
}

impl<'a> Dentry<'a> {
    fn new(
        endpoint: String,
        parent: Option<Box<Dentry<'a>>>,
        is_leaf: bool,
    ) -> Result<Dentry<'a>, &'static str> {
        if endpoint.len() > 4096 {
            return Err("endpoint length exceeds 4096 characters");
        }
        Ok(Dentry {
            endpoint,
            parent,
            children: vec![],
            is_leaf,
        })
    }

    fn get_absolute_path(&self) -> String {
        let mut path = String::new();
        let mut current = self;
        while let Some(parent) = current.parent.as_ref() {
            path = format!("/{}{}", current.endpoint, path);
            current = parent;
        }
        path
    }

    fn get_parent(&self) -> Option<&Dentry> {
        self.parent.as_ref().map(|v| &**v)
    }
}

impl DentryTable {
    fn new(root: Dentry<'static>, degree: usize, load_factor: f64) -> DentryTable {
        let threshold = load_factor * degree as f64;
        DentryTable {
            root,
            degree,
            load_factor,
            threshold,
        }
    }

    fn init_root() -> DentryTable {
        DentryTable {
            root: Dentry {
                endpoint: String::from("/"),
                parent: None,
                children: vec![],
                is_leaf: false,
            },
            degree: 2,
            load_factor: 0.75,
            threshold: 2.0,
        }
    }

    fn insert(&mut self, endpoint: String) -> Result<Box<Dentry>, &str> {
        todo!()
    }

    fn delete(&mut self, endpoint: String) -> Result<(), &str> {
        self.delete_recursive(&mut self.root.clone(), endpoint)
    }

    fn delete_recursive(&mut self, current: &mut Dentry, endpoint: String) -> Result<(), &str> {
        for (i, child) in current.children.iter_mut().enumerate() {
            if child.endpoint == endpoint {
                if child.is_leaf {
                    current.children.remove(i);
                    return Ok(());
                } else {
                    return Err("cannot delete non-leaf node");
                }
            }
        }
        Err("endpoint not found")
    }

    fn adjust_degree(&mut self) {
        let current_count = self.root.children.len();

        // 调整 degree
        if current_count > self.threshold as usize {
            self.degree += 1;
            self.threshold = self.load_factor * self.degree as f64;
        } else if current_count < self.threshold as usize / 2 && self.degree > 1 {
            self.degree -= 1;
            self.threshold = self.load_factor * self.degree as f64;
        }

        // 可能需要分裂或合并
        if current_count > self.degree {
            self.split();
        } else if current_count < self.degree / 2 {
            self.merge();
        }
    }

    fn split(&mut self) {
        let mid = self.root.children.len() / 2;

        // 将当前目录的子目录分为左右两部分
        let left_children = self.root.children[..mid].to_vec();
        let right_children = self.root.children[mid..].to_vec();

        // 创建左右两部分的目录
        let left = Dentry {
            endpoint: self.root.endpoint.clone(),
            parent: Some(Box::new(self.root.clone())),
            children: left_children,
            is_leaf: false,
        };

        let right = Dentry {
            endpoint: self.root.endpoint.clone(),
            parent: Some(Box::new(self.root.clone())),
            children: right_children,
            is_leaf: false,
        };

        // 将左右两部分的目录作为当前目录的子目录
        self.root.children = vec![Box::new(left), Box::new(right)];
    }

    fn merge(&mut self) {
        if self.degree > 1 {
            self.degree -= 1;
            let mut new_children = Vec::new();
            for child in &self.root.children {
                new_children.extend(child.children.iter().cloned());
            }
            self.root.children = new_children;
        }
    }

    fn search(&self, endpoint: String) -> Result<Box<Dentry>, &str> {
        self.search_recursive(&self.root, endpoint)
    }

    fn search_recursive(&self, current: &Dentry, endpoint: String) -> Result<Box<Dentry>, &str> {
        for child in &current.children {
            if child.endpoint == endpoint {
                return Ok(child.clone());
            }
            let result = self.search_recursive(child, endpoint.clone());
            if result.is_ok() {
                return result; // 找到后返回
            }
        }
        Err("not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dir() {
        let mut root = DentryTable::init_root();

        assert_eq!(root.root.endpoint, "/");

        let dir1 = root.root.insert("dir1".to_string()).unwrap();
        let search_dir1 = root.search("dir1".to_string()).unwrap();

        assert_eq!(search_dir1.endpoint, dir1.endpoint);
        assert_eq!(search_dir1.get_absolute_path(), "/dir1");
    }

    #[test]
    fn test_insert_existing_dir() {
        let mut root = DentryTable::init_root();

        let _ = root.root.insert("dir1".to_string());
        let dir1_again = root.root.insert("dir1".to_string());

        assert!(dir1_again.is_err());
    }

    #[test]
    fn test_insert_and_search() {
        let mut root = DentryTable::init_root();

        let _ = root.root.insert("dir1".to_string());
        let _ = root.root.insert("dir2".to_string());
        let _ = root.root.insert("dir3".to_string());
        let _ = root.root.insert("dir4".to_string());
        let search_dir1 = root.search("dir3".to_string()).unwrap();

        assert_eq!(search_dir1.endpoint, "dir3");
    }
}
