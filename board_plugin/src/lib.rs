pub mod components;
pub mod resources;
mod bounds;
mod systems;

use bevy::log;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy_inspector_egui::RegisterInspectable;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::TileSize;
use resources::BoardPosition;
use components::*;
use crate::bounds::Bounds2;
use crate::resources::tile::Tile;
use bevy::math::Vec3Swizzles;
use crate::resources::board::Board;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board Plugin");
        #[cfg(feature = "debug")]
            {
                app.register_inspectable::<Coordinates>();
                app.register_inspectable::<BombNeighbor>();
                app.register_inspectable::<Bomb>();
                app.register_inspectable::<Uncover>();
            }
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Option<Res<WindowDescriptor>>,
        asset_server: Res<AssetServer>, // The AssetServer Resource > allows loading files from the assets folder
    ) {
        let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
        let bomb_image = asset_server.load("sprites/bomb_emoji.png");

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

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size
        });

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
                spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                );
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

/// Generates the bomb counter text 2D Bundle for a given value
fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
    let (text, color) = (
        count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            }
        );
    // Generate a text bundle
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    color,
                    font,
                    font_size: size,
                },
            }],
            alignment : TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}

// what is this Handle part of the code?
fn spawn_tiles (
    parent: &mut ChildBuilder,
    tile_map: &TileMap,
    size: f32,
    padding: f32,
    color: Color,
    bomb_image: Handle<Image>,
    font: Handle<Font>,
) {
    // Tiles
    for (y, line) in tile_map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let coordinates = Coordinates {
                x: x as u16,
                y: y as u16,
            };
            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size - padding)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * size) + (size / 2.),
                    (y as f32 * size) + (size / 2.),
                    1.,
                ),
                ..Default::default()
            })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coordinates);

            match tile {
                Tile::Bomb => {
                    cmd.insert(Bomb);
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: bomb_image.clone(),
                            ..Default::default()
                        });
                    });
                }
                // If the tiles is a bomb neighbour we add the matching component and a tezt child
                Tile::BombNeighbor(v) => {
                    cmd.insert(BombNeighbor { count: *v });
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(bomb_count_text_bundle(
                            *v,
                            font.clone(),
                            size - padding,
                        ));
                    });
                }
                Tile::Empty => (),
            }
        }
    }
}