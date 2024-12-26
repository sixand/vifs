use crate::{dentry::Dentry, storage::Storage};

pub trait DentryTrait {}

pub trait InodeTrait {}

pub trait MetadataTrait {}

pub trait BlockTrait {}

pub trait FileTrait {}

pub trait FileOperations<B, D, F, S, E> {
    fn read(&mut self, index: usize) -> B;
    fn write(&mut self, fd: F, data: Vec<u8>);
    fn delete(&mut self);
    fn create(&mut self, filename: &str);
    fn open(&mut self) -> F;
    fn rename(&mut self, new_name: &str);
    fn cat(&self) -> B;
    fn stat(&self) -> S;
}

pub trait DirectoryOprations<D> {
    fn list(&self) -> Vec<D>;
    fn create(&mut self, name: &str);
    fn remove(&mut self);
    fn change(&self) -> String;
}

pub trait BlockReadWrite {
    fn read_block(&mut self, index: usize) -> &[u8];
    fn write_block(&mut self, index: usize, data: &[u8]);
}

pub trait BlockIterator<B> {
    fn next(&mut self) -> Option<&B>;
}

pub trait BlockAllocator<T> {
    fn allocate(&mut self, size: usize) -> Vec<Box<T>>;
    fn deallocate(&mut self, index: usize);
    fn is_allocated(&self, index: usize) -> bool;
}

// TODO: 存储连接接口,连接存储层，如挂载、创建、删除等操作
pub trait ConnectorStorage<'a, D, S, T> {
    fn mount(&'a mut self, root: Dentry, storage: Storage<T>) -> &'a Self;
    fn init(&mut self);
}

// 服务接口
pub trait ServiceTrait {
    fn put(&self, hash: &str, data: &[u8]) -> Result<(), &str>;
    fn get(&self, hash: &str) -> Vec<u8>;
    fn delete(&self, hash: &str);
    fn list(&self) -> Vec<String>;
    fn connect(&self) -> Result<(), &str>;
}

pub trait Finder {
    fn find(&self, name: &str) -> Option<u64>;
}

trait DentryOptions<D, I> {
    fn get_endpoint(&self) -> &String;
    fn get_parent(&self) -> &Option<Box<D>>;
    fn get_inode(&self) -> &Box<I>;
}

trait NodeOperations<I, B> {
    fn set_permissions(&mut self, permissions: u16) -> I;
    fn set_owner(&mut self, uid: u64, gid: u64) -> I;
    fn get_blocks(&self) -> B;
    fn is_deleted(&self) -> bool;
    fn do_version_up(&mut self) -> I;
    fn do_delete(&mut self) -> I;
}

trait InodeInfo {
    fn get_size(&self) -> u64;
    fn get_version(&self) -> u64;
    fn get_create_time(&self) -> u64;
    fn get_modified_time(&self) -> u64;
    fn get_first_block_hash(&self) -> String;
    fn get_last_block_hash(&self) -> String;
    fn get_links_count(&self) -> u64;
    fn get_premissions(&self) -> u16;
    fn get_uid(&self) -> u64;
}
