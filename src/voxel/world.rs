use super::block::EBlockType;
use super::mesh::*;
use crate::WindowSize;
use bevy::math::vec3;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::view::VisibleEntities;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
use bevy::utils::uuid::generate_composite_uuid;
extern crate noise as other_noise;
use super::chunk::*;
use rand::Rng;
use std::collections::HashMap;
use std::f32::consts::PI;

extern crate bevy_flycam;
use bevy_atmosphere::prelude::*;
use bevy_flycam::prelude::*;

#[derive(Resource)]
pub struct World {
    chunk_map: std::collections::HashMap<ChunkCoord, Chunk>,
    prev_chunk_coord_list: Vec<ChunkCoord>,
    current_chunk_coord_list: Vec<ChunkCoord>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunk_map: std::collections::HashMap::new(),
            prev_chunk_coord_list: Vec::new(),
            current_chunk_coord_list: Vec::new(),
        }
    }

    pub fn generate_world(&mut self) {
        let range = VoxelData::WORLD_SIZE as i32 / 2;

        for y in -range..range {
            for x in -range..range {
                let coord = ChunkCoord { x, y };
                let mut chunk = Chunk::new(coord.clone());
                chunk.is_updated = true;
                self.chunk_map.insert(coord.clone(), chunk);
            }
        }
    }

    pub fn get_chunkcoord_from_pos(pos: Vec3) -> ChunkCoord {
        let x = (pos.x / VoxelData::CHUNK_WIDTH as f32).floor() as i32;
        let y = (pos.z / VoxelData::CHUNK_HEIGHT as f32).floor() as i32;

        ChunkCoord { x, y }
    }

    pub fn check_view_distance(&mut self, pos: Vec3) {
        let coord = Self::get_chunkcoord_from_pos(pos);
        let range = VoxelData::VIEW_DISTANCE_IN_CHUNKS;

        // 모든 청크를 비활성화
        for chunk in self.chunk_map.values_mut() {
            chunk.is_active = false;
        }

        for y in (coord.y - range as i32)..(coord.y + range as i32) {
            for x in (coord.x - range as i32)..(coord.x + range as i32) {
                let coord = ChunkCoord { x, y };

                if let Some(chunk) = self.chunk_map.get_mut(&coord) {
                    chunk.is_active = true;
                }

                if !self.chunk_map.contains_key(&coord) {
                    let mut chunk = Chunk::new(coord.clone());
                    chunk.is_updated = true;
                    self.chunk_map.insert(coord.clone(), chunk);
                }
            }
        }
    }

    pub fn is_chunk_in_world(&mut self, coord: &ChunkCoord) -> bool {
        //println!("{0} , {1} , {2}", coord.x, coord.y, VoxelData::WORLD_SIZE as i32 * VoxelData::CHUNK_WIDTH - 1);

        if coord.x >= 0
            && coord.x < VoxelData::WORLD_SIZE as i32
            && coord.y >= 0
            && coord.y < VoxelData::WORLD_SIZE as i32
        {
            return true;
        }
        return false;
    }

    pub fn is_voxel_in_world(&mut self, pos: Vec3) -> bool {
        if pos.x > 0.0
            && pos.x < (VoxelData::WORLD_SIZE as i32 * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.y > 0.0
            && pos.y < (VoxelData::WORLD_SIZE as i32 * VoxelData::CHUNK_WIDTH - 1) as f32
            && pos.z > 0.0
            && pos.z < (VoxelData::WORLD_SIZE as i32 * VoxelData::CHUNK_WIDTH - 1) as f32
        {
            return true;
        }

        return false;
    }

    pub fn get_voxel(pos: Vec3) -> i32 {
        if pos.y < 1.0 {
            EBlockType::Stone as i32
        } else if pos.y as i32 == VoxelData::CHUNK_HEIGHT - 1 {
            EBlockType::Sand as i32
        } else {
            EBlockType::BedRock as i32
        }
    }

    pub fn get_chunk_is_update(&mut self, coord: ChunkCoord) -> bool {
        if let Some(chunk) = self.chunk_map.get_mut(&coord) {
            chunk.is_updated
        } else {
            false
        }
    }

    pub fn get_chunk_is_active(&mut self, coord: ChunkCoord) -> bool {
        if let Some(chunk) = self.chunk_map.get_mut(&coord) {
            chunk.is_active
        } else {
            false
        }
    }
}

pub fn update_chunk_in_view_range(
    mut world: ResMut<World>,
    mut query: Query<(&mut Visibility, &mut ChunkCoord)>,
) {
    for (mut visible, chunk) in query.iter_mut() {
        if world.get_chunk_is_active(chunk.clone()) {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}

pub fn setup(
    mut voxel_world: ResMut<World>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>,
) {
    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");

    for (coord, chunk) in &mut voxel_world.chunk_map {
        if chunk.is_updated {
            let temp_mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.vertices.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs.clone())
            .with_inserted_indices(Indices::U32(chunk.triangles.clone()));

            let cube_mesh_handle = meshes.add(temp_mesh.clone());
            commands
                .spawn((
                    PbrBundle {
                        mesh: cube_mesh_handle,
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(texture_handle.clone()),
                            ..default()
                        }),
                        ..default()
                    },
                    ChunkCoord {x:coord.x, y: coord.y },
                ))
                .insert(VisibilityBundle {
                    ..Default::default()
                });
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

    commands.spawn(AtmosphereCamera::default());
}

pub fn update(
    mut voxel_world: ResMut<World>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    // 뷰 거리 체크
    voxel_world.check_view_distance(query.single().translation);
    return;
    // 새로운 청크 스폰
    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");
    for (coord, chunk) in &mut voxel_world.chunk_map {
        if chunk.is_updated && !chunk.is_active {
            let temp_mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.vertices.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs.clone())
            .with_inserted_indices(Indices::U32(chunk.triangles.clone()));

            let cube_mesh_handle = meshes.add(temp_mesh.clone());
            commands
                .spawn((
                    PbrBundle {
                        mesh: cube_mesh_handle,
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(texture_handle.clone()),
                            ..default()
                        }),
                        ..default()
                    },
                    ChunkCoord {x:coord.x, y: coord.y },
                ))
                .insert(VisibilityBundle {
                    ..Default::default()
                });
        }
    }
}
