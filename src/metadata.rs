use std::process::id;

pub(crate) enum FileType {
    File, // 文件
    Dir,  // 目录
    Link, // 符号链接
}

pub(crate) struct Metadata {
    // 存储文件或目录的元数据
    pub(crate) file_type: FileType,
    pub(crate) uid: u64,                 // 所属用户的 ID
    pub(crate) gid: u64,                 // 所属组的 ID
    pub(crate) size: usize,              // 文件大小
    pub(crate) premissions: u16,         // 权限
    pub(crate) links_count: usize,       // 链接数
    pub(crate) create_at: u64,           // 创建时间
    pub(crate) modified_at: u64,         // 最后修改时间
    pub(crate) version: usize,           // 版本号
    pub(crate) is_delete: bool,          // 是否被删除
    pub(crate) first_block_hash: String, // 第一个块的哈希值
}

enum Premissions {
    GroupRead = 0o400,
    GroupWrite = 0o200,
    GroupExecute = 0o100,
    UserRead = 0o40,
    UserWrite = 0o20,
    UserExecute = 0o10,
    OtherRead = 0o4,
    OtherWrite = 0o2,
    OtherExecute = 0o1,
}

impl Metadata {
    pub(crate) fn new(file_type: FileType) -> Self {
        Metadata {
            file_type,
            uid: 0,
            gid: 0,
            size: 0,
            premissions: 0,
            links_count: 0,
            create_at: current_timestamp_secs(),
            modified_at: current_timestamp_secs(),
            version: 0,
            is_delete: false,
            first_block_hash: String::new(),
        }
    }

    pub(crate) fn create_file(&mut self) {
        self.premissions = permissions;
    }
}