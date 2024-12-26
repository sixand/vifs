// use crate::block::Block;
// use crate::dentry::Dentry;
// use crate::filesystem::Filesystem;
// use crate::inode::Inode;
// use crate::metadata::{FileType, Metadata, Permissions};
// use crate::storage::Storage;

// use crate::abstracts::{ConnectorStorage, FileOprations};

// use std::path::PathBuf;
// use std::{env, fs};

// pub struct PhysicalLayer {
//     filesystem: Filesystem,
//     storage: Storage,
//     root: Dentry,
// }

// impl ConnectorStorage for PhysicalLayer {
//     fn mount(&mut self, path: &str) {
//         let workspace = PathBuf::from(path);
//         match env::set_current_dir(&workspace) {
//             Ok(_) => {
//                 if !workspace.exists() {
//                     if let Err(e) = fs::create_dir_all(&workspace) {
//                         println!("Failed to create directory: {}", e);
//                     }
//                 }
//                 PhysicalLayer {
//                     storage: Storage::new(workspace.clone()),
//                     root: Dentry::new(
//                         String::from("root"),
//                         None,
//                         Box::new(Inode::new(String::from("root"))),
//                     ),
//                 };
//                 println!("Mounted at: {}", workspace.display());
//             }
//             Err(e) => {
//                 println!("Failed to mount: {}", e);
//             }
//         }
//     }
// }

// impl FileOprations<fs::File, Block, Dentry, Metadata> for PhysicalLayer {
//     fn create(&mut self, path: &str, file_type: FileType) -> Block {
//         if file_type == FileType::File {
//             let mut current = &mut self.root;
//             let first_block = Block::new();
//         }

//         match file_type {
//             FileType::File => {
//                 let mut current = &mut self.root;
//                 let mut inode = Inode::new(String::from(path));
//                 let mut matedata = Metadata::new(FileType::File);
//                 let mut dentry = Dentry {
//                     endpoint: String::from(path),
//                     parent: current,
//                     inode: Box::new(Inode::new(String::from(path))),
//                 };
//             }
//             FileType::Directory => {
//                 let mut current = &mut self.root;
//             }
//     }

//     fn read(&mut self, index: usize) -> Dentry {
//         todo!()
//     }

//     fn write(&mut self, index: usize, data: D) {
//         todo!()
//     }

//     fn delete(&mut self) {
//         todo!()
//     }

//     fn open(&mut self) -> F {
//         todo!()
//     }

//     fn rename(&mut self, new_name: &str) {
//         todo!()
//     }

//     fn cat(&self) -> B {
//         todo!()
//     }

//     fn stat(&self) -> S {
//         todo!()
//     }
// }
