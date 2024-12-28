use crate::abstracts::{
    BlockAllocator, ConnectorStorage,  FileOperations, ServiceTrait,
};
use crate::block::{Block, Superblock};
use crate::dentry::Dentry;
use crate::inode::Inode;
use crate::metadata::{FileType, Metadata};
use crate::storage::Storage;

pub struct Filesystem<T: ServiceTrait> {
    pub(crate) root: usize,                    // 指向根目录的index
    pub(crate) storage: Storage<T>,            // 存储结构
    pub(crate) current_dir: usize,             // 指向dentrys的index
    pub(crate) dentrys_list: Vec<Box<Dentry>>, // 存储所有目录项
    pub(crate) blocks: Superblock,
}

pub struct File {
    metadata: Metadata,
    inode: Inode,
    dentry: Dentry,
}

impl<T: ServiceTrait> FileOperations<Block, Dentry, File, Metadata> for Filesystem<T> {
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
        let root_dentry = Box::new(Dentry::new(
            "/".to_string(),
            None,
        ));

        let dentrys_list: Vec<Box<Dentry>> = vec![root_dentry.clone()];

        let root = dentrys_list.len();
        let current_dir = root;

        Self {
            root,
            storage,
            current_dir,
            dentrys_list,
            blocks: Superblock::new(),
        }
    }

    pub(crate) fn change_dir(&mut self, path: &str) {
        let root = self.root.clone();
        let mut current = &mut self.current_dir;
    }

    fn init(&mut self) {
        // 初始化文件系统
        // 加载根目录，加载目录项
        self.blocks.allocate(0);
    }
}

impl<'a, T: ServiceTrait> ConnectorStorage<'a, Dentry, Storage<T>, T> for Filesystem<T> {
    fn mount(&'a mut self, root: Dentry, storage: Storage<T>) -> &'a Self {
        // 挂载文件系统
        // 绑定根目录，绑定存储结构
        // self.root = Box::new(root); // 将根目录设置为新的Dentry
        self.storage = storage; // 设置存储结构
        // self.storage.binding(&self.root); // 绑定存储结构
        // self.current_dir = self.root.clone(); // 当前目录初始化为根目录的克隆
        self.init();
        self
    }

    fn init(&mut self) {
        // 初始化文件系统
        // 加载根目录，加载目录项
        // self.storage.binding(&mut self.root);
        self.storage.loading("0");
    }
}
