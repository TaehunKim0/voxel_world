mod noise;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{
    pbr::*,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use rand::prelude::*;
use bevy_flycam::prelude::*;
use noise::perlin;

struct WindowSize {
    x: i32,
    y: i32,
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        WindowSize { x: 1920, y: 1080 }
    }
}

fn main() {
    let window_size = WindowSize::default();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a boy".into(),
                name: Some("bevy.app".into()),
                resolution: (window_size.x as f32, window_size.y as f32).into(),
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
        }))
        .add_plugins(PlayerPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window_size = WindowSize::default();

    for y in 0..100 {
        for x in 0..100 {

            let noise = perlin::perlin_noise2d(x as f32, y as f32, 12);
            println!("{}", noise);
            let range = (noise * 100.) as i32;
            let color = match range {
                n if n < 0 => Color::rgb(0.,0.,1.),
                0 => Color::rgb(0.,1.,0.),
                0..=1 => Color::rgb(88. / 255.,57. / 255., 39. / 255.),
                1..=3 => Color::rgb(66. / 255.,65. / 255., 66. / 255.),
                _ => Color::rgb(66. / 255., 65. / 255. , 66. / 255.)
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::default().mesh()),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    x as f32 * Cuboid::default().size().x,
                    (noise * 100.) as i32 as f32 * Cuboid::default().size().y,
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
