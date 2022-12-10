use crate::{
    direction::Direction,
    enemy::{self, Enemy},
    entity::Entity,
    projectile::Projectile,
    tile,
    world::World,
    WORLD_SIZE, BOARD_SIZE, TILE_SIZE,
    utils::Position
};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::VirtualKeyCode;
use ggez::graphics::{self, Canvas};

use std::{
    collections::HashMap,
};

// Can change easily
const MAX_PLAYER_HEALTH: usize = 10;
const PLAYER_MELEE_DAMAGE: usize = 1;
const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;
// TODO look over these values
const HEAL_ABILITY_RETURN: usize = 2;
const HEAL_KEYCODE: VirtualKeyCode = KeyCode::H;
const LIGHTNING_KEYCODE: VirtualKeyCode = KeyCode::L;
const BUILD_KEYCODE: VirtualKeyCode = KeyCode::B;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const PLAYER_PROJECTILE_DAMAGE: usize = 1;
const PLAYER_INITIAL_SPEED: usize = 1;
const PLAYER_INITIAL_ENERGY: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::GRASS];

// This is with the covered tile model, but we could use the static/dynamic board paradighm or
// something else entirely
pub struct Player {
    // This is the position in the form (x, y)
    pub pos: Position,

    // The direction that the player is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // This controls the number of tiles a player moves in a direction in a given keypress
    pub speed: usize,

    // This is the player color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the player is represented as a solid color
    color: [f32; 4],

    // Stores player health: for player death and such
    health: usize,

    // planned energy, used for healing, projectiles, (teleportation?), building
    energy: usize,
    //
}

impl Player {
    pub fn draw(&self, canvas: &mut graphics::Canvas, world: &World) {
        let color = [1.0, 1.0, 1.0, 1.0];
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new_i32(
                    (self.pos.x as i32 - world.x_offset as i32) * TILE_SIZE.0 as i32,
                    (self.pos.y as i32 - world.y_offset as i32) * TILE_SIZE.1 as i32,
                    TILE_SIZE.0 as i32,
                    TILE_SIZE.1 as i32
                ))
                .color(color),
        )
    }
    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        self.health -= damage
    }

    pub fn new(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) -> Self {
        let temp = Self {
            pos: Position::new(0, 0),
            direction: Direction::North,
            speed: PLAYER_INITIAL_SPEED,
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
            energy: PLAYER_INITIAL_ENERGY,
        };
        world[temp.pos.y][temp.pos.x] = temp.color;
        temp
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    pub fn use_input(key: KeyInput, world: &mut World) {
        match key.keycode {
            Some(key_pressed) => match key_pressed {
                KeyCode::Down => {
                    world.player.direction = Direction::South;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Up => {
                    world.player.direction = Direction::North;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Left => {
                    world.player.direction = Direction::West;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Right => {
                    world.player.direction = Direction::East;
                    World::travel(world, Entity::Player);
                }

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    Player::melee_attack(world);
                }
                PROJECTILE_ATTACK_KEYCODE => {
                    if (world.player.energy > 0) {
                        Player::projectile_attack(world);
                        // world.player.energy -= 1;
                        // commented out so I can test everything
                    }
                }
                HEAL_KEYCODE => {
                    if world.player.energy >= 5 {
                        world.player.health += HEAL_ABILITY_RETURN;
                        world.player.energy -= 5;
                    }
                }
                BUILD_KEYCODE => {
                    Player::build(world);
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn build(world: &mut World) {
        let position = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            1,
        );
        if world.world[position.x][position.x] == tile::STRUCTURE {
            world.world[position.y][position.x] = world.board[position.y][position.x]
        } else if position != world.player.pos {
            world.world[position.y][position.x] = tile::STRUCTURE;
        }
    }

    pub fn melee_attack(world: &mut World) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let attacking_position = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );

        // We do not know what enemies are on the tile being attacked, so we need to go through the
        // enemies and check if any of them are on the attacking tile, then damage them
        for enemy in &mut world.enemies {
            if enemy.pos == attacking_position {
                enemy.damage(PLAYER_MELEE_DAMAGE);
            }
        }
    }

    // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // and such should be determined by the projectile object itself
    pub fn projectile_attack(world: &mut World) {
        let projectile_spawn_pos = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );
        if projectile_spawn_pos != world.player.pos {
            let projectile = Projectile::new(
                projectile_spawn_pos.x,
                projectile_spawn_pos.y,
                PLAYER_PROJECTILE_SPEED,
                PLAYER_PROJECTILE_DAMAGE,
                world.player.direction.clone(),
                world,
            );
            world.projectiles.push(projectile);
        }
    }

    pub fn can_travel_to(
        tile: [f32; 4], 
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Option<Entity>)>
    ) -> bool {
        if entity_positions.contains_key(&position) {
            return false;
        }
        for permissible_tile in PERMISSIBLE_TILES {
            if tile == permissible_tile {
                return true;
            }

        }
        false
    }
}

