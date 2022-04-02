use bevy::prelude::*;
use core::default::Default as Def;
use board_plugin::BoardPlugin;
use board_plugin::resources::{BoardAssets, BoardOptions, SpriteMaterial};
use bevy::log;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::CursorIcon::Default;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
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
        app.add_plugin(WorldInspectorPlugin::new())
        // Board plugin options
        // is this the right place to put this?
        .add_state(AppState::Out)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler)
        // Startup system (cameras)
        .add_startup_system(camera_setup)
        .add_startup_system(setup_board)
        // Run the app
        .run();
}


fn state_handler(mut game_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    let mut set_clear_state = |state: &mut ResMut<State<AppState>>| {
        log::debug!("clearing game");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    };

    let mut set_gen_state = |state: &mut ResMut<State<AppState>>| {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    };

    //Generate
    if keys.just_pressed(KeyCode::G) {
        set_clear_state(&mut game_state);
    }
    // game_state needs to leave the scope to exit . . . kind of jank xD
    set_gen_state(&mut game_state);

    // TODO: implement pause
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(
    mut  commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Def::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Def::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Def::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Def::default()
        },
        bomb_counter_font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb_emoji.png"),
            color: Color::WHITE,
        }
    });
    // Plugin Activation
    state.set(AppState::InGame).unwrap();
}