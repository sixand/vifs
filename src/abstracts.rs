pub trait FileOprations<S, F, D, B> {
    fn read(&mut self, index: usize) -> B;
    fn write(&mut self, index: usize, data: D);
    fn delete(&mut self);
    fn create(&mut self);
    fn open(&mut self) -> F;
    fn rename(&mut self, new_name: &str);
    fn cat(&self) -> B;
    fn stat(&self) -> S;
}

pub trait DirectoryOprations<D> {
    fn list(&self) -> Vec<D>;
    fn create(&mut self, name: &str);
    fn remove(&mut self);
    fn pwd(&self) -> String;
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

pub trait ConnectorStorage<D, T> {
    // TODO: 连接存储层，如挂载、创建、删除等操作
    fn mount(&mut self, root: D, storage: T);
    fn create(&mut self, name: &str, size: usize);
    fn delete(&mut self, name: &str);
    fn flush(&mut self);
}

impl dyn ConnectorStorage<Dentry, Storage> {
    fn mount(&mut self, root: Dentry, storage: Storage) {
        // 挂载存储层
    }
}

pub trait Finder {
    fn find(&self, name: &str) -> Option<u64>;
}

trait DentryOptions<D, I> {
    fn get_endpoint(&self) -> &String;
    fn get_parent(&self) -> &Option<Box<D>>;
    fn get_inode(&self) -> &Box<I> ;
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