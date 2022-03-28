use bevy::prelude::*;
use core::default::Default as Def;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::CursorIcon::Default;


fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "tenebrae_cl".to_string(),
        width: 600.,
        height: 800.,
        ..Def::default()
    })
        // Bevy default plugins
        .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // Board plugin options
    app.insert_resource(BoardOptions {
        safe_start: true,
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Def::default()
    })
        .add_plugin(BoardPlugin)
        // Startup system (cameras)
        .add_startup_system(camera_setup)
        // Run the app
        .run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}