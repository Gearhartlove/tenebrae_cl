use bevy::prelude::*;
use core::default::Default as Def;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;
use bevy::log;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::CursorIcon::Default;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    Out,
}

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
        // is this the right place to put this?
        .add_state(AppState::InGame)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler)
        // Startup system (cameras)
        .add_startup_system(camera_setup)
        // Run the app
        .run();
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    let mut c_pressed = || {
        log::debug!("clearing game");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    };

    if keys.just_pressed(KeyCode::C) {
        c_pressed();
    }

    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        c_pressed();
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pausing game");
        if state.current() == &AppState::InGame {
            log::info!("pausing game");
            // RFC: how do I pause the game?
            state.set(AppState::Paused).unwrap();
        }
        else if state.current() == &AppState::Out {
            log::info!("unpausing game");
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}