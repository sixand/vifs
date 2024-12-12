use crate::block::Block;
use crate::dentry::Dentry;
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


struct Node {
    id: String,
    file_type: FileType,
    metadata: Metadata,
    block_indices: Box<Block>,
    dentry: Dentry,
    hash: String,
}

trait NodeOperations {
    fn new(
        file_type: FileType,
        metadata: Metadata,
        block_indices: BlockTable,
        dentry: Rc<Dentry>,
    ) -> INode;

    fn set_permissions(&mut self, permissions: u16) -> INode;
    fn set_owner(&mut self, uid: u64, gid: u64) -> INode;
    fn get_blocks(&self) -> Vec<Block>;
    fn is_deleted(&self) -> bool;
    fn do_version_up(&mut self) -> INode;
    fn do_delete(&mut self) -> INode;
    fn do_remove(&mut self, block: &Block) -> &INode;
}


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



