use world::block::*;
use world::chunk::*;

use std::collections::HashMap;

pub trait WorldGenerator {
    // TODO: Add more functions, like get_temperature or something
    fn get_block(&self, block_registry: &BlockRegistry, bp: &BlockPos) -> (String, Block);
    fn get_chunk(&self, block_registry: &BlockRegistry, cp: &ChunkPos) -> Chunk {
        let init_pos = cp.get_min_block_pos();
        let mut chunk = Chunk::new(cp);
        for xc in init_pos.0 .. init_pos.0 + CHUNK_SIZE_i64 {
            for yc in init_pos.1 .. init_pos.1 + CHUNK_SIZE_i64 {
                let pic = BlockPos(xc, yc).pos_in_chunk();
                let bpic = BlockPos(pic.0 as i64, pic.1 as i64);
                let bp = BlockPos(xc, yc);
                let (block_name, block) = self.get_block(block_registry, &bp);
                chunk.set_block(bpic, block_name, block);
            }
        }
        self.post_chunk_gen(&mut chunk);
        chunk
    }
    fn post_chunk_gen(&self, _: &mut Chunk) {}
}

// pub struct FlatGenerator {
//     pub thresh: i64
// }

// impl WorldGenerator for FlatGenerator {
//     fn get_block(&self, bp: &BlockPos) -> Block {
//         if bp.1 <= self.thresh {
//             Block {
//                 // texture: self.texture_map.get("stone").unwrap(),
//                 material: Material::Stone,
//                 color: [0.4, 0.4, 0.4, 1.0],
//                 hardness: 1.0
//             }
//         } else {
//             Block {
//                 // texture: self.texture_map.get("air").unwrap(),
//                 material: Material::Air,
//                 color: [0.0, 0.0, 0.0, 0.0],
//                 hardness: 1.0
//             }
//         }
//     }
// }

#[derive(Clone)]
pub struct BlockRegistry {
    block_map: HashMap<&'static str, Block>
}

impl BlockRegistry {
    pub fn new() -> BlockRegistry {
        BlockRegistry {
            block_map: HashMap::new()
        }
    }
    pub fn get_block(&self, name: &str) -> Option<Block> {
        match self.block_map.get(&name) {
            Some(&block) => Some(block),
            None => None
        }
    }
    pub fn register_block(&mut self, name: &'static str, block: Block) -> Result<(), bool> {
        if self.get_block(name).is_some() { Err(false) }
        else {
            self.block_map.insert(name, block);
            Ok(())
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseGenerator {
    pub scale: f64,
    pub stretch: f64,
    pub shift: f64
}

impl WorldGenerator for NoiseGenerator {
    fn get_block(&self, block_registry: &BlockRegistry, bp: &BlockPos) -> (String, Block) {

        use noise::{perlin2, Seed};
        use noise::Brownian2;

        let b = Brownian2::new(perlin2, 9).wavelength(2.2);

        let v = b.apply(&Seed::new(0), &[bp.0 as f64 / self.scale, bp.1 as f64 / self.scale]) + self.shift;
        let c = v as f32;

        if 0.0 > c {
            (String::from("water"), block_registry.get_block("water").unwrap())
        } else if 0.1 > c && c >= 0.0 {
            (String::from("sand"), block_registry.get_block("sand").unwrap())
        } else {
            (String::from("grass"), block_registry.get_block("grass").unwrap())
        }
    }
}
