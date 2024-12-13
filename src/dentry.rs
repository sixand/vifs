const MAX_DEGREE: usize = 24; // 最大度数，即每个节点的最大子节点数量

type Dentries = Option<Box<Dentry>>;
type DentryChildren = Vec<Dentries>;

// 定义 DentryTable 结构体
#[derive(Debug, Clone)]
struct DentryTable {
    dentries: Dentries,  // 存储目录树的根节点
    degree: usize,                 // 每个节点的最大关键字数量
}

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

        // 从根节点开始搜索插入位置
        let mut current_dentry = self.dentries.as_mut().unwrap();

        if !current_dentry.is_leaf {
            // 如果当前节点不是叶子节点，则递归搜索插入位置
            let mut i = 0;
            while i < current_dentry.children.len() && current_dentry.children[i].as_ref().unwrap().pathname < pathname {
                i += 1;
            }

        Ok(())
    }

    fn get_absolute_path(&self, pathname: String) -> Option<String> {
        unimplemented!()
    }

    fn get_path(&self, dentry: &DentryTable, pathname: &str, current_path: String) -> Option<String>{
        unimplemented!()
    }

    fn search_dentry(&self, dentry: &Dentry, pathname: String) -> Option<&DentryTable>{
        unimplemented!()
    }
    fn search(&self, pathname: String) -> Result<(), String> {
        unimplemented!()
    }

    fn split_child(&mut self, parent: &mut Dentry, index: usize){
        unimplemented!()
    }

    fn is_full(&self, dentry: &Dentry) -> bool{
        unimplemented!()
    }

    fn insert_non_full(&mut self, dt: &mut DentryTable, pathname:String) -> Result<(), String>{
        unimplemented!()
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