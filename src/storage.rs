use std::path::PathBuf;
use crate::{abstracts::{ConnectorStorage, FileOprations}, dentry::Dentry};

// 存储结构
pub(crate) struct Storage<T> {
    root: T, // 存储根目录
}