use std::borrow::Borrow;
// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 定义 DentryTable 结构体
struct DentryTable {
    dentries: Vec<Dentry>, // 存储所有 Dentry 节点的 Rc 指针
}

impl DentryTable {
    fn new() -> DentryTable {
        DentryTable {
            dentries: Vec::new(),
        }
    }
}

trait DentryTableOperations {
    // 获取所有 Dentry 节点的引用
    fn get_dentries(&self) -> Vec<Dentry>;

    // 添加一个新的 Dentry 节点，返回节点的引用
    fn add(&mut self, parent_index: Option<usize>) -> &Dentry;

    // 移除一个 Dentry 节点
    fn remove(&mut self, dentry: Dentry) -> Result<&DentryTable, &'static str>;
}

impl DentryTableOperations for DentryTable {
    fn get_dentries(&self) -> Vec<Dentry> {
        self.dentries.clone()
    }

    fn add(&mut self, parent_index: Option<usize>) -> Dentry {
        let index = self.dentries.len();
        let dentry = Dentry::new(index, parent_index);
    }

    fn remove(&mut self, dentry: Dentry) -> Result<&DentryTable, &'static str> {
        if !dentry.children.borrow().is_empty() {
            return Err("Cannot delete dentry with existing children.");
        }

        if let Some(index) = self.dentries.iter().position(|d| *d == dentry) {
            self.dentries.remove(index);
            Ok(self)
        }else {
            Err("Dentry not found.")
        }
    }
}

#[derive(Debug, Clone)]
struct Dentry {
    // 存储目录树的节点引用
    index: usize,                 // 在 DentryTable 中的位置
    parent: Option<Box<Dentry>>,       // 使用 Weak 指针来避免循环引用
    children: Vec<Dentry>,    // 存储子节点
}

impl Dentry {
    fn new(index: usize, parent: Option<Box<Dentry>>) -> Rc<Dentry> {
        Rc::new(Dentry {
            index,
            parent,
            children: Vec::new(),
        })
    }
}

// 定义 DentryOperations 特性，用于操作 Dentry 节点
trait DentryOperations {
    // // 返回当前 Dentry 的子目录的引用，可能为空
    fn get_children(&self) -> Vec<Dentry>;

    // 返回当前 Dentry 的父目录的引用，可能为 None
    fn get_parent(&self) -> Option<Dentry>;

    // 获取兄弟目录项的引用，需要 DentryTable 来查找兄弟项
    // fn get_brother(&self, dentry_table: &DentryTable) -> Vec<&Dentry>;

    // // 获取当前节点的路径，返回一个包含所有父节点的向量
    // fn get_location(&self, dentry_table: &DentryTable) -> Vec<&Dentry>;
}

impl DentryOperations for Dentry {
    fn get_parent(&self) -> Option<Dentry> {
        self.parent.as_ref().map(|p| p.borrow().clone()) // 克隆父目录的引用
    }

    fn get_children(&self) -> Vec<Dentry> {
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

        {
            let parent_index = Some(0);
            let table_ops: &mut dyn DentryTableOperations = &mut dentry_table;
            table_ops.add(parent_index);

            println!("{:?}", dentry_table.get_dentries());

            assert_eq!(dentry_table.get_dentries().len(), 1);
        }
    }

    #[test]
    fn test_dentry_relationship() {
        let mut dentry_table = DentryTable::new();

        {
            let table_ops: &mut dyn DentryTableOperations = &mut dentry_table;
            let parent = table_ops.add(None);
            let child = Dentry::new(1, Some(Rc::downgrade(&parent)));

            assert_eq!(child.get_parent().unwrap(), parent);
        }
        assert_eq!(dentry_table.dentries.len(), 2);
    }

    // #[test]
    // fn test_get_children_dentry() {
    //     let mut dentry_table = DentryTable::new();
    //     let dentry = dentry_table.add(None);

    //     let sub_dentry = dentry_table.add(Some(0));

    //     let sub_dentries = dentry.get_children();

    //     assert_eq!(sub_dentries.len(), 1);
    //     // assert_eq!(sub_dentries[0].index, 1);
    //     // assert!(sub_dentry.get_parent().is_none());
    // }

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
