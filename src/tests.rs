
// #[cfg(test)]
// #[warn(unused_imports)]
// use super::*;
// #[warn(unused_imports)]
// use crate::block::*;
// #[warn(unused_imports)]
// use crate::node::*;

// #[test]
// fn write_data() {
//     let mut blk: Block = Block::new();
//     let write_data = "hello world".as_bytes();

//     blk.write(write_data);

//     let read_data = blk.read();

//     // 比较读取的数据是否与写入的数据相同
//     assert_eq!(read_data, write_data);

//     // 检查 is_dirty 是否被正确设置为
//     assert_eq!(blk.is_dirty, true);
// }

