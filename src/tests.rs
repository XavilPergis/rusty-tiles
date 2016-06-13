#![allow(unused_imports)]

use std::collections::HashMap;

use world::gen::*;
use world::world::*;
use world::chunk::*;
use world::block::*;

#[test]
fn length_of_chunk_map() {
    let wg = RandomGenerator {};

    let mut world = World {
        gen: wg,
        loaded_chunks: HashMap::new()
    };

    let glim = 5;

    for xp in -glim .. glim {
        for yp in -glim .. glim {
            world.gen_chunk(ChunkPos(xp, yp));
        }
    }

    if 4 * glim * glim != world.loaded_chunks.len() as i64 {
        for key in world.loaded_chunks.keys() {
            println!("{:?}", key);
        }
        println!("{:?} {:?}", 4 * glim * glim, world.loaded_chunks.len());
        panic!();
    }
}

#[test]
fn get_block_in_bounds() {
    let wg = RandomGenerator {};

    let mut world = World {
        gen: wg,
        loaded_chunks: HashMap::new()
    };

    let glim = 5;

    for xp in -glim .. glim {
        for yp in -glim .. glim {
            world.gen_chunk(ChunkPos(xp, yp));
        }
    }

    if let Some(n) = world.get_block(BlockPos(-26, -8)) {

    } else {
        panic!();
    }
}
