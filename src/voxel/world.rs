use super::mesh::*;
use crate::WindowSize;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
extern crate noise as other_noise;
use super::chunk::*;
use rand::Rng;
use std::f32::consts::PI;
pub struct World {}

impl World {
    pub const WORLD_SIZE: i32 = 5;

    pub fn is_chunk_in_world(&mut self, coord: ChunkCoord) -> bool {
        if coord.x > 0
            && coord.x < Self::WORLD_SIZE * VoxelData::CHUNK_WIDTH - 1
            && coord.y > 0
            && coord.y < Self::WORLD_SIZE * VoxelData::CHUNK_WIDTH - 1
        {
            return true;
        }
        return false;
    }

    pub fn is_voxel_in_world(&mut self, pos: Vec3) -> bool {
        if pos.x > 0.0
            && pos.x < (Self::WORLD_SIZE * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.y > 0.0
            && pos.y < (Self::WORLD_SIZE * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.z > 0.0
            && pos.z < (Self::WORLD_SIZE * VoxelData::CHUNK_WIDTH - 1) as f32
        {
            return true;
        }

        return false;
    }

    pub fn get_voxel(pos: Vec3) -> i32 {
        if pos.y < 1.0 {
            1
        } else if pos.y as i32 == VoxelData::CHUNK_HEIGHT - 1 {
            3
        } else {
            2
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>,
) {
    let mut rng = rand::thread_rng();

    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");

    const NUM_CHUNKS: i32 = 4;

    for y in 0..NUM_CHUNKS {
        for x in 0..NUM_CHUNKS {
            let chunk_coord = ChunkCoord {x, y};

            let chunk = Chunk::new(chunk_coord);
            println!("chunk len : {}", chunk.vertices.len());

            let temp_mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs)
            .with_inserted_indices(Indices::U32(chunk.triangles));

            let cube_mesh_handle = meshes.add(temp_mesh.clone());
            let _random_color = Color::rgb(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            );

            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh_handle,
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(texture_handle.clone()),
                        ..default()
                    }),
                    ..default()
                },
                // 다른 필요한 컴포넌트들 추가
            ));
        }
    }
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        brightness: 200.0,
    });
    // directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            ..default()
        },
        transform: Transform {
            translation: Vec3::new(20.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(10.0),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
