use crate::block::Block;
use std::rc::Rc;
use std::collections::HashMap;
use crate::{BLOCK_SIZE, get_uuid,current_timestamp_secs, calculate_hash};


enum FileType {
    File, // 文件
    Dir,  // 目录
    Link, // 符号链接
}


struct File {
    name: String,
}


struct Dir {
    name: String,
}


struct Link {
    name: String,
}

struct Encryption {
    is_encrypted: bool,
    encrypt_algorithm: String,
}


struct Metadata {
    // 存储文件或目录的元数据
    uid: u64,                 // 所属用户的 ID
    gid: u64,                 // 所属组的 ID
    size: usize,              // 文件大小
    premissions: u16,         // 权限
    links_count: usize,       // 链接数
    create_at: u64,           // 创建时间
    modified_at: u64,         // 最后修改时间
    version: usize,           // 版本号
    is_delete: bool,          // 是否被删除
    first_block_hash: String, // 第一个块的哈希值
}

struct Dentry {
    // 存储目录树的节点引用
    sub_dentry: Vec<Rc<Dentry>>,          // 子目录
    parent_dentry: Option<*const Dentry>, // 父目录的指针
    index: usize,                         // 在 DentryTable 中的位置
}

trait DentryOperations {
    fn new(index: usize, parent: Option<*const Dentry>) -> Dentry;
    fn get_sub_dentry(&self) -> &Vec<Rc<Dentry>>;
    fn get_parent_dentry(&self) -> Option<&Dentry>;
    fn get_brother_dentries(&self, dentry_table: &DentryTable) -> &Vec<Rc<Dentry>>;
    fn get_location(&self, dentry_table: &DentryTable) -> Vec<&Dentry>;
}

struct DentryTable {
    // 存储所有 Dentry 节点
    dentries: Vec<Rc<Dentry>>, // 存储所有 Dentry 节点
}

trait DentryTableOperations {
    fn add_dentry(&mut self, parent_index: Option<usize>) -> Rc<Dentry>;
}

struct BlockTable {
    // 存储所有 Block 节点
    blocks: Vec<Block>, // 存储所有 Block 节点
}

struct INode {
    id: String,
    file_type: FileType,
    metadata: Metadata,
    blocks: Box<BlockTable>,
    dentry: Dentry,
    hash: String,
}


trait INodeOperations {
    fn new(
        file_type: FileType,
        metadata: Metadata,
        blocks: BlockTable,
        dentry: Rc<Dentry>,
    ) -> INode;

    fn delete(&mut self) -> INode;
    fn get_information(&self) -> &INode;
    fn set_permissions(&mut self, permissions: u16) -> INode;
    fn set_owner(&mut self, uid: u64, gid: u64) -> INode;
    fn version_up(&mut self) -> INode;
    fn is_deleted(&self) -> bool;
    fn get_block_indices(&self) -> Vec<Block>;
    fn remove(&mut self, block: &Block) -> &INode;
}

// impl DentryOperations for Dentry {
//     fn new(index: usize, parent: Option<*const Dentry>) -> Dentry {
//         Dentry {
//             sub_dentry: Vec::new(),
//             parent_dentry: parent,
//             index,
//         }
//     }

//     fn get_sub_dentry(&self) -> &Vec<Rc<Dentry>> {
//         // 直接返回子目录的引用
//         &self.sub_dentry
//     }

//     fn get_parent_dentry(&self) -> Option<&Dentry> {
//         // 返回父目录的引用
//         if let Some(parent) = self.parent_dentry {
//             unsafe { Some(&*parent) } // 解引用指针
//         } else {
//             None
//         }
//     }

//     fn get_brother_dentries(&self, dentry_table: &DentryTable) -> &Vec<Rc<Dentry>> {
//         if let Some(parent) = self.get_parent_dentry() {
//             parent
//                 .sub_dentry
//                 .iter()
//                 .filter(|&&sibling| sibling.index != self.index)
//                 .clone() 
//                 .collect()
//         } else {
//             Vec::new() // 如果没有父目录，则没有兄弟节点
//         }
//     }

//     fn get_location(&self, dentry_table: &DentryTable) -> Vec<&Dentry> {
//         let mut location = Vec::new();
//         let mut current_dentry = self;
//         while let Some(parent) = current_dentry.get_parent_dentry() {
//             location.push(parent);
//             current_dentry = parent;
//         }
//         location.reverse(); // 反转顺序，使路径从根目录到当前目录
//         location
//     }
// }




// impl DentryTableOperations for DentryTable {
//     fn new() -> DentryTable {
//         DentryTable {
//             dentries: Vec::new(),
//         }
//     }

//     fn add_dentry(&mut self, parent_index: Option<usize>) -> Rc<Dentry> {
//         let index = self.dentries.len();
//         let parent = parent_index.map(|i| &self.dentries[i] as *const Dentry);
//         let new_dentry = Rc::new(Dentry::new(index, parent));
//         self.dentries.push(new_dentry.clone());
//         new_dentry // 返回新添加的 Dentry 的引用
//     }
// }


// impl INodeOperations for INode {
//     fn new(
//         file_type: FileType,
//         metadata: Metadata,
//         blocks: BlockTable,
//         dentry: Some(Dentry),
//     ) -> INode {
//         // 如果 dentry 为 None，则创建一个root的 Dentry 节点
//         INode {
//             id: get_uuid(),
//             file_type,
//             metadata,
//             blocks,
//             dentry,
//         }
//     }

//     fn delete(&mut self) -> &INode {
//         // 实现删除逻辑
//         if let Some(ref mut metadata) = self.metadata {
//             metadata.is_deleted = true;
//             metadata.links_count -= 1;
//             metadata.modified_at = current_timestamp_secs();
//             metadata.version += 1;
//             metadata.size = 0;
//         }
//         self
//     }

//     fn get_information(&self) -> &INode {
//         // 返回节点的引用
//         self
//     }

//     fn set_permissions(&mut self, permissions: u16) {
//         // 实现设置权限的逻辑
//         if let Some(ref mut metadata) = self.metadata {
//             metadata.permissions = permissions;
//         }
//     }

//     fn set_owner(&mut self, uid: u64, gid: u64) {
//         // 实现设置所有者的逻辑
//         if let Some(ref mut metadata) = self.metadata {
//             metadata.uid = uid;
//             metadata.gid = gid;
//         }
//     }

//     fn version_up(&mut self) {
//         // 实现版本号增加的逻辑
//         if let Some(ref mut metadata) = self.metadata {
//             metadata.version += 1;
//         }
//     }

//     fn is_deleted(&self) -> bool {
//         // 实现判断是否被删除的逻辑
//         self.metadata.as_ref().map_or(false, |m| m.is_deleted)
//     }

//     fn get_block_indices(&self) -> Vec<Block> {
//         // 实现获取块索引的逻辑
//         self.blocks.blocks.clone() // 返回块的克隆
//     }

//     fn remove(&mut self, block: &Block) -> &INode {
//         // 实现移除块的逻辑
//         self.blocks.map(|mut block| {
//             block.is_deleted = true;
//             block.is_dirty = true;
//         });
//         self
//     }
// }




// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_dentry_construction() {
//         let dentry_table = DentryTable::new();
//         let dentry = dentry_table.add_dentry(None);

//         assert_eq!(dentry.index, 0);
//         assert_eq!(dentry.parent_dentry, None);
//     }

//     #[test]
//     fn test_get_sub_dentry() {
//         let dentry_table = DentryTable::new();
//         let dentry = dentry_table.add_dentry(None);

//         let sub_dentry = dentry_table.add_dentry(Some(0));

//         let sub_dentries = dentry.get_sub_dentry();

//         assert_eq!(sub_dentries.len(), 1);
//         assert_eq!(sub_dentries[0].index, 1);
//     }

//     #[test]
//     fn test_get_parent_dentry() {
//         let dentry_table = DentryTable::new();
//         let dentry = dentry_table.add_dentry(None);

//         let sub_dentry = dentry_table.add_dentry(Some(0));
//         let sub_sub_dentry = sub_dentry.add_dentry(Some(1));
//         let parent_dentry = dentry.get_parent_dentry();

//         assert_eq!(sub_dentry, sub_sub_dentry.get_parent_dentry());
//         assert_eq!(dentry, sub_dentry.get_parent_dentry());
//         assert_eq!(parent_dentry, None);
//     }

//     #[test]
//     fn test_get_brother_dentries() {
//         let dentry_table = DentryTable::new();

//         let parent_dentry = dentry_table.add_dentry(None);
//         let sub_dentry = dentry_table.add_dentry(parent_dentry.index);

//         let dentry1 = sub_dentry.add_dentry(sub_dentry.index);
//         let dentry2 = sub_dentry.add_dentry(sub_dentry.index);

//         let brother_dentries = dentry1.get_brother_dentries(&dentry_table);

//         assert_eq!(brother_dentries.len(), 1);
//         assert_eq!(brother_dentries[0].index, dentry1.index);
//     }

//     #[test]
//     fn test_get_location() {
//         let dentry_table = DentryTable::new();
//         let dentry1 = dentry_table.add_dentry(None);
//         let dentry2 = dentry_table.add_dentry(dentry1.index);
//         let dentry3 = dentry_table.add_dentry(dentry2.index);
//         let dentry4 = dentry_table.add_dentry(dentry3.index);


//         let location = dentry4.get_location(&dentry_table);

//         // 检查路径是否正确
//         assert_eq!(location.len(), 4);
//         assert_eq!(location[0].index, dentry1.index);
//         assert_eq!(location[1].index, dentry2.index);
//         assert_eq!(location[2].index, dentry3.index);
//         assert_eq!(location[3].index, dentry4.index);
//     }
// }
