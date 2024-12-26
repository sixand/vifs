use crate::current_timestamp_secs;

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

impl Metadata {
    pub(crate) fn new(file_type: FileType) -> Self {
        Self {
            file_type,
            uid: 0,
            gid: 0,
            size: 0,
            premissions: Permissions::default_file_permissions(),
            links_count: 0,
            create_at: current_timestamp_secs(),
            modified_at: current_timestamp_secs(),
            version: 0,
            is_delete: false,
            first_block_hash: String::new(),
        }
    }
}

pub(crate) enum Permissions {
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

impl Permissions {
    pub(crate) fn default_user_permissions() -> u16 {
        (Self::UserRead as u16) | (Self::UserWrite as u16) | (Self::UserExecute as u16)
    }

    pub(crate) fn default_group_permissions() -> u16 {
        (Self::GroupRead as u16) | (Self::GroupWrite as u16) | (Self::GroupExecute as u16) | (Self::OtherRead as u16) | (Self::OtherExecute as u16)
    }

    pub(crate) fn default_other_permissions() -> u16 {
        (Self::OtherRead as u16) | (Self::OtherWrite as u16) | (Self::OtherExecute as u16)
    }

    pub(crate) fn default_file_permissions() -> u16 {
        Self::default_user_permissions() | Self::default_group_permissions() | Self::default_other_permissions()
    }

    pub(crate) fn other_cant_write() -> u16 {
        (Self::OtherRead as u16) | (Self::OtherExecute as u16)
    }

    pub(crate) fn group_cant_write() -> u16 {
        (Self::GroupRead as u16) | (Self::GroupExecute as u16)
    }
}