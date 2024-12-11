use crate::block::*;
use crate::node::*;

// 存储结构
struct Storage {
    blocks: Vec<Block>,
    inodes: Vec<INode>,
    root_inode: INode,
}

trait StorageOperations {
    fn new() -> Storage;
    fn mount_storage(&mut self);
    fn umount_storage(&mut self);
    fn display_storage(&self);
    fn create_file(&mut self, name: &str, data: File) -> INode;
    fn read_file(&self, inode: &INode) -> File;
    fn delete_file(&mut self, inode: &INode) -> INode;
    fn rename_file(&mut self, inode: &INode, new_name: &str) -> INode;
    fn move_file(&mut self, inode: &INode, new_path: &str) -> INode;
    fn duplicate_file(&mut self, inode: &INode, new_path: &str) -> INode;
    fn get_file_size(&self, inode: &INode) -> usize;
    fn get_file_version(&self, inode: &INode) -> usize;
    fn clean_file(&mut self, inode: &INode) -> INode;
    fn get_storage_capacity(&self) -> usize;
}