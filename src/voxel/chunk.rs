use std::ops::Index;

use super::mesh::*;

use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

pub struct Chunk {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<u32>,
    pub uvs: Vec<Vec2>,
    pub voxel_map: Vec<Vec<Vec<bool>>>,
}

impl Chunk {
    pub fn new(chunk_x: f32, chunk_y: f32, chunk_z: f32) -> Self {
        let mut chunk = Chunk {
            vertices: Vec::new(),
            triangles: Vec::new(),
            uvs: Vec::new(),
            voxel_map: Vec::new(),
        };

        for _ in 0..VoxelData::CHUNK_WIDTH {
            let mut row = Vec::with_capacity(VoxelData::CHUNK_HEIGHT as usize);

            for _ in 0..VoxelData::CHUNK_HEIGHT {
                let mut col = Vec::with_capacity(VoxelData::CHUNK_WIDTH as usize);

                for _ in 0..VoxelData::CHUNK_WIDTH {
                    col.push(false);
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

    fn add_voxel_data(&mut self, pos: Vec3) {
        let mut vertex_index = self.vertices.len() as u32;

        // 6방향의 면 그리기
        for p in 0..6 {
            // if self.check_voxel(pos) && !self.check_voxel(pos + VoxelData::FACE_CHECKS[p]) {
            //     println!("Voxel 이 내부를 바라보고 있습니다.");
            //     continue;
            // }

            println!("외부 Voxel 입니다");

            // 각 면(삼각형 2개) 그리기
            // // 1. Vertex, UV 4개 추가
            for i in 0..=3 {
                let triangle_index = VoxelData::VOXEL_TRIS[p][i];
                self.vertices
                    .push(VoxelData::VOXEL_VERTS[triangle_index as usize] + pos);
                self.uvs.push(VoxelData::VOXEL_UVS[i as usize]);
            }

            // // 2. Triangle의 버텍스 인덱스 6개 추가
            self.triangles.push(vertex_index);
            self.triangles.push(vertex_index + 1);
            self.triangles.push(vertex_index + 2);
            self.triangles.push(vertex_index + 2);
            self.triangles.push(vertex_index + 1);
            self.triangles.push(vertex_index + 3);
            vertex_index += 4;
        }
    }

    fn check_voxel(&mut self, pos: Vec3) -> bool {
        let x = pos.x.floor() as i32;
        let y = pos.y.floor() as i32;
        let z = pos.z.floor() as i32;

        if x < 0
            || x > VoxelData::CHUNK_WIDTH - 1
            || y < 0
            || y > VoxelData::CHUNK_HEIGHT - 1
            || z < 0
            || z > VoxelData::CHUNK_WIDTH - 1
        {
            return false;
        }

        return self.voxel_map[x as usize][y as usize][z as usize];
    }

    fn populate_voxel_map(&mut self) {
        for x in 0..VoxelData::CHUNK_HEIGHT {
            for y in 0..VoxelData::CHUNK_WIDTH {
                for z in 0..VoxelData::CHUNK_WIDTH {
                    self.voxel_map[x as usize][y as usize][z as usize] = true;
                }
            }
        }
    }
}
