// #![allow(dead_code, non_upper_case_globals)]

use world::chunk::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BlockPos(pub i64, pub i64);

// fn get_texture<'a>(app: &'a App, name: &str) -> &'a BlockTexture {
//     match app.texture_atlas.get(name) {
//         Some(tex) => tex,
//         None => app.texture_atlas.get("missing").unwrap()
//     }
// }

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[derive(RustcDecodable, RustcEncodable)]
pub enum Material {
    Air,
    Earth,
    Stone,
    Metal,
    Gem,
    Wat
}

#[derive(Debug, Copy, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Block {
    pub material: Material,
    pub color: [f32; 4],
    pub hardness: f64,
}

impl Block {
    #[allow(dead_code)]
    pub fn new() -> Block {
        Block {
            material: Material::Air,
            color: [0.0, 0.0, 0.0, 1.0],
            hardness: 1.0
        }
    }
}

pub trait BlockEntity {

}

#[inline(always)]
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
