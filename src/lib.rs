use uuid::Uuid;
use sha2::{Digest, Sha256};
#[allow(unused_imports)]
use std::time::{SystemTime, UNIX_EPOCH};

pub const BLOCK_SIZE: usize = 512; // 块大小
pub const MAX_FILES: usize = 1024; // 文件夹最大文件数
pub const MAX_FILENAME_LEN: usize = 255; // 文件名最大长度
pub const DEFAULT_INODE_ID: usize = 0; // 默认的 inode id
pub const DEFAULT_FILE_PERMISSIONS: u16 = 0o655; // 默认的权限
pub const DEFAULT_DIR_PERMISSIONS: u16 = 0o655; // 默认的权限
// pub const DATA_DIR: string = "/data".to_string(); // 默认的数据路径
// pub const INODE_TABLE_DIR: string = ".inode_table.idx".to_string(); // 默认的 inode 表路径
// pub const BLOCK_TABLE_DIR: string = ".block_table.idx".to_string(); // 默认的 block 表路径
// pub const DENTRY_TABLE_DIR: string = ".dentry_table.idx".to_string(); // 默认的 dentry 表路径


// 获取当前时间戳
pub fn current_timestamp_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
}

pub fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn calculate_hash(data: &[u8], timestamp: u64) -> String {
    // 将数据和时间戳组合为一个字符串
    let combined_data = format!("{}{}", String::from_utf8_lossy(data), timestamp);

    // 创建 SHA-256 哈希器
    let mut hasher = Sha256::new();
    hasher.update(combined_data);

    // 计算哈希值并转换为十六进制字符串
    let result = hasher.finalize();
    format!("{:x}", result)
}

mod block;
// mod node;
mod dentry;
mod tests;