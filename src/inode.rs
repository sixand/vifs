use std::collections::HashMap;
use crate::block::Block;

type BlockIndices = HashMap<usize, Block>;

#[derive(Clone)]
pub(crate) struct Inode {
    id: usize,
    block_indices: BlockIndices,
    hash: String,
}

impl Inode {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            block_indices: HashMap::new(),
            hash: String::new(),
        }
    }

    pub(crate) fn get_inode_id(&self) -> usize {
        self.id
    }
}