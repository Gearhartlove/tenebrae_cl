pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::reflect::List;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::TileSize;
use resources::BoardPosition;
use components::Coordinates;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Option<Res<WindowDescriptor>>,
    ) {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.set_bombs(40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        // if no option is set, use the default one
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        let window_options = match window {
            None => WindowDescriptor::default(),
            Some(o) => o.clone(),
        };

        // Tilemap generation
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());

        // We define the size of our tiles in world space
        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => adaptive_tile_size(
                window_options,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // Define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Costume(p) => p,
        };

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite
                // pivot is centered
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                // Tiles
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _tile) in line.iter().enumerate() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..Default::default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            // Add the 'Coordinates' component to our tile entity
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

}

/// Computes a tile size that matches the window according to the tile map size
fn adaptive_tile_size(
    window: WindowDescriptor,
    (min, max): (f32, f32), // Tile size constraints
    (width, height): (u16, u16), // TIle map dimensions
) -> f32 {
    let max_width = window.width / width as f32;
    let max_height = window.height / height as f32;
    max_width.min(max_height).clamp(min, max)
}

// pub fn create_board(
//     mut commands: Commands,
//     board_options: Option<Res<BoardOptions>>,
//     window: Option<Res<WindowDescriptor>>,
// ) {
//     // if no option is set, use the default one
//     let options = match board_options {
//         None => BoardOptions::default(),
//         Some(o) => o.clone(),
//     };
//     let window_options = match window {
//         None => WindowDescriptor::default(),
//         Some(o) => o.clone(),
//     };
//
//     // Tilemap generation
//     let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
//     tile_map.set_bombs(options.bomb_count);
//     #[cfg(feature = "debug")]
//     // Tilemap debugging
//     log::info!("{}", tile_map.console_output());
//
//     // We define the size of our tiles in world space
//     let tile_size = match options.tile_size {
//         TileSize::Fixed(v) => v,
//         TileSize::Adaptive { min, max } => adaptive_tile_size(
//             window_options,
//             (min, max),
//             (tile_map.width(), tile_map.height()),
//         ),
//     };
//
//     let board_size = Vec2::new(
//         tile_map.width() as f32 * tile_size,
//         tile_map.height() as f32 * tile_size,
//     );
//     log::info!("board size: {}", board_size);
//     // Define the board anchor position (bottom left)
//     let board_position = match options.position {
//         BoardPosition::Centered { offset } => {
//             Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
//         }
//         BoardPosition::Costume(p) => p,
//     };
//
//     commands
//         .spawn()
//         .insert(Name::new("Board"))
//         .insert(Transform::from_translation(board_position))
//         .insert(GlobalTransform::default())
//         .with_children(|parent| {
//             // We spawn the board background sprite at the center of the board, since the sprite
//             // pivot is centered
//             parent
//                 .spawn_bundle(SpriteBundle {
//                     sprite: Sprite {
//                         color: Color::WHITE,
//                         custom_size: Some(board_size),
//                         ..Default::default()
//                     },
//                     transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
//                     ..Default::default()
//                 })
//                 .insert(Name::new("Background"));
//
//             // Tiles
//             for (y, line) in tile_map.iter().enumerate() {
//                 for (x, _tile) in line.iter().enumerate() {
//                     parent
//                         .spawn_bundle(SpriteBundle {
//                             sprite: Sprite {
//                                 color: Color::GRAY,
//                                 custom_size: Some(Vec2::splat(
//                                     tile_size - options.tile_padding as f32,
//                                 )),
//                                 ..Default::default()
//                             },
//                             transform: Transform::from_xyz(
//                                 (x as f32 * tile_size) + (tile_size / 2.),
//                                 (y as f32 * tile_size) + (tile_size / 2.),
//                                 1.,
//                             ),
//                             ..Default::default()
//                         })
//                         .insert(Name::new(format!("Tile ({}, {})", x, y)))
//                         // Add the 'Coordinates' component to our tile entity
//                         .insert(Coordinates {
//                             x: x as u16,
//                             y: y as u16,
//                         });
//                 }
//             }
//     });
// }