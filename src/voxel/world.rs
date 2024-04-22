use super::mesh::*;
use crate::WindowSize;
use bevy::math::vec3;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
use bevy::utils::uuid::generate_composite_uuid;
extern crate noise as other_noise;
use super::chunk::*;
use rand::Rng;
use std::f32::consts::PI;

extern crate bevy_flycam;
use bevy_flycam::prelude::*;

#[derive(Resource)]
pub struct World {
    chunks: [[Chunk; VoxelData::WORLD_SIZE]; VoxelData::WORLD_SIZE],
}

impl World {
    pub fn new() -> Self {
        use core::array;

        let temp = World {
            chunks: array::from_fn(|_y| array::from_fn(|_x| Chunk::default())),
        };

        temp
    }

    pub fn generate_world(&mut self) {
        for y in 0..VoxelData::VIEW_DISTANCE_IN_CHUNKS {
            for x in 0..VoxelData::VIEW_DISTANCE_IN_CHUNKS {
                self.chunks[y as usize][x as usize] = Chunk::new(ChunkCoord {
                    x: x as i32,
                    y: y as i32,
                });
                self.chunks[y as usize][x as usize].is_updated = true;
            }
        }
    }

    pub fn get_chunkcoord_from_pos(pos: Vec3) -> ChunkCoord {
        if pos.x < 0.0 || pos.z < 0.0 {
            return ChunkCoord { x: -1, y: -1 };
        }

        let x = (pos.x / VoxelData::CHUNK_WIDTH as f32);
        let y = (pos.z / VoxelData::CHUNK_HEIGHT as f32);

        return ChunkCoord {
            x: x as i32,
            y: y as i32,
        };
    }

    pub fn check_view_distance(&mut self, pos: Vec3) {
        let mut coord = Self::get_chunkcoord_from_pos(pos);
        //println!("{} {} / {} {}", pos.x, pos.z, coord.x, coord.y);
        for y in (coord.y)..(coord.y + VoxelData::VIEW_DISTANCE_IN_CHUNKS) {
            for x in (coord.x)..(coord.x + VoxelData::VIEW_DISTANCE_IN_CHUNKS) {
                if self.is_chunk_in_world(&mut coord) {
                    //println!("청크가 월드(20x20) 내에 있음.{} {} / {} {}",pos.x, pos.z, coord.x, coord.y);
                    if let Some(row) = self.chunks.get_mut(y as usize) {
                        if let Some(chunk) = row.get_mut(x as usize) {
                            //println!("청크 존재함 {} {} / {} {} {} ",coord.x, coord.y, y, x, chunk.is_updated);
                            if !chunk.is_updated {
                                //println!("생성 안된 청크 {} {} ", y, x);
                                let mut new_chunk = Chunk::new(ChunkCoord {
                                    x: x as i32,
                                    y: y as i32,
                                });
                                new_chunk.is_updated = true;
                                self.chunks[y as usize][x as usize] = new_chunk;

                                println!("청크 정보 생성 {} {} ", x, y);

                            }
                        }
                    }
                } else {
                    //println!("청크가 월드(20x20) 내에 없음. {} {}", coord.x, coord.y);
                }
            }
        }
    }

    pub fn is_chunk_in_world(&mut self, coord: &mut ChunkCoord) -> bool {
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
            1
        } else if pos.y as i32 == VoxelData::CHUNK_HEIGHT - 1 {
            3
        } else {
            2
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
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");
    let num_chunk = VoxelData::WORLD_SIZE;

    for y in 0..num_chunk {
        for x in 0..num_chunk {
            if let Some(row) = voxel_world.chunks.get_mut(y as usize) {
                if let Some(chunk) = row.get_mut(x as usize) {
                    if chunk.is_updated == true {
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
                        chunk.is_active = true;
                    }
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

    // 새로운 청크 스폰
    let texture_handle: Handle<Image> = asset_server.load("Blocks.png");
    for y in 0..VoxelData::WORLD_SIZE {
        for x in 0..VoxelData::WORLD_SIZE {
            if let Some(row) = voxel_world.chunks.get_mut(y as usize) {
                if let Some(chunk) = row.get_mut(x as usize) {
                    if chunk.is_updated && !chunk.is_active {
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

                        chunk.is_active = true;

                    }
                }
            }
        }
    }
}
