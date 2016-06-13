use piston_window::*;

use ::Player;
use world::block::*;
use world::world::*;
use input::*;

#[inline]
fn world_to_screen(x: i64, y: i64, px: f64, py: f64) -> (f64, f64) {
    (x as f64 - px, y as f64 - py)
}

pub fn render_all(win: &mut PistonWindow, player: &mut Player, world: &World, event: &Event, is: &InputState) {

    let topleft = &player.pos;
    let block_size = player.block_size;

    // Starting coords (world space). These are passed into the function
    let x0 = topleft.0.floor() as i64;
    let y0 = topleft.1.floor() as i64;

    // Calculate how many blocks we can see
    let nx = (is.win_size.0 / (block_size.floor().abs() as u32)) as i64; // How many columns the player can see
    let ny = (is.win_size.1 / (block_size.floor().abs() as u32)) as i64; // How many rows the player can see

    // Offset the end coords by the starting coords and add in the fov dimensions
    // I have no idea why three needs to be added on, but without it, there is an odd black border
    let x1 = x0 + nx + 3;
    let y1 = y0 + ny + 3;

    win.draw_2d(event, |ctx, gl| {

        // Clear the screen. Uh... Yeah.
        clear([0.0, 0.0, 0.0, 1.0], gl);

        // Iterate over all blocks in current fov
        for xc in x0 .. x1 - 1 {
            for yc in y0 .. y1 - 1 {
                // Only draw the block if it actually exists (which is hopefully 100% of the time)
                if let Some(block) = world.get_block(BlockPos(xc, yc)) {
                    // Dransfrorm world space to pixes space
                    let dp = world_to_screen(xc, yc, topleft.0, topleft.1);

                    // Translate...
                    let translated = ctx.transform.trans(dp.0 * block_size, dp.1 * block_size);

                    // Draw a square for each tile
                    let square = rectangle::square(0.0, 0.0, block_size);

                    // TODO: Don't get the block's color
                    // Flush the square to the screen
                    rectangle(block.color, square, translated, gl);
                }
            }
        }
    });
}
