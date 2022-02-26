use bevy::prelude::*;
use core::default::Default as Def;

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
    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}