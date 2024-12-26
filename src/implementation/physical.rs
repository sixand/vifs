use crate::filesystem::Filesystem;
use crate::storage::Storage;
use crate::abstracts::ServiceTrait;
use std::{
    env,
    fs::{self, File},
    path::{self, PathBuf},
};

pub struct PhysicalService;
pub struct PhysicalFs;

const WORKDIR: &str = "./";

impl ServiceTrait for PhysicalService {
    fn connect(&self) -> Result<(), &str> {
        let work_dir = PathBuf::from(WORKDIR);

        if !fs::exists(work_dir.clone()).unwrap() {
            fs::create_dir(work_dir).unwrap();
        }
        env::current_dir().unwrap();
        Ok(())
    }

    fn get(&self, hash: &str) -> Vec<u8> {
        if fs::exists(hash).unwrap() {
            fs::read(hash).unwrap()
        } else {
            vec![]
        }
    }

    fn put(&self, hash: &str, data: &[u8]) -> Result<(), &str> {
        if fs::exists(hash).unwrap() {
            let md = fs::metadata(path::Path::new(hash)).unwrap();
            if md.is_file() {
                fs::write(hash, data).expect_err("文件写入失败");
                Ok(())
            } else {
                Err("对象不可读写")
            }
        } else {
            Err("文件不存在")
        }
    }

    fn delete(&self, hash: &str) {
        let _ = hash;
        unimplemented!()
    }

    fn list(&self) -> Vec<String> {
        fs::read_dir(WORKDIR)
            .unwrap()
            .map(|res| res.unwrap().path().display().to_string())
            .collect()
    }
}

impl PhysicalFs {
    pub fn new() -> Filesystem<PhysicalService> {
        Filesystem::new(Storage {
            service: PhysicalService,
        })
    }

    pub fn flush(&self) {
        unimplemented!()
    }

    pub fn sync(&self) {
        unimplemented!()
    }

    pub fn close(&self) {
        unimplemented!()
    }

    pub fn open(&self) {
        unimplemented!()
    }

    pub fn write(&self, hash: &str) -> Vec<u8> {
        unimplemented!()
    }

    pub fn read(&self, hash: &str, data: &[u8]) -> Result<(), &str> {
        unimplemented!()
    }
}
