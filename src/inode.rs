use std::collections::HashMap;
use crate::block::Block;

type BlockIndices = HashMap<usize, Block>;

#[derive(Clone)]
pub(crate) struct Inode {
    id: String,
    block_indices: BlockIndices,
    hash: String,
}

impl Inode {
    pub(crate) fn new(id: String) -> Self {
        Self {
            id,
            block_indices: HashMap::new(),
            hash: String::new(),
        }
    }
}