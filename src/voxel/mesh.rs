use bevy::prelude::*;
use crate::voxel::chunk::*;


pub struct MeshData {
    pub positions: Vec<[f32;3]>,
    pub normals: Vec<[f32;3]>,
    pub indices: Vec<u32>,
    pub uvs: Vec<[f32;2]>
}

pub struct AtlasUV {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

pub fn update_meshes(chunk:VisibleChunks){
    for chunk in
}


