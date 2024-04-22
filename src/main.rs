mod noise;
mod voxel;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::LogDiagnosticsPlugin, pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin}, prelude::*, render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        view::NoFrustumCulling,
        RenderPlugin,
    }, ui::update, window::{PresentMode, WindowTheme}
};
use bevy_flycam::prelude::*;
use image::{flat::View, Rgb, Rgba};
use voxel::world;
use bevy_atmosphere::prelude::*;

#[derive(Resource)]
struct WindowSize {
    x: i32,
    y: i32,
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        WindowSize { x: 1280, y: 720 }
    }
}

fn main() {
    let window_size = WindowSize::default();
    let mut voxel_world = voxel::world::World::new();
    voxel_world.generate_world();

    App::new()
        // .add_plugins((
        //     DefaultPlugins.set(RenderPlugin {
        //         render_creation: RenderCreation::Automatic(WgpuSettings {
        //             features: WgpuFeatures::POLYGON_MODE_LINE,
        //             ..default()
        //         }),
        //         ..default()
        //     }),
        //     //WireframePlugin,
        // ))
        // .insert_resource(WireframeConfig {
        //     global: true,
        //     default_color: Color::Rgba {
        //         red: 1.0,
        //         green: 1.0,
        //         blue: 1.0,
        //         alpha: 1.0,
        //     },
        // })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a boy".into(),
                name: Some("bevy.app".into()),
                position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
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
        .add_plugins((PlayerPlugin,AtmospherePlugin))
        .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(window_size)
        .insert_resource(voxel_world)
        .add_systems(Startup, world::setup)
        .add_systems(Update, world::update)
        .run();
}
