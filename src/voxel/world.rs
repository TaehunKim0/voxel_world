use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use crate::{noise::*, WindowSize};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>
) {
    for y in 0..100 {
        for x in 0..100 {
            let noise = random_perlin::perlin_noise2d(x as f32, y as f32, 12);

            let range = (noise * 100.) as i32;

            let color = match range {
                n if n < 0 => Color::rgb(0., 0., 1.),
                0..=1 => Color::rgb(66. / 255., 65. / 255., 66. / 255.),
                1..=2 => Color::rgb(88. / 255., 57. / 255., 39. / 255.),
                _ => Color::rgb(126. / 255., 200. / 255., 80. / 255.),
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::default().mesh()),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    x as f32 * Cuboid::default().size().x,
                    (range as f32) * Cuboid::default().size().y,
                    y as f32 * Cuboid::default().size().z,
                ),
                ..default()
            });
        }
    }

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
