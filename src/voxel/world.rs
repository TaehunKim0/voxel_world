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

#[derive(Resource)]
pub struct World {
    world_size: i32,
    chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new(world_size: i32) -> Self {
        let mut chunks = Vec::with_capacity(world_size as usize);
        for y in 0..world_size {
            let mut row = Vec::with_capacity(world_size as usize);
            for x in 0..world_size {
                row.push(Chunk::new(ChunkCoord { x, y }));
            }
            chunks.push(row);
        }

        World { world_size, chunks }
    }

    pub fn is_chunk_in_world(&mut self, coord: ChunkCoord) -> bool {
        if coord.x > 0
            && coord.x < self.world_size * VoxelData::CHUNK_WIDTH - 1
            && coord.y > 0
            && coord.y < self.world_size * VoxelData::CHUNK_WIDTH - 1
        {
            return true;
        }
        return false;
    }

    pub fn is_voxel_in_world(&mut self, pos: Vec3) -> bool {
        if pos.x > 0.0
            && pos.x < (self.world_size * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.y > 0.0
            && pos.y < (self.world_size * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.z > 0.0
            && pos.z < (self.world_size * VoxelData::CHUNK_WIDTH - 1) as f32
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
    mut voxel_world: Res<World>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>,
) {
    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");
    let num_chunk = voxel_world.world_size;

    for y in 0..num_chunk {
        for x in 0..num_chunk {
            if let Some(row) = voxel_world.chunks.get(y as usize) {
                if let Some(chunk) = row.get(x as usize) {
                    let temp_mesh = Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.vertices.clone())
                    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs.clone())
                    .with_inserted_indices(Indices::U32(chunk.triangles.clone()));

                    let cube_mesh_handle = meshes.add(temp_mesh.clone());
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
