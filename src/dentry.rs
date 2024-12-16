use std::collections::BTreeMap;

// 定义 DentryTable 结构体
#[derive(Debug, Clone)]
struct Dentry {
    endpoint: String, // 目录名或文件名
    parent: Option<Box<Dentry>>,
    children: BTreeMap<String, Dentry>,
}

impl Dentry {
    // 新目录
    fn new(endpoint: &str, parent: Option<Box<Dentry>>) -> Self {
        Dentry {
            endpoint: endpoint.to_string(),
            parent,
            children: BTreeMap::new(),
        }
    }

    // 插入子目录
    fn do_add(&mut self, endpoint: &str) -> Result<Dentry, String> {
        if self.children.contains_key(endpoint) {
            return Err("Subdirectory already exists".to_string());
        }
        let child = Dentry::new(endpoint, Some(Box::new(self.clone())));
        self.children.insert(endpoint.to_string(), child.clone());

        Ok(child)
    }

    // 删除子目录
    fn do_remove(&mut self, endpoint: &str) -> bool {
        self.children.remove(endpoint).is_some()
    }

    // 列出当前所有子目录
    fn list(&self) -> Vec<&Dentry> {
        self.children.values().collect()
    }

    // 获取父目录
    fn get_parent(&self) -> Option<Box<Dentry>> {
        self.parent.clone()
    }

    // 获取绝对路径
    fn get_abspath(&self) -> String {
        let mut abspath = String::new();
        let mut current = Some(self);

        while let Some(dentry) = current {
            abspath.insert_str(0, &format!("{}/", dentry.endpoint.to_string()));
            current = dentry.parent.as_ref().map(|parent| &**parent);
        }

        // 处理根目录的情况
        if abspath.is_empty() {
            return "/".to_string();
        } else {
            // 去掉最后一个多余的斜杠
            if abspath.ends_with('/') {
                abspath.pop();
            }

            // 去掉开头的多余斜杠
            if abspath.starts_with("//") {
                abspath.remove(0);
            }
        }
        abspath
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_dentry_table() {
        let mut root_dentry = Dentry::new("/", None);

        assert_eq!(root_dentry.get_abspath(), "/".to_string());

        let dir1 = root_dentry.do_add("dir1");
        let dir2 = dir1.expect("REASON").do_add("dir2");
        assert_eq!(
            dir2.as_ref().unwrap().get_abspath(),
            "/dir1/dir2".to_string()
        );

        let dir3 = dir2.expect("REASON").do_add("dir3");
        assert!(dir3.is_ok());

        assert_eq!(
            dir3.as_ref().unwrap().get_abspath(),
            "/dir1/dir2/dir3".to_string()
        );

        let dir4 = dir3.expect("REASON").do_add("dir4.txt");

        assert_eq!(
            dir4.as_ref().unwrap().get_abspath(),
            "/dir1/dir2/dir3/dir4.txt".to_string()
        );
    }
}
