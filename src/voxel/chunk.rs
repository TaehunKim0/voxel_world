use super::block::*;
use super::mesh::*;
use bevy::prelude::*;
use noise::NoiseFn;
use noise::Perlin;

use crate::noise::basic_perlin;
use crate::noise::basic_perlin::*;
extern crate noise as other_noise;
pub struct Chunk {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<u32>,
    pub uvs: Vec<Vec2>,
    pub voxel_map: Vec<Vec<Vec<i32>>>,
    pub chunk_coord: ChunkCoord,
}

pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl Chunk {
    pub fn new(chunk_coord: ChunkCoord) -> Self {
        let mut chunk = Chunk {
            vertices: Vec::new(),
            triangles: Vec::new(),
            uvs: Vec::new(),
            voxel_map: Vec::new(),
            chunk_coord,
        };

        for _ in 0..VoxelData::CHUNK_WIDTH {
            let mut row = Vec::with_capacity(VoxelData::CHUNK_HEIGHT as usize);
            for _ in 0..VoxelData::CHUNK_HEIGHT {
                let mut col = Vec::with_capacity(VoxelData::CHUNK_WIDTH as usize);
                for _ in 0..VoxelData::CHUNK_WIDTH {
                    col.push(0);
                }
                row.push(col);
            }
            chunk.voxel_map.push(row);
        }

        chunk.populate_voxel_map();
        chunk.create_mesh_data();

        chunk
    }

    fn create_mesh_data(&mut self) {
        for y in 0..VoxelData::CHUNK_HEIGHT {
            for x in 0..VoxelData::CHUNK_WIDTH {
                for z in 0..VoxelData::CHUNK_WIDTH {
                    self.add_voxel_data(Vec3::new(x as f32, y as f32, z as f32));
                }
            }
        }
    }

    fn add_texture(&mut self, texture_id: i32) {
        let mut y = (texture_id / VoxelData::TEXTURE_ATLAS_SIZE) as f32;
        let mut x = texture_id as f32 - (y * VoxelData::TEXTURE_ATLAS_SIZE as f32) as f32;

        let offset = 0.005;

        x = x * VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE;
        y = y * VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE;

        y = 1.0 - y - VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE;

        self.uvs.push(Vec2::new(
            x + offset,
            y + VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE - offset,
        )); // 좌상단 (LT)
        self.uvs.push(Vec2::new(x + offset, y + offset)); // 좌하단 (LB)
        self.uvs.push(Vec2::new(
            x + VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE - offset,
            y + VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE - offset,
        )); // 우상단 (RT)
        self.uvs.push(Vec2::new(
            x + VoxelData::NORMALIZE_BLOCK_TEXTURE_SIZE - offset,
            y + offset,
        )); // 우하단 (RB)
    }

    fn add_voxel_data(&mut self, pos: Vec3) {
        let mut vertex_index = self.vertices.len() as u32;

        // 6방향의 면 그리기
        for p in 0..6 {
            if self.check_voxel(pos) && !self.check_voxel(pos + VoxelData::FACE_CHECKS[p]) {
                //println!("외부 Voxel 입니다");
                // 각 면(삼각형 2개) 그리기
                // // 1. Vertex, UV 4개 추가

                let block_id =
                    self.voxel_map[pos.x as usize][pos.y as usize][pos.z as usize] as usize;

                let offset = Vec3::new(
                    self.chunk_coord.x as f32 * VoxelData::CHUNK_WIDTH as f32,
                    0.0,
                    self.chunk_coord.y as f32 * VoxelData::CHUNK_WIDTH as f32,
                );

                self.vertices.push(
                    pos + VoxelData::VOXEL_VERTS[VoxelData::VOXEL_TRIS[p as usize][0] as usize]
                        + offset,
                );
                self.vertices.push(
                    pos + VoxelData::VOXEL_VERTS[VoxelData::VOXEL_TRIS[p as usize][1] as usize]
                        + offset,
                );
                self.vertices.push(
                    pos + VoxelData::VOXEL_VERTS[VoxelData::VOXEL_TRIS[p as usize][2] as usize]
                        + offset,
                );
                self.vertices.push(
                    pos + VoxelData::VOXEL_VERTS[VoxelData::VOXEL_TRIS[p as usize][3] as usize]
                        + offset,
                );

                let mut block = Block::new();
                self.add_texture(block.block_types[block_id as usize].get_texture_id(p as i32));

                // // 2. Triangle의 버텍스 인덱스 6개 추가
                self.triangles.push(vertex_index);
                self.triangles.push(vertex_index + 1);
                self.triangles.push(vertex_index + 2);
                self.triangles.push(vertex_index + 2);
                self.triangles.push(vertex_index + 1);
                self.triangles.push(vertex_index + 3);
                vertex_index += 4;
            } else {
                //println!("내부 Voxel 입니다");
            }
        }
    }

    fn check_voxel(&mut self, pos: Vec3) -> bool {
        let x = pos.x.floor() as i32;
        let y = pos.y.floor() as i32;
        let z = pos.z.floor() as i32;

        if !Self::isin_voxel_in_chunk(pos.x as i32, pos.y as i32, pos.z as i32) {
            return false;
        }
        let block = Block::new();
        return block.block_types[self.voxel_map[x as usize][y as usize][z as usize] as usize]
            .is_solid;
    }

    fn populate_voxel_map(&mut self) {
        let perlin = Perlin::new(1932);
        let simplex_noise = other_noise::Simplex::new(other_noise::Simplex::DEFAULT_SEED);
        let zoom = 10.0;
        for x in 0..VoxelData::CHUNK_HEIGHT {
            for y in 0..VoxelData::CHUNK_WIDTH {
                for z in 0..VoxelData::CHUNK_WIDTH {
                    //self.voxel_map[x as usize][y as usize][z as usize] = super::world::World::get_voxel(Vec3::new(x as f32, y as f32, z as f32));
                    self.voxel_map[x as usize][y as usize][z as usize] = 0;

                    if y < 1 {
                        self.voxel_map[x as usize][y as usize][z as usize] = 2;
                    } else if y == VoxelData::CHUNK_HEIGHT - 1 {
                        //let w = perlin.get([x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1]);
                        // /* let w = simplex_noise.get([x as f64 / zoom, y as f64 / zoom, z as f64 / zoom]); */
                        // let w = basic_perlin::perlin_noise2d(x as f32, y as f32, 4);
                        self.voxel_map[x as usize][y as usize][z as usize] = 0;
                    } else {
                        self.voxel_map[x as usize][y as usize][z as usize] = 1;
                    }
                }
            }
        }
    }

    fn isin_voxel_in_chunk(x: i32, y: i32, z: i32) -> bool {
        if (x < 0
            || x > VoxelData::CHUNK_WIDTH - 1
            || y < 0
            || y > VoxelData::CHUNK_HEIGHT - 1
            || z < 0
            || z > VoxelData::CHUNK_WIDTH - 1)
        {
            return false;
        }
        return true;
    }
}
