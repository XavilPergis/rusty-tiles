#![allow(dead_code, non_upper_case_globals)]

use std::collections::HashMap;
use world::block::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_i64: i64 = CHUNK_SIZE as i64;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChunkPos {
    pub x: i64,
    pub y: i64
}

impl ChunkPos {
    pub fn new(x: i64, y: i64) -> ChunkPos {
        ChunkPos {
            x: x,
            y: y
        }
    }
    pub fn get_min_block_pos(&self) -> BlockPos {
        BlockPos(self.x * CHUNK_SIZE_i64, self.y * CHUNK_SIZE_i64)
    }
}

#[derive(Clone, Debug)]
struct ChunkData {
    data: HashMap<BlockPos, Block>,
    pos: ChunkPos,
    initialized: bool
}

impl ChunkData {
    fn new(x: i64, y: i64) -> ChunkData {
        ChunkData {
            data: HashMap::new(),
            pos: ChunkPos::new(x, y),
            initialized: false
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn is_full(&self) -> bool {
        self.data.len() == CHUNK_SIZE * CHUNK_SIZE
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    map: ChunkData
}

#[derive(Copy, Clone, Debug)]
pub struct ChunkRequest {
    pos: ChunkPos
}

impl ChunkRequest {
    pub fn new(pos: ChunkPos) -> ChunkRequest { ChunkRequest { pos: pos } }
}

impl Chunk {
    pub fn new(pos: &ChunkPos) -> Chunk {
        Chunk {
            map: ChunkData::new(pos.x, pos.y)
        }
    }

    pub fn set_block(&mut self, pos: BlockPos, block: Block) {
        self.map.data.insert(pos, block);
    }

    pub fn get_block_at_local(&self, x: u64, y: u64) -> Option<Block> {
        match self.map.data.get(&BlockPos(x as i64, y as i64)) {
            Some(blk) => Some(blk.clone()),
            None => None
        }
    }
}
