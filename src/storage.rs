use std::path::PathBuf;

// 存储结构
pub(crate) struct Storage {
    root: PathBuf, // 存储根目录
}

impl Storage {
    pub(crate) fn new(root: String) -> Self {
        Storage {
            root: PathBuf::from(root),
        }
    }
}