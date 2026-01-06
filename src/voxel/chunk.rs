use bevy::math::Vec3;
use serde::{Deserialize, Serialize};
use noise::{Perlin, NoiseFn};


pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub cx: i32,
    pub cz: i32,
    pub blocks: Vec<u8>,
}


impl Chunk {
    pub fn new(cx: i32, cz: i32) -> Self {
        Self {
            cx,
            cz,
            blocks: vec![0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE],
        }
    }

    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + CHUNK_SIZE * (z + CHUNK_SIZE * y)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, id: u8) {
        let index = Self::index(x, y, z);
        self.blocks[index] = id;
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = Self::index(x, y, z);
        self.blocks[index]
    }


    pub fn fill_test_terrain(&mut self) {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if y < 10 {
                        self.set_block(x, y, z, 1); // Erde
                    }
                }
            }
        }
    }

    pub fn generate_chunk(&mut self) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = self.cx * CHUNK_SIZE as i32 + x as i32;
                let world_z = self.cz * CHUNK_SIZE as i32 + z as i32;
                let h = generate_height(world_x as f64, world_z as f64);
                for y in 0..CHUNK_HEIGHT {
                    let idx = Chunk::index(x, y, z);
                    self.blocks[idx] = if (y as i32) < h { 1 } else { 0 };
                }
            }
        }
    }

    
}

fn generate_height(x: f64, z: f64) -> i32 {
    let perlin = Perlin::default();
    let n = perlin.get([x * 0.01, z * 0.01]);
    (n * 20.0 + 64.0) as i32
}

pub fn get_chunks_playerposition(position: Vec3,range:ChunkRange)-> Vec<(i32,i32)>{
    let playerposx :i32 = position.x as i32;
    let playerposz :i32 = position.z as i32;

    let chunkx = playerposx / CHUNK_SIZE as i32;
    let chunkz = playerposz / CHUNK_SIZE as i32;
    let range_iter = range.value();
    let range_iter_i32 = range_iter as i32;

    let mut returnvector:Vec<(i32,i32)> = Vec::new();
    for xcordinates in -range_iter_i32..=range_iter_i32 {
        for zcordinates in -range_iter_i32..=range_iter_i32 {
            returnvector.push((chunkx+xcordinates,chunkz+zcordinates))

        }
    }
    returnvector


}

#[derive(Debug,Copy,Clone)]
pub struct ChunkRange(u8);
impl ChunkRange {
    pub fn new(range:u8) -> Option<Self> {
        if range <= 9 {
            Some(ChunkRange(range))
        } else {
            None
        }
    }

    fn value(self) -> u8 {
        self.0
    }
}

