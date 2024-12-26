use crate::abstracts::*;
use crate::{calculate_hash, current_timestamp_nanos, BLOCK_SIZE};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Superblock {
    total_blocks: usize,                          // 总块数
    free_blocks: Vec<Box<Block>>,                 // 空闲块
    allocated_blocks: HashMap<usize, Box<Block>>, // 已分配的块
    block_map: HashMap<usize, Box<Block>>,        // 块映射
}

struct IdGenerator {
    last_timestamp: Mutex<u64>,
    sequence: Mutex<u64>,
}

impl IdGenerator {
    fn allocate_id() -> u64 {
        let id_gen = IdGenerator {
            last_timestamp: Mutex::new(0),
            sequence: Mutex::new(0),
        };
        let mut last_timestamp = id_gen.last_timestamp.lock().unwrap();
        let mut sequence = id_gen.sequence.lock().unwrap();

        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        if *last_timestamp == timestamp {
            *sequence += 1;
            if *sequence >= 1_000 {
                // 等待下一个毫秒
                loop {
                    timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis() as u64;

                    if *last_timestamp < timestamp {
                        break;
                    }
                }
            }
        } else {
            *sequence = 0;
        }

        *last_timestamp = timestamp;

        (timestamp << 10) | *sequence
    }
}

pub struct Block {
    id: u64,
    index: usize,             // 块索引,用于标识块的位置，方便进行块的查找和管理
    size: usize,              // 块大小,用于限制块的大小，避免内存溢出
    offset: usize,            // 块偏移量,用于指示块在数据存储中的位置，帮助快速定位数据块
    data: [u8; BLOCK_SIZE],   // 数据,用于存储实际的数据
    valid_length: usize, // 有效数据长度,用于指示块内有效数据的长度，确保只处理有效数据，避免处理无效数据
    is_dirty: bool,      // 是否被修改,用于标记块是否被修改，以便在需要时进行持久化存储
    is_used: bool,       // 是否被使用,用于标记块是否被使用，以便进行块的分配和回收
    is_deleted: bool,    // 是否被删除
    timestamp: u128,     // 时间戳
    last_modify_time: u128, // 上次修改时间
    next: Option<Box<Block>>, // 下一个块,用于双向链表
    prev: Option<Box<Block>>, // 上一个块,用于双向链表
    hash: String,        // 哈希值,用于标识块的唯一性
}

impl Block {
    pub(crate) fn new() -> Block {
        Block {
            id: IdGenerator::allocate_id(),
            index: 0,
            size: 0,
            offset: 0,
            data: [0; BLOCK_SIZE],
            valid_length: 0,
            is_dirty: false,
            is_deleted: false,
            is_used: false,
            timestamp: 0,
            last_modify_time: 0,
            next: None,
            prev: None,
            hash: String::new(),
        }
    }

    fn truncate(&mut self, size: usize) {
        if size < self.valid_length {
            self.valid_length = size;
            self.data[size..].fill(0);
        }
    }

    fn get_valid_length(&self) -> usize {
        self.valid_length
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn is_used(&self) -> bool {
        self.is_used
    }

    fn do_touch(&mut self) {
        // 更新时间戳
        self.last_modify_time = current_timestamp_nanos();
        // 标记为脏
        self.is_dirty = true;
        // 更新哈希值
        self.hash = calculate_hash(&self.data, self.timestamp.try_into().unwrap());
    }

    fn do_flush(&mut self) {
        // 写入数据到磁盘
        self.is_dirty = false;
        self.is_used = true;
        self.last_modify_time = current_timestamp_nanos();
    }

    fn release_block(&mut self) {
        // 释放块
        self.data.fill(0);
        self.is_deleted = true;
        self.is_used = false;
        self.do_touch();
    }
}

impl BlockReadWrite for Block {
    fn read_block(&mut self, index: usize) -> &[u8] {
        if index > self.valid_length {
            panic!("index out of range");
        } else {
            &self.data[index..self.valid_length]
        }
    }
    fn write_block(&mut self, index: usize, data: &[u8]) {
        let len = data.len();

        // 检查是否有足够的空间来写入数据
        if len > BLOCK_SIZE && (index + len) > BLOCK_SIZE {
            panic!("data is too large");
        }

        // 将提供的数据复制到块的 data 数组中
        self.data[index..len].copy_from_slice(data);
        self.valid_length = len;
        self.timestamp = current_timestamp_nanos();
        self.is_used = true;
        self.truncate(len);
        self.do_touch();
    }
}

impl BlockIterator<Block> for Block {
    fn next(&mut self) -> Option<&Block> {
        self.next.as_ref().map(|b| b.as_ref())
    }
}

impl BlockAllocator<Block> for Superblock {
    fn allocate(&mut self, size: usize) -> Vec<Box<Block>> {
        if self.free_blocks.len() >= size {
            let mut allocated_blocks = Vec::new();
            for _ in 0..size {
                let block = self.free_blocks.pop().unwrap();
                allocated_blocks.push(block);
            }

            allocated_blocks
        } else {
            panic!("Not enough free blocks");
        }
    }

    fn deallocate(&mut self, index: usize) {
        if let Some(block) = self.allocated_blocks.remove(&index) {
            self.free_blocks.push(block);
        }
    }

    fn is_allocated(&self, index: usize) -> bool {
        self.allocated_blocks.contains_key(&index)
    }
}
