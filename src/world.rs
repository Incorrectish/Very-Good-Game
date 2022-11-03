use crate::{
    WORLD_SIZE,
    player::Player,
    enemy::Enemy,
    projectile::Projectile,
    tile,
    random
};
use rand::rngs::ThreadRng;

pub struct World {
    // world to store the state of tiles in between frames
    pub world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],

    // store an instance of a player
    pub player: Player,

    // list of enemies in our world
    pub enemies: Vec<Enemy>,

    // list of all the projectiles in the world
    pub projectiles: Vec<Projectile>,
}

impl World {
    // generates the center boss room for map
    pub fn gen_boss(world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        let x: usize = (WORLD_SIZE.0 as usize) / 2 - 1;
        let y: usize = (WORLD_SIZE.1 as usize) / 2 - 1;
        println!("{}", x);
        println!("{}", y);
        for i in 0..8 {
            for j in 0..8 {
                world[x-3+i][y-3+j] = tile::WALL;
            }
        }
        world[x][y] = tile::PORTAL;
        world[x+1][y] = tile::PORTAL;
        world[x][y+1] = tile::PORTAL;
        world[x+1][y+1] = tile::PORTAL;
    }

    pub fn gen_water(rng: &mut ThreadRng, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        let mut lakes_added = 0;
        while lakes_added < 5 {
            let x = random::rand_range(rng, 5, WORLD_SIZE.0);
            let y = random::rand_range(rng, 5, WORLD_SIZE.1);

            Self::gen_water_helper(rng, x, y, 0, world);
            lakes_added += 1;
        }
    }

    pub fn gen_water_helper(rng: &mut ThreadRng, x: i16, y: i16, dist: i16, world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]) {
        if world[x as usize][y as usize] == tile::FLOOR {
            world[x as usize][y as usize] = tile::WATER;
        }

        if random::bernoulli(rng, 1.-0.1*(dist as f32)) {
            Self::gen_water_helper(rng, x+1, y, dist+1, world);
        }
        if random::bernoulli(rng, 1.-0.1*(dist as f32)) {
            Self::gen_water_helper(rng, x-1, y, dist+1, world);
        }
        if random::bernoulli(rng, 1.-0.1*(dist as f32)) {
            Self::gen_water_helper(rng, x, y+1, dist+1, world);
        }
        if random::bernoulli(rng, 1.-0.1*(dist as f32)) {
            Self::gen_water_helper(rng, x, y-1, dist+1, world);
        }
    }
}
