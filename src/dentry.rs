use crate::inode::Inode;

pub(crate) struct Dentry {
    pub(crate) endpoint: String,
    pub(crate) parent: Option<Box<Dentry>>,
    pub(crate) inode: Box<Inode>,
}

impl Dentry {
    pub(crate) fn new(endpoint: String, parent: Option<Box<Dentry>>, inode: Box<Inode>) -> Self {
        Self {
            endpoint,
            parent,
            inode,
        }
    }
}