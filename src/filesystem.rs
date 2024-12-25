use crate::dentry::Dentry;
use crate::storage::Storage;
use crate::metadata::{Metadata, FileType};



struct Filesystem {
    root: Dentry,
    storage: Storage,
}

impl Filesystem {
    fn mount_storage(&mut self) {
        Storage::new("./data".to_owned());
    }
    
    fn create_file(&mut self, path: &str) -> Result<(), String> {
        Metadata {
            file_type: FileType::File,
            uid: id::uid(),
            gid: todo!(),
            size: todo!(),
            premissions: todo!(),
            links_count: todo!(),
            create_at: todo!(),
            modified_at: todo!(),
            version: todo!(),
            is_delete: todo!(),
            first_block_hash: todo!(),
        }
}