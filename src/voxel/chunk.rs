use bevy::math::Vec3;
use serde::{Deserialize, Serialize};
use noise::{Perlin, NoiseFn};
use bevy::prelude::*;
use std::collections::HashMap;
use crate::{Player, PlayerPositions};
use std::collections::HashSet;
use std::sync::OnceLock;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const DEFAULT_CHUNK_RANGE: u8 = 5;
//Static variable EDGE_INDICES will be filled by edge_indices_value()
static EDGE_INDICES: OnceLock<HashSet<usize>> = OnceLock::new();


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub cx: i32,
    pub cz: i32,
    pub blocks: Vec<u8>,
    pub is_dirty: bool,
}


impl Chunk {
    pub fn new(cx: i32, cz: i32) -> Self {
        Self {
            cx,
            cz,
            blocks: vec![0; CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE],
            is_dirty: true,
        }
    }

    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + CHUNK_SIZE * (z + CHUNK_SIZE * y)
    }

    pub fn get_coords(index: usize) -> (usize, usize, usize) {
        let s = CHUNK_SIZE;

        let x = index % s;
        let z = (index / s) % s;
        let y = index / (s * s);

        (x, y, z)
    }

    //set_block will be used for generating chunks
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, id: u8) {
        let index = Self::index(x, y, z);
        self.blocks[index] = id;
            }
    //change_block is functional similar to set_block but will be used when player changes a block
    pub fn change_block(&mut self, x: usize, y: usize, z: usize, id: u8, mut storage:ResMut<ChunkStorage>) {
        //Each time a block in the chunk is changed set the is_dirty flag in order that mesh of chunk gets newly calculated.
        let index = Self::index(x, y, z);
        self.blocks[index] = id;
        self.is_dirty = true;
        //Check if index is an edge index, if yes, mark neighbour chunk as dirty as well
        if edge_indices_value().contains(&index)  {
            if x == 0 {
            if let Some(chunk) = storage.chunks.get_mut(&(self.cx - 1, self.cz)) {
                chunk.is_dirty = true;
            }
            }
            if x == CHUNK_SIZE-1 {
                if let Some(chunk) = storage.chunks.get_mut(&(self.cx + 1, self.cz)) {
                    chunk.is_dirty = true;
                }
            }
            if z == 0 {
                if let Some(chunk) = storage.chunks.get_mut(&(self.cx, self.cz-1)) {
                    chunk.is_dirty = true;
                }
            }
            if z == CHUNK_SIZE-1 {
                if let Some(chunk) = storage.chunks.get_mut(&(self.cx, self.cz+1)) {
                    chunk.is_dirty = true;
                }
            }
        }

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

#[derive(Component,Default)]
pub struct PlayerChunk{
    pub chunk:Vec<(i32,i32)>,
}


#[derive(Resource,Default)]
pub struct VisibleChunks{
    pub chunks: Vec<(i32,i32)>,
}

pub fn update_visible_chunks(
    mut query_player_chunk:Query<&mut PlayerChunk,With<Player>>,
    mut visible_chunks:ResMut<VisibleChunks>,
    player_position:Res<PlayerPositions>,
    mut storage:ResMut<ChunkStorage>) {
    let player_position_var = player_position.positions[0];
    match ChunkRange::new(0) {
        Some(range) => {
            let current_chunk = get_chunks_playerposition(player_position_var, range);
            let mut value_query_player_chunk = match query_player_chunk.single_mut() {
                Ok(player_chunk) => player_chunk,
                Err(_) => return,
            };
            //The following check will evaluate to true in the following cases:
            //1.First call after start of game because PlayerChunk will be empty
            //2.If current chunk changes after player movement
            if current_chunk != value_query_player_chunk.chunk {
                match ChunkRange::new(DEFAULT_CHUNK_RANGE) {
                    Some(range) => {
                        let current_visible_chunks = get_chunks_playerposition(player_position_var, range);
                        //Zuerst die identifizieren welche nicht gleich sind
                        let hashset_current_visible_chunks:HashSet<(i32,i32)>=current_visible_chunks.iter().copied().collect();
                        let hashset_old_visible_chunks:HashSet<(i32,i32)>=visible_chunks.chunks.iter().copied().collect();
                        let mut helpvector:HashSet<(i32,i32)>=hashset_current_visible_chunks.difference(&hashset_old_visible_chunks).copied().collect();


                        //Für die die nicht gleich sind, Nachbaren in visible chunks identifizieren und diese dort als dirty markieren
                        let mut hashset_vectorneighbours:HashSet<(i32,i32)>=HashSet::new();
                        for &(coordx,coordz) in helpvector.iter(){
                            hashset_vectorneighbours.extend(get_neighbours((coordx,coordz)).iter().copied());

                        }
                        let mut hashset_vectorneighboursintersections:HashSet<(i32,i32)>=hashset_vectorneighbours.intersection(&hashset_current_visible_chunks).copied().collect();
                        for &(coordx,coordz) in hashset_vectorneighboursintersections.iter() {
                            if let Some(chunk)=storage.chunks.get_mut(&(coordx,coordz)){
                                chunk.is_dirty=true;
                            }
                        }
                        visible_chunks.chunks = current_visible_chunks;
                        value_query_player_chunk.chunk=current_chunk;
                    }
                    None => {}
                }
            }
        }
        None => {}

    }
}
#[derive(Resource,Default)]
pub struct ChunkStorage {
    pub chunks: HashMap<(i32,i32), Chunk>,
}

pub fn load_missing_chunks(visible_chunks:Res<VisibleChunks>,mut storage:ResMut<ChunkStorage>) {
    //The following statement will be true in the first run, i.e. game start, because update_visible_chunks is executed before and will set the change tick.
    if visible_chunks.is_changed(){
        for &(coordinatex,coordinatey) in &visible_chunks.chunks{
            if !storage.chunks.contains_key(&(coordinatex,coordinatey)){
                let mut chunk = Chunk::new(coordinatex,coordinatey);
                chunk.generate_chunk();
                storage.chunks.insert((coordinatex,coordinatey), chunk);

            }
        }
    }
}
//Get 4 neighbourhood for a given chunk
pub fn get_neighbours(coordinates:(i32,i32))->Vec<(i32,i32)>{
    let (x, z) = coordinates;

    vec![
        (x + 1, z), // East
        (x - 1, z), // West
        (x, z + 1), // South
        (x, z - 1), // North
    ]
}

pub fn generate_edge_indices()->HashSet<usize>{
    let mut vecindices:HashSet<usize>=HashSet::new();
    for ch in 0..CHUNK_HEIGHT{
        for cs in 0..CHUNK_SIZE{
            //Add index to vector, functional similar to Chunk.index, i.e. x + CHUNK_SIZE * (z + CHUNK_SIZE * y)
            vecindices.insert(cs + CHUNK_SIZE * (0 + CHUNK_SIZE * ch));
            //CHUNK_SIZE-1 because that is the maximal index value
            vecindices.insert(cs + CHUNK_SIZE * (CHUNK_SIZE-1 + CHUNK_SIZE * ch));
        }
    }
    for ch in 0..CHUNK_HEIGHT{
        for cs in 0..CHUNK_SIZE{
            //Add index to vector, functional similar to Chunk.index, i.e. x + CHUNK_SIZE * (z + CHUNK_SIZE * y)
            vecindices.insert(0 + CHUNK_SIZE * (cs + CHUNK_SIZE * ch));
            //CHUNK_SIZE-1 because that is the maximal index value
            vecindices.insert(CHUNK_SIZE-1 + CHUNK_SIZE * (cs + CHUNK_SIZE * ch));
        }
    }
    vecindices
}


pub fn edge_indices_value() -> &'static HashSet<usize> {
    EDGE_INDICES.get_or_init(|| generate_edge_indices())
}