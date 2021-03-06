use std::collections::{ HashMap, HashSet };
use world::block::*;
use world::chunk::*;

#[derive(Clone)]
pub struct World {
    pub loaded_chunks: HashMap<ChunkPos, Box<Chunk>>,
    pub queued_chunks: HashSet<ChunkPos>
}

impl World {
    pub fn new() -> World {
        World {
            loaded_chunks: HashMap::new(),
            queued_chunks: HashSet::new()
        }
    }
    pub fn chunk_exists(&self, pos: ChunkPos) -> bool {
        self.loaded_chunks.get(&pos).is_some()
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

    pub fn get_block(&self, pos: BlockPos) -> Option<(String, Block)> {
        let (cpos, ipos) = World::get_pos_pair(pos);
        if let Some(chunk) = self.get_chunk(cpos) {
            chunk.get_block_at_local(ipos.0, ipos.1)
        } else {
            None
        }
    }
}

// impl ToJson for World {
//     fn to_json(&self) -> Json {
//         /*
//         "loaded_chunks": {
//             "0,0":
//         }
//         */
//
//         // Json::String(format!("{}+{}i", self.a, self.b))
//     }
// }
