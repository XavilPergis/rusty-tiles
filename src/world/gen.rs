#![allow(dead_code)]

use world::block::*;
use world::chunk::*;
use render::texture::*;
use ::BlockTexture;

use std::collections::HashMap;

const COLOR_LIST: [[f32; 4]; 5] = [
    [0.0, 0.0, 0.0, 0.0],
    [0.1, 1.0, 0.0, 1.0],
    [0.4, 0.4, 0.4, 1.0],
    [0.8, 0.8, 0.8, 1.0],
    [0.8, 0.0, 0.5, 1.0]
];

pub trait WorldGenerator {
    // TODO: Add more functions, like get_temperature or something
    fn get_block(&self, bp: &BlockPos) -> Block;
    fn get_chunk(&self, cp: &ChunkPos) -> Chunk {
        let init_pos = cp.get_min_block_pos();
        let mut chunk = Chunk::new(cp);
        for xc in init_pos.0 .. init_pos.0 + CHUNK_SIZE_i64 {
            for yc in init_pos.1 .. init_pos.1 + CHUNK_SIZE_i64 {
                let pic = BlockPos(xc, yc).pos_in_chunk();
                let bpic = BlockPos(pic.0 as i64, pic.1 as i64);
                let bp = BlockPos(xc, yc);
                chunk.set_block(bpic, self.get_block(&bp));
            }
        }
        self.post_chunk_gen(&mut chunk);
        chunk
    }
    fn post_chunk_gen(&self, chunk: &mut Chunk) {}
}

pub struct FlatGenerator {
    pub thresh: i64
}

impl WorldGenerator for FlatGenerator {
    fn get_block(&self, bp: &BlockPos) -> Block {
        if bp.1 <= self.thresh {
            Block {
                // texture: self.texture_map.get("stone").unwrap(),
                material: Material::Stone,
                color: [0.4, 0.4, 0.4, 1.0],
                hardness: 1.0
            }
        } else {
            Block {
                // texture: self.texture_map.get("air").unwrap(),
                material: Material::Air,
                color: [0.0, 0.0, 0.0, 0.0],
                hardness: 1.0
            }
        }
    }
}

pub struct RandomGenerator;

impl WorldGenerator for RandomGenerator {
    fn get_block<'b>(&self, bp: &BlockPos) -> Block {
        use rand;
        use rand::*;

        let mut rng = rand::thread_rng();

        Block {
            // texture: self.texture_map.get("stone").unwrap(),
            material: MATERIAL_LIST[rng.gen::<usize>() % 5],
            color: COLOR_LIST[rng.gen::<usize>() % 5],
            hardness: 1.0
        }
    }
}

#[derive(Copy, Clone)]
struct BlockRegistry {
    block_list: HashMap<&str, Block>
}

#[derive(Copy, Clone)]
pub struct NoiseGenerator {
    pub scale: f64,
    pub stretch: f64,
    pub block_registry: BlockRegistry
}

impl WorldGenerator for NoiseGenerator {
    fn get_block(&self, bp: &BlockPos) -> Block {

        use noise::{perlin2, Seed};
        use noise::Brownian2;

        let b = Brownian2::new(perlin2, 4).wavelength(2.8);

        let v = b.apply(&Seed::new(0), &[bp.0 as f64 / self.scale, bp.1 as f64 / self.scale]);
        let c = v as f32;

        if 0.0 > c {
            let n = 1.0 - c.abs()/2.0;
            self.block_registry.get_block("water").unwrap()
        } else if 0.1 > c && c >= 0.0 {
            self.block_registry.get_block("sand").unwrap()
        } else {
            self.block_registry.get_block("grass").unwrap()
        }
    }
}
