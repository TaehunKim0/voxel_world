mod noise;
mod voxel;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
    diagnostic::LogDiagnosticsPlugin,
    diagnostic::FrameTimeDiagnosticsPlugin
};
use bevy_flycam::prelude::*;
use voxel::world;

#[derive(Resource)]
struct WindowSize {
    x: i32,
    y: i32,
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        WindowSize { x: 800, y: 600 }
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
                mode: bevy::window::WindowMode::Windowed,
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
//       .add_plugins(LogDiagnosticsPlugin::default())
//        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(window_size)
        .add_systems(Startup, world::setup)
        .run();
}
