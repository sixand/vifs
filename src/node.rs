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
    raw_id: usize,
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
