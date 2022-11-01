use crate::direction::Direction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::{SCREEN_SIZE, TILE_SIZE, WORLD_SIZE};
use ggez::{
    event,
    graphics::{self, Canvas},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

pub struct State {
    // Time delta, unused for now I think
    delta: u128,
    // RGBA values
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    // Current tile, in order to iterate over the thing
    tile: i16,

    // world to store the state of tiles in between frames
    world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],

    // store an instance of a player
    player: Player,

    // list of enemies in our world
    enemies: Vec<Enemy>,
}

impl State {
    // just returns the default values
    pub fn new() -> Self {
        let mut world = [[[0.; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]; 
        Self {
            delta: 0,
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
            tile: 0,
            world,
            player: Player::new(&mut world),
            enemies: vec![Enemy::new(&mut world, 10, 10)]
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for index in (0..self.enemies.len()).rev() {
            self.enemies[index].update(&mut self.enemies);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // render the graphics with the rgb value, it will always be black though because the
        // self.r,g,b,a values never change from 0
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([self.r, self.g, self.b, self.a]),
        );


        // draw our state matrix "world" to the screen
        // We must partition our window into small sectors of 32 by 32 pixels and then for each
        // individual one, change each of the pixels to the color corresponding to its place in the
        // state matrix
        for i in 0..WORLD_SIZE.1 {
            for j in 0..WORLD_SIZE.0 {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            j as i32 * TILE_SIZE.0 as i32,
                            i as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(self.world[i as usize][j as usize]),
                );
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        // ALT, SUPER KEY RESULTS IN A "NONE" VALUE CRASHING THIS STUFF
        self.player.use_input(input, &mut self.world, &mut self.enemies);
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }
}
