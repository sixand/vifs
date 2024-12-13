const MAX_DEGREE: usize = 24; // 最大度数，即每个节点的最大子节点数量

// 定义 DentryTable 结构体
struct DentryTable {
    dentries: Option<Box<Dentry>>,  // 存储目录树的根节点
    degree: usize,                 // 每个节点的最大关键字数量
}

type DentryChildren = Vec<Option<Box<DentryTable>>>;

#[derive(Debug, Clone)]
struct Dentry {
    // 存储目录树的节点引用
    pathname: String,       // 存储路径名
    children: DentryChildren,    // 存储子节点
    is_leaf: bool,               // 是否为叶子节点
}

impl Dentry  {
    // 构造函数，创建一个新的 Dentry 节点
    fn new(pathname: String, is_leaf: bool) -> Self {
        Dentry {
            pathname,
            children: Vec::new(),
            is_leaf,
        }
    }
}

impl DentryTable {
    // 构造函数，创建一个新的 DentryTable
    fn new(degree: usize) -> Self {
        DentryTable {
            dentries: None,
            degree,
        }
    }
}

trait BPlusTree {
    fn insert(&mut self, pathname: String) -> Result<(), String>;
    fn get_absolute_path(&self, pathname: String) -> Option<String>;
    fn get_path(&self, dentry: &DentryTable, pathname: &str, current_path: String) -> Option<String>;
    fn search_dentry(&self, dentry: &Dentry, pathname: String) -> Option<&DentryTable>;
    fn search(&self, pathname: String) -> Result<(), String>;
    fn split_child(&mut self, parent: &mut Dentry, index: usize);
    fn is_full(&self, dentry: &Dentry) -> bool;
    fn insert_non_full(&mut self, dt: &mut DentryTable, pathname:String) -> Result<(), String>;
}

impl BPlusTree for DentryTable {
    fn insert(&mut self, pathname: String) -> Result<(), String>   {
        if self.dentries.is_none() {
            // 如果根节点为空，创建一个新的根节点
            let new_dentry = Dentry::new(pathname.clone(), true);
            // 将新的根节点插入到树中
            self.dentries = Some(Box::new(new_dentry));
            return Ok(());
        }

        let root = self.dentries.as_mut().unwrap();
        if self.is_full(&*root) {
            // 如果根节点已满，创建一个新的根节点
            let mut new_root = Dentry::new(String::new(), false);
            // 将旧的根节点作为新的根节点的子节点
            new_root.children.push(Some(Box::new(root.clone())));
            // 将新的根节点插入到树中
    }

    fn is_full(&self, dentry: &Dentry) -> bool {
        // 判断目录树是否已满
        dentry.children.len() == self.degree
    }

    fn insert_non_full(&mut self, dt: &mut DentryTable, pathname:String) -> Result<(), String> {
        if dt.is_leaf {
            // 如果是叶子节点，直接插入
            dt.children.push(Some(Box::new(Dentry::new(pathname, true))));
            dt.children.sort_by(|a, b| a.as_ref().unwrap().pathname.cmp(&b.as_ref().unwrap().pathname)); // 排序
            return Ok(());
        } else {
            // 如果不是叶子节点，找到插入位置
            let mut i = dt.children.len() - 1;
            // 从右往左查找插入位置
            while i >= 0 && dt.children[i].as_ref().unwrap().pathname > pathname  {
                i -= 1;
            }

            if dt.children[i].is_none() {
                dt.children[i] = Some(Box::new(Dentry::new("".to_string(), false)));
            }

            let child = dt.children[i].as_mut().unwrap();
            if child.children.len() == self.degree {
                // 如果子节点已满，分裂子节点
                self.split_child(child, i);
                if pathname < child.pathname {
                    // 如果插入位置在左半部分，插入左半部分
                    self.insert_non_full(child, pathname);
                } else {
                    // 如果插入位置在右半部分，插入右半部分
                    self.insert_non_full(child, pathname);
                }
            } else {
                // 如果子节点未满，直接插入
                self.insert_non_full(child, pathname);
            }
        }
    }

    // 分裂子节点
    fn split_child(&mut self, parent: &mut Dentry, index: usize) {
        let degree = self.degree;
        let child = parent.children[index].as_mut().unwrap();
        let mut new_child = Dentry::new("".to_string(), child.is_leaf);

        for _ in 0..(degree / 2) {
            new_child.children.push(child.children.pop().unwrap());
        }

        if !child.is_leaf {
            for _ in 0..(degree / 2) {
                new_child.children.push(child.children.pop().unwrap());
            }
        }

        let child_key = child.pathname.clone();

        println!("child_key: {:?}", child_key);

        // 将中间键插入到父节点中
        parent.children.insert(index, Some(Box::new(new_child)));
    }

    fn search(&self, pathname: String) -> Result<(), String> {
        // 在树中查找指定的路径名
        if self.dentries.is_none() {
            return Err("DentryTable is empty".to_string());
        } else {
            let root_dentry = self.dentries.as_ref()?;
            return self.search_dentry(root_dentry, pathname);
        }
    }

    fn search_dentry(&self, dentry: &Dentry, pathname: String) -> Option<&DentryTable> {
        let mut i = 0;

        // 从左往右查找
        while i < dentry.children.len() && dentry.children[i].as_ref().unwrap().pathname < pathname {
            i += 1;
        }

        // 如果找到，返回
        if i < dentry.children.len() && dentry.children[i].as_ref().unwrap().pathname == pathname {
            return Some(dentry);
        }else{
            return None
        }

        // 如果不是叶子节点，递归查找
        self.search_dentry(dentry.children[i].as_ref()?, pathname)
    }

    fn get_absolute_path(&self, pathname: String) -> Option<String> {
        // 获取指定路径名的绝对路径
        self.get_path(self.dentries.as_ref()?, pathname, String::new())
    }

    fn get_path(&self, dentry: &DentryTable, pathname: &str, current_path: String) -> Option<String> {
        let mut i = 0;
        // 从左往右查找
        while i < dentry.pathname.len() && dentry.pathname[i] < pathname {
            i += 1;
        }

        if i < node.pathname.len() && node.pathname[i] > pathname {
            i += 1
        }

        if i < dentry.pathname.len() && dentry.pathname[i] == pathname {
            // 如果找到，返回
            return Some(format!("{}/{}", current_path, pathname));
        }

        if !dentry.is_leaf {
            // 如果不是叶子节点，递归查找
            return None
        }

        let next_path = format!("{}/{}", current_path, dentry.pathname[i]);
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_dentry_table() {
        let mut dentry_table = DentryTable::new(MAX_DEGREE);
        dentry_table.insert("a".to_string());
        dentry_table.insert("b".to_string());
        dentry_table.insert("c".to_string());
        dentry_table.insert("d".to_string());

        assert_eq!(dentry_table.search("a".to_string()).is_ok(), true);
    }
}