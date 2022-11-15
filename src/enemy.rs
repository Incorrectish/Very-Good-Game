use crate::{direction::Direction, WORLD_SIZE, world::World, tile, movable::Movable};

const ENEMY_HEALTH: usize = 5;

// This is basically the same as the enemy for now, but I am just testing an enemy system
pub struct Enemy {
    // This is the position in the form (x, y)
    pub pos: (usize, usize),
    
    // The direction that the enemy is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    direction: Direction,
    
    // This simply stores the color of the tile that the enemy is currently on, so that when they
    // move off of it, it can be rendered properly back to what it was 
    covered_tile: [f32; 4],

    // This is the enemy color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the enemy is represented as a solid color
    color: [f32; 4],

    // Stores enemy health: for enemy death and such
    pub health: usize,
}

impl Enemy {
    pub fn new(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize], x: usize, y: usize) -> Self {
        let temp = Self {
            pos: (x, y),
            direction: Direction::North,
            covered_tile: world[y][x],
            color: tile::ENEMY,
            health: ENEMY_HEALTH,
        };
        world[y][x] = temp.color;
        temp
    }

    // TODO: rewrite to make the travel function the same as player travel
    pub fn travel(
        &mut self,
        world: &mut World,
    ) {
        world.world[self.pos.1][self.pos.0] = self.covered_tile;
        match self.direction {
            Direction::North => {
                if self.pos.1 > 0 {
                    self.pos.1 -= 1
                }
            }
            Direction::South => {
                if self.pos.1 < (WORLD_SIZE.1 - 1) as usize {
                    self.pos.1 += 1
                }
            }
            Direction::East => {
                if self.pos.0 < (WORLD_SIZE.0 - 1) as usize {
                    self.pos.0 += 1
                }
            }
            Direction::West => {
                if self.pos.0 > 0 {
                    self.pos.0 -= 1
                }
            }
        }
        self.covered_tile = world.world[self.pos.1][self.pos.0];
        world.world[self.pos.1][self.pos.0] = self.color;
    }

    pub fn update(world: &mut World) {
        // thinking of using a hack to remove all the enemies at the position instead because two
        // enemies cannot be on the same tile, would avoid the f32 lack of equality
        for index in (0..world.enemies.len()).rev() {
            if world.enemies[index].health <= 0 {
                Enemy::kill(world, index);
                world.enemies.remove(index);
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        world.world[world.enemies[index].pos.1][world.enemies[index].pos.0] = world.enemies[index].covered_tile;
    }
}

impl Movable for Enemy {
    fn set_pos(&mut self, new_pos: (usize, usize)) {
        todo!()
    }

    fn get_pos(&self) -> (usize, usize) {
        todo!()
    }

    fn get_x(&self) -> usize {
        todo!()
    }

    fn get_y(&self) -> usize {
        todo!()
    }

    fn get_covered_tile(&self) -> [f32; 4] {
        todo!()
    }

    fn set_covered_tile(&mut self, new_tile: [f32; 4]) {
        todo!()
    }

    fn get_color(&self) -> [f32; 4] {
        todo!()
    }

    fn get_direction(&self) -> Direction {
        todo!()
    }
}
