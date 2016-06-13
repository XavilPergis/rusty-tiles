#![allow(dead_code, non_upper_case_globals)]

use std::collections::{ HashMap, HashSet };
use world::gen::*;
use world::block::*;
use world::chunk::*;
use util::queue::Queue;

mod consts {
    pub const CHUNK_SIZE: usize = 16;
    pub const BLOCK_SIZE: f64 = 1.0;
}

#[derive(Clone)]
pub struct World {
    pub gen_queue: Queue<ChunkRequest>,
    pub loaded_chunks: HashMap<ChunkPos, Box<Chunk>>,
    pub queued_chunks: HashSet<ChunkPos>
}

impl World {
    pub fn new() -> World {
        World {
            gen_queue: Queue::new(),
            loaded_chunks: HashMap::new(),
            queued_chunks: HashSet::new()
        }
    }
    pub fn chunk_exists(&self, pos: ChunkPos) -> bool {
        self.loaded_chunks.get(&pos).is_some()
    }
    pub fn gen_chunk<G: WorldGenerator>(&mut self, world_gen: &G, pos: ChunkPos) {
        self.queued_chunks.remove(&pos);
        if let None = self.loaded_chunks.get(&pos) {
            self.loaded_chunks.insert(pos, Box::new(world_gen.get_chunk(&pos)));
        }
    }

    // FIXME: Queueing seems like a corner case... Is there a better way to do this?
    pub fn is_queued(&self, pos: ChunkPos) -> bool {
        self.queued_chunks.contains(&pos)
    }
    pub fn set_queued(&mut self, pos: ChunkPos) {
        self.queued_chunks.insert(pos);
    }
    pub fn set_chunk(&mut self, chunk: Chunk, pos: ChunkPos) {
        self.queued_chunks.remove(&pos);
        self.loaded_chunks.insert(pos, Box::new(chunk));
    }
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        if let Some(c) = self.loaded_chunks.get(&pos) {
            Some(c)
        } else {
            // self.gen_queue.enqueue(ChunkRequest::new(pos));
            None
        }
    }

    pub fn get_pos_pair(pos: BlockPos) -> (ChunkPos, (u64, u64)) {
        (pos.containing_chunk_pos(), pos.pos_in_chunk())
    }

    pub fn get_block(&self, pos: BlockPos) -> Option<Block> {
        let (cpos, ipos) = World::get_pos_pair(pos);
        if let Some(chunk) = self.get_chunk(cpos) {
            chunk.get_block_at_local(ipos.0, ipos.1)
        } else {
            None
        }
    }
}
