// #![allow(unused_variables)]

// Crate Imports ----------------------------------------------
extern crate piston_window;
extern crate rand;
extern crate noise;
extern crate num;
extern crate threadpool;

// Use from standart library ----------------------------------
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
// use std::sync::mpsc::;

// Use from external crates -----------------------------------
use piston_window::*;
use num::NumCast;
use threadpool::ThreadPool;

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
    texture_atlas: HashMap<&'static str, BlockTexture>
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
            texture_atlas: HashMap::new()
        }
    }

    fn register_texture_from_path<'a, P: AsRef<Path>>(&mut self, name: &'static str, path: P) -> bool {
        let texture = Texture::from_path(
            &mut self.window.factory,
            path, // The path to the texture
            Flip::None,
            &TextureSettings::new()
        );
        match texture.ok() {
            Some(tex) => {
                self.texture_atlas.insert(name, tex);
                true
            }
            None => false
        }
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
    block_size: f64,
    world: World
}

fn main() {
    let mut app = App::new();

    // let texture_atlas: HashMap<&str, BlockTexture> = HashMap::new();

    app.register_texture_from_path("sand",  "textures/sand.png");
    app.register_texture_from_path("water", "textures/sand.png");
    app.register_texture_from_path("grass", "textures/sand.png");
    app.register_texture_from_path("sand",  "textures/sand.png");
    app.register_texture_from_path("stone", "textures/sand.png");

    static PLAYER_ACCEL_RATE: f64 = 0.021;
    static PLAYER_DECEL_RATE: f64 = 1.09;
    static PLAYER_MAX_VELOCITY: f64 = 2.0;
    static PLAYER_VIEW_DISTANCE: i64 = 4;
    static NUM_THREADS: usize = 16;

    // let wg = RandomGenerator {};
    // let wg = NoiseGenerator {
    //     scale: 40.0,
    //     stretch: 2.0
    // };
    // let wg = FlatGenerator {
    //     thresh: 5
    // };

    let mut world: World = World::new();

    let mut player = Player {
        pos: (0.0, 0.0),
        vel: (0.0, 0.0),
        block_size: 40.0,
        world: World::new()
    };

    let pool = ThreadPool::new(NUM_THREADS);
    let (tx, rx) = mpsc::channel::<(Chunk, ChunkPos)>();

    let mut is = input::InputState::new();

    is.win_size = app.size_as_tuple();

    let wg = NoiseGenerator {
        scale: 40.0,
        stretch: 2.0
    };

    while let Some(event) = app.window.next() {
        match event {
            Event::Render(ra) => {
                rendering::render_all(&mut app.window, &mut player.clone(), &world, &event, &is);
            },
            Event::AfterRender(ara) => {},
            Event::Update(ua) => {

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
                            pool.execute(move || {
                                match txc.send((wgc.get_chunk(&chunk_pos), chunk_pos)).ok() {
                                    Some(v) => {
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
            Event::Idle(ia) => {},
            Event::Input(ipt) => {
                input::handle_input(ipt, &mut is);
            }
        }
    }
}
