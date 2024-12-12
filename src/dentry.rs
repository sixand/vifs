use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 定义 DentryTable 结构体
struct DentryTable {
    dentries: Vec<Rc<Dentry>>, // 存储所有 Dentry 节点的 Rc 指针
}

trait DentryTableOperations {
    // 创建一个新的 DentryTable
    fn new() -> DentryTable;

    // 添加一个新的 Dentry 节点，返回节点的引用
    fn add(&mut self, parent_index: Option<usize>) -> Rc<Dentry>;

    // 移除一个 Dentry 节点
    fn remove(&mut self, dentry: &Dentry) -> Result<&DentryTable, &'static str>;
}

impl DentryTableOperations for DentryTable {
    fn new() -> DentryTable {
        DentryTable {
            dentries: Vec::new(),
        }
    }

    fn add(&mut self, parent_index: Option<usize>) -> Rc<Dentry> {
        let index = self.dentries.len();
        let parent = parent_index.map(|i| Rc::downgrade(&self.dentries[i])); // 使用 Weak 指针
                                                                             // 创建一个新的 Dentry 节点
        let new_dentry = Dentry::new(index, parent);

        // 将 Rc 指针添加到 DentryTable
        self.dentries.push(new_dentry.clone());
        new_dentry // 返回新添加的 Dentry 的 Rc 指针
    }

    fn remove(&mut self, dentry: &Dentry) -> Result<&DentryTable, &'static str> {
        // 检查是否存在子目录, 若存在则不允许删除
        if !dentry.children.borrow().is_empty() {
            return Err("Cannot delete dentry with existing children.");
        }

        // 查找 Dentry 的索引并移除
        if let Some(pos) = self
            .dentries
            .iter()
            .position(|x| Rc::ptr_eq(x, &Rc::new(dentry.clone())))
        {
            self.dentries.remove(pos);
        }
        Ok(self) // 返回当前的 DentryTable
    }
}

#[derive(Debug,Clone)]
struct Dentry {
    // 存储目录树的节点引用
    index: usize,                       // 在 DentryTable 中的位置
    parent: Option<Weak<Dentry>>,       // 使用 Weak 指针来避免循环引用
    children: RefCell<Vec<Rc<Dentry>>>, // 存储子节点的 Rc 指针
}

trait DentryOperations {
    // 创建一个新的 Dentry 节点, 当初始化根目录时，parent应为 None
    fn new(index: usize, parent: Option<Weak<Dentry>>) -> Rc<Dentry>;

    // // 返回当前 Dentry 的子目录的引用，可能为空
    fn get_children(&self) -> Vec<Rc<Dentry>>;

    // 返回当前 Dentry 的父目录的引用，可能为 None
    fn get_parent(&self) -> Option<Rc<Dentry>>;

    // 获取兄弟目录项的引用，需要 DentryTable 来查找兄弟项
    // fn get_brother(&self, dentry_table: &DentryTable) -> Vec<&Dentry>;

    // // 获取当前节点的路径，返回一个包含所有父节点的向量
    // fn get_location(&self, dentry_table: &DentryTable) -> Vec<&Dentry>;
}

impl DentryOperations for Dentry {
    fn new(index: usize, parent: Option<Weak<Dentry>>) -> Rc<Dentry> {
        Rc::new(Dentry {
            index,
            parent,
            children: RefCell::new(Vec::new()),
        })
    }

    fn get_parent(&self) -> Option<Rc<Dentry>> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    fn get_children(&self) -> Vec<Rc<Dentry>> {
        self.children.borrow().clone() // 返回子节点的克隆
    }

    // fn get_brother(&self, dentry_table: &DentryTable) -> Vec<&Dentry> {
    //     if let Some(parent) = self.get_parent_dentry() {
    //         // 获取父目录的子目录，过滤掉当前目录
    //         parent
    //             .sub_dentry
    //             .iter()
    //             .filter(|sibling| sibling.index != self.index) // 过滤掉当前目录
    //             .cloned() // 使用 cloned() 来获取 Rc<Dentry> 的克隆
    //             .collect() // 收集到 Vec<Rc<Dentry>>
    //     }
    // }

    // fn get_location(&self, dentry_table: &DentryTable) -> Vec<&Dentry> {
    //     let mut location = Vec::new();
    //     let mut current_dentry = self;
    //     while let Some(parent) = current_dentry.get_parent_dentry() {
    //         location.push(parent);
    //         current_dentry = parent;
    //     }
    //     location.reverse(); // 反转顺序，使路径从根目录到当前目录
    //     location
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dentrytable_construction() {
        let mut dentry_table = DentryTable::new();
        let dentry = dentry_table.add(None);

        assert_eq!(dentry.index, 0);
    }

    #[test]
    fn test_dentry_construction() {
        let mut dentry_table = DentryTable::new();
        let dentry = dentry_table.add(None);

        let parent = dentry.get_parent();

        assert_eq!(dentry.index, 0);
        assert!(parent.is_none());
    }

    #[test]
    fn test_get_children_dentry() {
        let mut dentry_table = DentryTable::new();
        let dentry = dentry_table.add(None);

        let sub_dentry = dentry_table.add(Some(0));

        let sub_dentries = dentry.get_children();

        assert_eq!(sub_dentries.len(), 1);
        // assert_eq!(sub_dentries[0].index, 1);
        // assert!(sub_dentry.get_parent().is_none());
    }

    // #[test]
    // fn test_get_parent_dentry() {
    //     let dentry_table = DentryTable::new();
    //     let dentry = dentry_table.add(None);

    //     let children = dentry_table.add(Some(0));
    //     let sub_children = children.add(Some(1));
    //     let parent_dentry = dentry.get_parent();

    //     assert_eq!(children, sub_children.get_parent());
    //     assert_eq!(dentry, children.get_parent());
    //     assert_eq!(parent_dentry, None);
    // }

    // #[test]
    // fn test_get_brother_dentries() {
    //     let dentry_table = DentryTable::new();

    //     let parent_dentry = dentry_table.add(None);
    //     let sub_dentry = dentry_table.add(parent_dentry.index);

    //     let dentry1 = sub_dentry.add(sub_dentry.index);
    //     let dentry2 = sub_dentry.add(sub_dentry.index);

    //     let brother_dentries = dentry1.get_brother(&dentry_table);

    //     assert_eq!(brother_dentries.len(), 1);
    //     assert_eq!(brother_dentries[0].index, dentry1.index);
    // }

    // #[test]
    // fn test_get_location() {
    //     let dentry_table = DentryTable::new();
    //     let dentry1 = dentry_table.add(None);
    //     let dentry2 = dentry_table.add(dentry1.index);
    //     let dentry3 = dentry_table.add(dentry2.index);
    //     let dentry4 = dentry_table.add(dentry3.index);

    //     let location = dentry4.get_location(&dentry_table);

    //     // 检查路径是否正确
    //     assert_eq!(location.len(), 4);
    //     assert_eq!(location[0].index, dentry1.index);
    //     assert_eq!(location[1].index, dentry2.index);
    //     assert_eq!(location[2].index, dentry3.index);
    //     assert_eq!(location[3].index, dentry4.index);
    // }
}
