use bevy::prelude::*;

use super::chunk::ChunkCoord;

pub struct VoxelData {}
impl VoxelData {
    pub const CHUNK_HEIGHT: i32 = 20;
    pub const CHUNK_WIDTH: i32 = 10;

    pub const TEXTURE_ATLAS_SIZE: i32 = 4;
    pub const NORMALIZE_BLOCK_TEXTURE_SIZE: f32 = 1.0 / 4 as f32;

    pub const WORLD_SIZE: usize = 30;
    pub const VIEW_DISTANCE_IN_CHUNKS: i32 = 10;

    // 큐브의 8개 버텍스의 상대 위치
    pub const VOXEL_VERTS: [Vec3; 8] = [
        // Front
        Vec3::new(0.0, 0.0, 0.0), // LB
        Vec3::new(1.0, 0.0, 0.0), // RB
        Vec3::new(1.0, 1.0, 0.0), // RT
        Vec3::new(0.0, 1.0, 0.0), // LT
        // Back
        Vec3::new(0.0, 0.0, 1.0), // LB
        Vec3::new(1.0, 0.0, 1.0), // RB
        Vec3::new(1.0, 1.0, 1.0), // RT
        Vec3::new(0.0, 1.0, 1.0), // LT
    ];

    //    7 ──── 6
    //  / │       / │
    // 3 ──── 2   │
    // │  │     │  │
    // │  4───│─5
    // │/        │/
    // 0 ──── 1
    // 큐브의 각 면을 이루는 삼각형들의 버텍스 인덱스 데이터
    pub const VOXEL_TRIS: [[i32; 4]; 6] = [
        // Back Face (-Z)
        [0, 3, 1, 2],
        // Front Face (+Z)
        [5, 6, 4, 7],
        // Top Face (+Y)
        [3, 7, 2, 6],
        // Bottom Face (-Y)
        [1, 5, 0, 4],
        // Left Face (-X)
        [4, 7, 0, 3],
        // Right Face (+X)
        [1, 2, 5, 6],
    ];

    pub const FACE_CHECKS: [Vec3; 6] = [
        Vec3::new(0.0, 0.0, -1.0), // Back Face   (-Z)
        Vec3::new(0.0, 0.0, 1.0),  // Front Face  (+Z)
        Vec3::new(0.0, 1.0, 0.0),  // Top Face    (+Y)
        //
        Vec3::new(0.0, -1.0, 0.0), // Bottom Face (-Y)
        Vec3::new(-1.0, 0.0, 0.0), // Left Face   (-X)
        Vec3::new(1.0, 0.0, 0.0),  // Right Face  (+X)
    ];

    // voxelTris의 버텍스 인덱스 순서에 따라 정의된 UV 좌표 데이터
    pub const VOXEL_UVS: [Vec2; 4] = [
        Vec2::new(0.0, 0.0), // LB
        Vec2::new(0.0, 1.0), // LT
        Vec2::new(1.0, 0.0), // RB
        Vec2::new(1.0, 1.0), // RT
    ];
}
