// Compare this snippet from block.rs:
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{BLOCK_SIZE, get_uuid, current_timestamp_nanos, calculate_hash};

pub struct Block {
    id: String,
    data: [u8; BLOCK_SIZE],             // 数据
    valid_length: usize,                // 有效数据长度
    is_dirty: bool,                     // 是否被修改
    is_deleted: bool,                   // 是否被删除
    timestamp: u128,                    // 时间戳
    last_access_time: u128,             // 上次访问时间
    previous_block: Option<Box<Block>>, // 前一个块
    hash: String,                       // 哈希值
}

trait BlockChainOperations {
    fn new() -> Block;
    // 读取数据
    fn read(&self) -> &[u8];
    // 写入数据
    fn write(&mut self, data: &[u8]) -> Result<&mut Block, String>;
    // 读取指定偏移量和长度的数据
    fn read_at(&self, offset: usize, size: usize) -> Result<&[u8], String>;
    // 获取有效数据长度
    fn get_valid_length(&self) -> usize;
    // 获取哈希值
    fn get_hash(&self) -> &str;
    // 获取上次访问时间
    fn get_last_access_time(&self) -> u128;
    // 获取前一个块
    fn get_previous_block(&self) -> Option<&Block>;
}

trait BlockOperations {
    fn new() -> Block;
    // 读取数据
    fn read(&self) -> &[u8];
    // 写入数据
    fn write(&mut self, data: &[u8]) -> Result<&mut Block, String>;
    // 读取指定偏移量和长度的数据
    fn read_at(&self, offset: usize, size: usize) -> Result<&[u8], String>;
    // 写入指定偏移量和长度的数据
    fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<&mut Block, String>;
    // 追加数据
    fn append(&mut self, data: &[u8]) -> Result<&mut Block, String>;
    // 追加指定偏移量和长度的数据
    fn append_at(&mut self, offset: usize, data: &[u8]) -> Result<&mut Block, String>;
    // 截断数据
    fn truncate(&mut self, size: usize) -> &mut Block;
    // 获取有效数据长度
    fn get_valid_length(&self) -> usize;
    // 获取是否被修改
    fn is_dirty(&self) -> bool;
    // 标记为脏
    fn touch(&mut self) -> &mut Block;
    // 获取是否被删除
    fn is_deleted(&self) -> bool;
    // 标记为删除
    fn delete(&mut self);
    // 标记已冲洗并刷新时间戳和哈希值
    fn flush(&mut self);
    // 获取上次访问时间
    fn get_last_access_time(&self) -> u128;
    // 获取哈希值
    fn get_hash(&self) -> &str;
}

impl BlockOperations for Block {
    fn new() -> Block {
        Block {
            id: get_uuid(),
            data: [0; BLOCK_SIZE],
            valid_length: 0,
            is_dirty: false,
            is_deleted: false,
            timestamp: 0,
            last_access_time: 0,
            previous_block: None,
            hash: String::new(),
        }
    }

    fn read(&self) -> &[u8] {
        &self.data[..self.valid_length]
    }

    fn write(&mut self, data: &[u8]) -> Result<&mut Block, String> {
        let len = data.len();

        // 检查是否有足够的空间来写入数据
        if len > BLOCK_SIZE {
            return Err("data is too large".to_string());
        }

        // 将提供的数据复制到块的 data 数组中
        self.data[..len].copy_from_slice(data);
        self.valid_length = len;
        self.timestamp = current_timestamp_nanos();
        self.hash = calculate_hash(&self.data, self.timestamp.try_into().unwrap());

        Ok(self.touch())
    }

    fn read_at(&self, offset: usize, size: usize) -> Result<&[u8], String> {
        if offset + size > self.valid_length {
            return Err("Read out of bounds".to_string());
        }
        Ok(&self.data[offset..offset + size])
    }

    fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<&mut Block, String> {
        let len = data.len();
        // 检查是否有足够的空间来写入数据
        if offset + len > BLOCK_SIZE {
            return Err("Expected an error when writing beyond block size".to_string());
        }
        // 将提供的数据复制到块的 data 数组中
        self.data[offset..offset + len].copy_from_slice(data);
        self.valid_length = offset + len; // 更新有效长度
        self.is_deleted = false;

        Ok(self.touch())
    }

    fn append(&mut self, data: &[u8]) -> Result<&mut Block, String> {
        let data_len = data.len();
        if data_len > BLOCK_SIZE {
            return Err("data is too large, you need apply new block".to_string());
        }

        let start = self.valid_length; // 计算append数据的起始位置

        self.data[start..start + data_len].copy_from_slice(data); // 将数据追加到块的 data 数组中

        self.valid_length += data_len; // 更新有效长度
        self.last_access_time = current_timestamp_nanos();

        Ok(self.touch())
    }

    fn append_at(&mut self, offset: usize, data: &[u8]) -> Result<&mut Block, String> {
        let data_len = data.len();
        // 检查是否有足够的空间来写入数据
        if offset + data_len > BLOCK_SIZE {
            return Err("Append out of bounds".to_string());
        }
        // 将提供的数据追加到块的 data 数组中
        self.data[offset..offset + data_len].copy_from_slice(data);
        self.valid_length = offset + data_len; // 更新有效长度

        Ok(self.touch())
    }

    fn truncate(&mut self, size: usize) -> &mut Block {
        if size < self.valid_length {
            self.valid_length = size;
            self.data[size..].fill(0);
            return self.touch()
        }
        self
    }

    fn get_valid_length(&self) -> usize {
        self.valid_length
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn touch(&mut self) -> &mut Block {
        self.last_access_time = current_timestamp_nanos();
        self.is_dirty = true;
        self.hash = calculate_hash(&self.data, self.timestamp.try_into().unwrap());
        self
    }

    fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn get_last_access_time(&self) -> u128 {
        self.last_access_time.try_into().unwrap()
    }

    fn delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }

    fn flush(&mut self) {
        self.is_dirty = false;
        self.last_access_time = current_timestamp_nanos();
    }

    fn get_hash(&self) -> &str {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_write() {
        let mut block = Block::new();

        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);

        // 读取数据
        let read_data = block.read();

        // 将字节切片转换为字符串
        let read_data_str = std::str::from_utf8(read_data).unwrap();
        let data_to_write_str = std::str::from_utf8(data_to_write).unwrap();

        assert_eq!(read_data_str, data_to_write_str);
    }

    #[test]
    fn test_block_write_at_data() {
        let mut block = Block::new();
        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);
        // 写入指定偏移量和长度的数据
        // 尝试写入超出块大小的数据
        let new_data = b"Six"; // 要插入的单个字节
        let offset = 7; // 从偏移量 6 开始写入

        let result = block.write_at(offset, new_data);
        let read_data_str = std::str::from_utf8(block.read()).unwrap();

        assert_eq!(read_data_str, "Hello, Six");
    }

    #[test]
    fn test_block_write_at_overflow() {
        let mut block = Block::new();
        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);
        // 写入指定偏移量和长度的数据
        // 尝试写入超出块大小的数据
        let new_data = b"a"; // 要插入的单个字节
        let offset = 5; // 从偏移量 5 开始写入
        let overflow_length = BLOCK_SIZE; // 超出块大小的长度

        // 创建一个包含重复值的字节数组
        let data_to_write_at = vec![new_data[0]; overflow_length]; // 用 'a' 填充的数组

        let result = block.write_at(offset, &data_to_write_at);

        // 断言写入操作应该返回错误
        assert!(result.is_err(), "Expected an error when writing beyond block size");
    }

    #[test]
    fn test_block_append() {
        let mut block = Block::new();

        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);

        // 追加数据
        let append_data = b"Append";
        let new_block = block.append(append_data);

        // 读取数据
        let read_data = new_block.expect("REASON").read();

        // 将字节切片转换为字符串
        let read_data_str = std::str::from_utf8(read_data).unwrap();
        let append_data_str = std::str::from_utf8(append_data).unwrap();

        let valid_data_str = format!("{}{}",
            std::str::from_utf8(data_to_write).unwrap(),
            std::str::from_utf8(append_data).unwrap()
        );

        assert_eq!(read_data_str, valid_data_str);
        assert_eq!(block.is_dirty(), true);
    }

    #[test]
    fn test_truncate() {
        let mut block = Block::new();

        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);

        // 截断数据
        let new_block = block.truncate(5);

        // 读取数据
        let read_data = new_block.read();

        // 将字节切片转换为字符串
        let read_data_str = std::str::from_utf8(read_data).unwrap();

        assert_eq!(read_data_str, "Hello");
        assert_eq!(new_block.get_valid_length(), 5);
        assert_eq!(block.is_dirty(), true);
    }

    #[test]
    fn test_delete_block() {
        let mut block = Block::new();
        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);
        // 删除块
        block.delete();

        assert_eq!(block.is_deleted(), true);
    }

    #[test]
    fn test_valid_hash() {
        let mut block = Block::new();
        // 写入数据
        let data_to_write = b"Hello, world!";
        let _ = block.write(data_to_write);
        // 标记为已修改
        block.touch();
        // 计算哈希值
        let hash = calculate_hash(&block.data, block.timestamp.try_into().unwrap());
        assert_eq!(hash, block.hash);
    }
}