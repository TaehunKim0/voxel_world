mod noise;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{
    pbr::*,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
};

use bevy_flycam::prelude::*;

use noise::perlin;
use std::f32::consts::PI;
fn main() {
    let window_x: u32 = 800;
    let window_y: u32 = 600;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a boy".into(),
                    name: Some("bevy.app".into()),
                    resolution: (800., 600.).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();

    // for y in 0..window_y {
    //     for x in 0..window_x {
    //         let result = perlin::perlin_noise2d(x as f32, y as f32, 12);
    //         let idx = ((result + 1.0) * 7.0) as usize;
    //         print!("{}", ascii_chars[idx.min(ascii_chars.len() - 1)]);
    //     }
    //     println!();
    // }
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let test_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        ..default()
    });
    let mut shapes: Vec<Handle<Mesh>> = [].to_vec();

    for _ in 0..10000 {
        shapes.push(meshes.add(Cuboid::default()));
    }

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: test_material.clone(),
                transform: Transform::from_xyz(0.0 + i as f32 * 2., 2.0, -10.0)
                    .with_rotation(Quat::from_rotation_y(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

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

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 6., 7.0).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
    //     ..default()
    // });
}
