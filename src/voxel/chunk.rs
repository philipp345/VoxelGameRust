use serde::{Deserialize, Serialize};



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



    
}

