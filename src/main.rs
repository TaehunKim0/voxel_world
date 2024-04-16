mod noise;
mod voxel;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    diagnostic::LogDiagnosticsPlugin,
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        view::NoFrustumCulling,
        RenderPlugin,
    },
    window::{PresentMode, WindowTheme},
};
use bevy_flycam::prelude::*;
use image::{flat::View, Rgb, Rgba};
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
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            // You need to add this plugin to enable wireframe rendering
            //WireframePlugin,
        ))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
        })
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "I am a boy".into(),
        //         name: Some("bevy.app".into()),
        //         resolution: (window_size.x as f32, window_size.y as f32).into(),
        //         present_mode: PresentMode::AutoNoVsync,
        //         prevent_default_event_handling: false,
        //         window_theme: Some(WindowTheme::Dark),
        //         enabled_buttons: bevy::window::EnabledButtons {
        //             maximize: false,
        //             ..Default::default()
        //         },
        //         mode: bevy::window::WindowMode::Windowed,
        //         visible: true,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        .add_plugins(PlayerPlugin)
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(window_size)
        .add_systems(Startup, world::setup)
        .run();
}
