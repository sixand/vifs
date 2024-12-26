use crate::abstracts::ServiceTrait;
use crate::dentry::Dentry;

// 存储结构
pub(crate) struct Storage<T> {
    pub(crate) service: T,
}

impl<T: ServiceTrait> Storage<T> {
    fn new(service: T) -> Self {
        Self {
            service,
        }
    }

    pub(crate) fn save(&mut self, data: &[u8]) -> String {
        // 保存数据
        String::from(format!("{:?}", data))
    }

    pub(crate) fn loading(&mut self, hash: &str) -> Vec<u8> {
        // 加载数据
        let _ = hash;
        vec![0; 1024]
    }

    pub(crate) fn binding(&mut self, root: &Dentry) {
        // 绑定根目录
        let _ = root;
    }
}