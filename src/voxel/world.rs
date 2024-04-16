use super::block::*;
use super::mesh::*;
use crate::noise::{self, basic_perlin::*, random_perlin::*};
use crate::{noise::*, WindowSize};
use bevy::pbr::{CascadeShadowConfigBuilder};
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
extern crate noise as other_noise;
use super::chunk::*;
use rand::Rng;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>,
) {
    let mut rng = rand::thread_rng();

    const NUM_CHUNKS: i32 = 5; // 청크 개수를 조절할 수 있는 상수

    let chunk = Chunk::new(0.0,0.0,0.0);

    let temp_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs)
    .with_inserted_indices(Indices::U32(chunk.triangles));


    let cube_mesh_handle = meshes.add(temp_mesh.clone());

    let random_color = Color::rgb(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    );

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: random_color,
                ..default()
            }),
            ..default()
        },
        // 다른 필요한 컴포넌트들 추가 가
    ));

    // directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
