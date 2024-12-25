use crate::block::Block;
use std::collections::HashMap;

type BlockIndices = HashMap<usize, Block>;

pub(crate) struct Inode {
    id: String,
    raw_id: usize,
    block_indices: BlockIndices,
    hash: String,
}

trait NodeOperations {
    fn set_permissions(&mut self, permissions: u16) -> Inode;
    fn set_owner(&mut self, uid: u64, gid: u64) -> Inode;
    fn get_blocks(&self) -> BlockIndices;
    fn is_deleted(&self) -> bool;
    fn do_version_up(&mut self) -> Inode;
    fn do_delete(&mut self) -> Inode;
}
