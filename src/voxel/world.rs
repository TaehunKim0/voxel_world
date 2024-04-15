use super::block::*;
use crate::noise::{self, random_perlin::*, basic_perlin::*};
use crate::{noise::*, WindowSize};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use other_noise::NoiseFn;

extern crate noise as other_noise;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _window_size: Res<WindowSize>,
) {
    const PLANET_RADIUS: i32 = 40;
    let simplex_noise = other_noise::Simplex::new(other_noise::Simplex::DEFAULT_SEED);
    let zoom: f64 = 10.0;

    for x in (0)..PLANET_RADIUS {
        for y in (0)..PLANET_RADIUS {
            for z in (0)..PLANET_RADIUS {
                let w = simplex_noise.get([x as f64 / zoom, y as f64 / zoom, z as f64 / zoom]);
                //let w = noise::basic_perlin::perlin_noise3d(x as f32, y as f32, z as f32, 8);
                let inside_planet = ((x * x + y * y + z * z) as f64).sqrt() <= PLANET_RADIUS as f64;

                if inside_planet && w > 0.0 {
                    let distance_from_center =
                        ((x * x + y * y + z * z) as f64).sqrt() / PLANET_RADIUS as f64;

                    let material = match distance_from_center {
                        d if d < 0.5 => StandardMaterial {
                            base_color: Color::rgb(0.8, 0.2, 0.1), // Lava texture
                            ..default()
                        },
                        d if d > 0.95 => StandardMaterial {
                            base_color: Color::rgb(0.3, 0.7, 0.2), // Land texture (Grass)
                            ..default()
                        },
                        d if d > 0.9 => StandardMaterial {
                            base_color: Color::rgb(0.6, 0.4, 0.2), // Dirt texture
                            ..default()
                        },
                        _ => StandardMaterial {
                            base_color: Color::rgb(0.1, 0.3, 0.8), // Ocean texture
                            ..default()
                        },
                    };

                    let material = materials.add(material);
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::default().mesh()),
                        material,
                        transform: Transform::from_xyz(
                            x as f32 * Cuboid::default().size().x,
                            y as f32 * Cuboid::default().size().y,
                            z as f32 * Cuboid::default().size().z,
                        ),
                        ..default()
                    });
                }
            }
        }
    } // for x in 0..MAP_SIZE {
      //     for y in 0..MAP_SIZE {
      //         for z in 0..MAP_SIZE {
      //             if ((x * x + y * y + z * z) as f64).sqrt() <= MAP_SIZE as f64 {
      //                 commands.spawn(PbrBundle {
      //                     mesh: meshes.add(Cuboid::default().mesh()),
      //                     material: materials.add(StandardMaterial {
      //                         base_color: Color::rgb(0.8, 0.7, 0.6),
      //                         ..default()
      //                     }),
      //                     transform: Transform::from_xyz(
      //                         x as f32 * Cuboid::default().size().x,
      //                         y as f32 * Cuboid::default().size().y,
      //                         z as f32 * Cuboid::default().size().z,
      //                     ),
      //                     ..default()
      //                 });
      //             }
      //         }
      //     }
      // }

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
