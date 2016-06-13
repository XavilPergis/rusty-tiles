#![allow(dead_code, non_upper_case_globals)]

use render::texture::*;
use world::chunk::*;
use std::fmt::{ Debug, Formatter, Error };
use std::cell::RefCell;
use ::{ BlockTexture, App };

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockPos(pub i64, pub i64);

// fn get_texture<'a>(app: &'a App, name: &str) -> &'a BlockTexture {
//     match app.texture_atlas.get(name) {
//         Some(tex) => tex,
//         None => app.texture_atlas.get("missing").unwrap()
//     }
// }

#[derive(Debug, Copy, Clone)]
pub enum Blocks {
    Air,
    Stone
}

// impl Blocks {
//     fn texture(&self, app: &App) -> Option<BlockTexture> {
//         let cell = RefCell::new(get_texture(&app, "stone"));
//         match *self {
//             Blocks::Air => None,
//             Blocks::Stone => Some(cell.clone().)
//         }
//     }
// }

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Air,
    Earth,
    Stone,
    Metal,
    Gem,
    Wat
}

#[derive(Copy, Clone)]
pub struct Block {
    pub material: Material,
    // pub texture: &'a BlockTexture,
    pub color: [f32; 4],
    pub hardness: f64,
}

impl Block {
    pub fn new() -> Block {
        Block {
            material: Material::Air,
            color: [0.0, 0.0, 0.0, 1.0],
            hardness: 1.0
        }
    }
}

impl Debug for Block {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.debug_struct("Block")
            .field("material", &self.material)
            // .field("texture", &"<GFX Texture>")
            .field("color", &self.color)
            .field("hardness", &self.hardness)
            .finish()
    }
}

pub trait BlockEntity {

}

fn is_positive(n: i64) -> bool {
    n >= 0
}

impl BlockPos {
    #[inline(always)]
    pub fn pos_in_chunk(&self) -> (u64, u64) {

        let x = if is_positive(self.0) { self.0 } else { (self.0 + CHUNK_SIZE_i64) } % CHUNK_SIZE_i64;
        let y = if is_positive(self.1) { self.1 } else { (self.1 + CHUNK_SIZE_i64) } % CHUNK_SIZE_i64;

        ((x.abs() % CHUNK_SIZE_i64) as u64, (y.abs() % CHUNK_SIZE_i64) as u64)
    }
    #[inline(always)]
    pub fn containing_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new((self.0 as f64 / CHUNK_SIZE_i64 as f64).floor() as i64, (self.1 as f64 / CHUNK_SIZE_i64 as f64).floor() as i64)
    }
}
