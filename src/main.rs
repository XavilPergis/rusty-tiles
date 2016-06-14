// Crate Imports ----------------------------------------------
extern crate piston_window;
extern crate rand;
extern crate noise;
extern crate num;
extern crate threadpool;
extern crate serde;
extern crate serde_json;

// Use from standart library ----------------------------------
use std::collections::*;
use std::path::Path;
use std::sync::mpsc;

// Use from external crates -----------------------------------
use piston_window::*;
use num::NumCast;
use threadpool::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;

// Local module definitions -----------------------------------
mod world;
mod render;
mod tests;
mod input;
mod util;

// Local use statements ---------------------------------------
use world::gen::*;
use world::world::*;
use world::block::*;
use world::chunk::*;
use render::render as rendering;

// Typedefs ---------------------------------------------------
pub type BlockTexture = G2dTexture<'static>;

pub struct App {
    window: PistonWindow,
    texture_atlas: HashMap<&'static str, BlockTexture>,
    block_registry: BlockRegistry
}

impl App {
    fn new() -> App {
        let size = Size {
            width: 1000,
            height: 500
        };
        App {
            window: {
                WindowSettings::new("Foo", (size.width, size.height))
                .exit_on_esc(true)
                .build()
                .unwrap_or_else(|e| panic!("Error building PistonWindow: {}", e))
            },
            texture_atlas: HashMap::new(),
            block_registry: BlockRegistry::new()
        }
    }

    fn register_texture_from_path<'a, P: AsRef<Path>>(&mut self, name: &'static str, path: P) -> Result<(), bool> {
        let texture = Texture::from_path(
            &mut self.window.factory,
            path, // The path to the texture
            Flip::None,
            &TextureSettings::new()
        );
        match texture.ok() {
            Some(tex) => {
                self.texture_atlas.insert(name, tex);
                Ok(())
            }
            None => Err(false)
        }
    }

    fn register_block(&mut self, name: &'static str) -> Result<(), bool> {
        let block = Block::new();
        self.block_registry.register_block(name, block)
    }

    fn size_as_tuple<N: NumCast>(&self) -> (N, N) {
        let win_size = self.window.window.size();
        (NumCast::from(win_size.width).unwrap(), NumCast::from(win_size.height).unwrap())
    }
}

#[derive(Clone)]
pub struct Player {
    pos: (f64, f64),
    vel: (f64, f64),
    block_size: f64
}

fn main() {
    let mut app = App::new();

    app.register_texture_from_path("water", "textures/water.png").unwrap();
    app.register_texture_from_path("sand",  "textures/sand.png").unwrap();
    app.register_texture_from_path("grass", "textures/grass.png").unwrap();
    app.register_texture_from_path("stone", "textures/stone.png").unwrap();

    app.register_block("water").unwrap();
    app.register_block("sand").unwrap();
    app.register_block("grass").unwrap();

    static NUM_THREADS:          usize = 2;
    static PLAYER_ACCEL_RATE:      f64 = 0.021;
    static PLAYER_DECEL_RATE:      f64 = 1.09;
    static PLAYER_MAX_VELOCITY:    f64 = 2.0;
    static PLAYER_VIEW_DISTANCE:   i64 = 4;

    let mut world: World = World::new();

    let mut player = Player {
        pos: (0.0, 0.0),
        vel: (0.0, 0.0),
        block_size: 40.0
    };

    let pool = ThreadPool::new(NUM_THREADS);
    let (tx, rx) = mpsc::channel::<(Chunk, ChunkPos)>();

    let mut is = input::InputState::new();

    is.win_size = app.size_as_tuple();

    let mut json = String::new();

    let mut file = File::open("gen.json").unwrap();
    file.read_to_string(&mut json).unwrap();

    let gen_map: BTreeMap<String, Value> = serde_json::from_str(&json).unwrap();
/*
pub struct NoiseGenerator {
    pub scale: f64,
    pub stretch: f64,
    pub shift: f64,
    pub wavelength: f64,
    pub octaves: usize,
    pub seed: usize,
    pub water_thresh: f64,
    pub sand_thresh: f64,
    pub grass_thresh: f64
}
 */
    // scale, stretch, shift, wavelength, octaves, func
    let wg = NoiseGenerator {
        scale:        gen_map.get(&"scale".to_string())     .unwrap().as_f64().unwrap(), // 30.0
        stretch:      gen_map.get(&"stretch".to_string())   .unwrap().as_f64().unwrap(), // 1.8
        shift:        gen_map.get(&"shift".to_string())     .unwrap().as_f64().unwrap(), // 0.3
        wavelength:   gen_map.get(&"wavelength".to_string()).unwrap().as_f64().unwrap(), // 2.7
        octaves:      gen_map.get(&"octaves".to_string())   .unwrap().as_u64().unwrap() as usize,
        seed:         gen_map.get(&"seed".to_string())      .unwrap().as_u64().unwrap() as u32,
        water_thresh: gen_map.get(&"wthresh".to_string())   .unwrap().as_f64().unwrap(),
        sand_thresh:  gen_map.get(&"sthresh".to_string())   .unwrap().as_f64().unwrap(),
        grass_thresh: gen_map.get(&"gthresh".to_string())   .unwrap().as_f64().unwrap(),
        algo:         gen_map.get(&"algo".to_string())      .unwrap().as_string().unwrap().to_string()
    };

    while let Some(event) = app.window.next() {
        match event {
            Event::Render(_ra) => {
                rendering::render_all(&mut app.window, &mut app.texture_atlas, &mut player.clone(), &world, &event, &is);
            },
            Event::AfterRender(_ara) => {},
            Event::Update(_ua) => {

                let current_block = BlockPos(player.pos.0.floor() as i64, player.pos.1.floor() as i64);
                let current_chunk = current_block.containing_chunk_pos();

                player.block_size += is.scroll_dir;
                is.scroll_dir = 0.0;

                player.pos.0 += player.vel.0;
                player.pos.1 += player.vel.1;

                player.vel.1 /= PLAYER_DECEL_RATE;
                player.vel.0 /= PLAYER_DECEL_RATE;

                if is.going_up    && PLAYER_MAX_VELOCITY > player.vel.1.abs() { player.vel.1 -= PLAYER_ACCEL_RATE; }
                if is.going_down  && PLAYER_MAX_VELOCITY > player.vel.1.abs() { player.vel.1 += PLAYER_ACCEL_RATE; }
                if is.going_left  && PLAYER_MAX_VELOCITY > player.vel.0.abs() { player.vel.0 -= PLAYER_ACCEL_RATE; }
                if is.going_right && PLAYER_MAX_VELOCITY > player.vel.0.abs() { player.vel.0 += PLAYER_ACCEL_RATE; }

                for xc in current_chunk.x - PLAYER_VIEW_DISTANCE .. current_chunk.x + PLAYER_VIEW_DISTANCE {
                    for yc in current_chunk.y - PLAYER_VIEW_DISTANCE .. current_chunk.y + PLAYER_VIEW_DISTANCE {
                        let chunk_pos = ChunkPos::new(xc, yc);
                        if !world.chunk_exists(chunk_pos) && !world.is_queued(chunk_pos) {
                            world.set_queued(chunk_pos);
                            let wgc = wg.clone();
                            let txc = tx.clone();
                            let tbr = app.block_registry.clone();
                            pool.execute(move || {
                                match txc.send((wgc.get_chunk(&tbr, &chunk_pos), chunk_pos)).ok() {
                                    Some(_v) => {
                                        println!("Generated chunk ({}, {})", chunk_pos.x, chunk_pos.y);
                                    },
                                    None => { /* Something terrible happened! Ignore it for now. */ }
                                }
                            });
                        }
                    }
                }

                // TODO: Figure out why this works.
                for (chunk, chunk_pos) in rx.try_recv() {
                    world.set_chunk(chunk, chunk_pos);
                }

            },
            Event::Idle(_ia) => {},
            Event::Input(ipt) => {
                input::handle_input(ipt, &mut is);
            }
        }
    }
}
