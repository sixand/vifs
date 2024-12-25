use crate::inode::Inode;

pub(crate) struct Dentry {
    endpoint: String,
    parent: Option<Box<Dentry>>,
    inode: Box<Inode>,
}

trait DentryOptions {
    fn get_endpoint(&self) -> &String;
    fn get_parent(&self) -> &Option<Box<Dentry>>;
    fn get_inode(&self) -> &Box<Inode> ;
}