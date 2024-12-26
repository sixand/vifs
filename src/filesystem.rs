use crate::abstracts::{
    BlockAllocator, BlockTrait, ConnectorStorage, DentryTrait, DirectoryOprations, FileOperations, FileTrait, InodeTrait, MetadataTrait, ServiceTrait
};
use crate::block::{Block,Superblock};
use crate::dentry::Dentry;
use crate::inode::Inode;
use crate::metadata::{FileType, Metadata};
use crate::storage::Storage;

use std::fs;

pub struct Filesystem<T: ServiceTrait> {
    pub(crate) root: Box<Dentry>,        // 根目录
    pub(crate) storage: Storage<T>,      // 存储结构
    pub(crate) current_dir: Box<Dentry>, // 当前目录
    pub(crate) blocks: Superblock,
}

pub struct File {
    metadata: Metadata,
    inode: Inode,
    dentry: Dentry,
}

impl<T: ServiceTrait> FileOperations<Block, Dentry, File, Metadata> for Filesystem<T>
{
    fn read(&mut self, index: usize) -> Block {
        unimplemented!()
    }

    fn write(&mut self, fd: File, data: Vec<u8>) {
        let len = data.len();
        // Superblock::new().write_block(index, &data);
    }

    fn delete(&mut self) {
        todo!()
    }

    fn create(&mut self, filename: &str) {
        todo!()
    }

    fn open(&mut self) -> File {
        todo!()
    }

    fn rename(&mut self, new_name: &str) {
        todo!()
    }

    fn cat(&self) -> Block {
        todo!()
    }

    fn stat(&self) -> Metadata {
        todo!()
    }
}

pub struct Dir {
    metadata: Metadata,
    inode: Inode,
    dentry: Dentry,
}

impl<T: ServiceTrait> Filesystem<T> {
    pub(crate) fn new(storage: Storage<T>) -> Self {
        Self {
            root: Box::new(Dentry::new(
                "/".to_string(),
                None,
                Box::new(Inode::new("0".to_string())),
            )),
            storage,
            current_dir: Box::new(Dentry::new(
                "/".to_string(),
                None,
                Box::new(Inode::new("0".to_string())),
            )),
            blocks: Superblock::new(),
        }
    }

    fn init(&mut self) {
        // 初始化文件系统
        // 加载根目录，加载目录项
        self.blocks.allocate(size);
    }
}

impl<'a, T: ServiceTrait> ConnectorStorage<'a, Dentry, Storage<T>, T> for Filesystem<T> {
    fn mount(&'a mut self, root: Dentry, storage: Storage<T>) -> &'a Self {
        // 挂载文件系统
        // 绑定根目录，绑定存储结构
        self.root = Box::new(root); // 将根目录设置为新的Dentry
        self.storage = storage; // 设置存储结构
        self.storage.binding(&self.root); // 绑定存储结构
        self.current_dir = self.root.clone(); // 当前目录初始化为根目录的克隆
        self.init();
        self
    }

    fn init(&mut self) {
        // 初始化文件系统
        // 加载根目录，加载目录项
        self.storage.binding(&mut self.root);
        self.storage.loading("0");
    }
}
