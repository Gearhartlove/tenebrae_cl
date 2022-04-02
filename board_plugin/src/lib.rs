pub mod components;
pub mod resources;
mod bounds;
mod systems;
mod events;

use bevy::log;
use bevy::prelude::*;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::TileSize;
use resources::BoardPosition;
use components::*;
use crate::bounds::Bounds2;
use crate::resources::tile::Tile;
use bevy::math::Vec3Swizzles;
use crate::resources::board::Board;
use bevy::utils::{AHashExt, HashMap};
use crate::events::*;
use crate::systems::input::input_handling;
use crate::systems::uncover::{trigger_event_handler, uncover_tiles};
use bevy_inspector_egui::RegisterInspectable;
use bevy::ecs::schedule::StateData;
use crate::resources::BoardAssets;

pub struct BoardPlugin<T> {
    pub running_state: T,
}


impl<T: StateData>  Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        // When the running states comes into the stack we load a board
        app
            .add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_board),
        )
            .add_system_set(
                SystemSet::on_update(self.running_state.clone())
                    .with_system(systems::input::input_handling)
                    .with_system(systems::uncover::trigger_event_handler),
            )
            .add_system_set(
                SystemSet::on_in_stack_update(self.running_state.clone())
                    .with_system(systems::uncover::uncover_tiles),
            )
            .add_system_set(SystemSet::on_exit(self.running_state.clone())
                .with_system(Self::cleanup_board),
            )
            .add_event::<TileTriggerEvent>();


        // app.add_startup_system(Self::create_board)
        //     .add_system(input_handling)
        //     .add_system(trigger_event_handler)
        //     .add_system(uncover_tiles)
        //     .add_event::<TileTriggerEvent>();
        // log::info!("Loaded Board Plugin");
        // #[cfg(feature = "debug")]
        //     {
        //         app.register_inspectable::<Coordinates>();
        //         app.register_inspectable::<BombNeighbor>();
        //         app.register_inspectable::<Bomb>();
        //         app.register_inspectable::<Uncover>();
        //     }
    }
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assets: Res<BoardAssets>,
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

        let mut covered_tiles = HashMap::with_capacity((tile_map.width()
            * tile_map.height()).into());

        let mut safe_start = None;
        let board_entity = commands
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
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
                spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start
                );

            })
            .id();

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
        });
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
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
fn bomb_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
    let color = board_assets.bomb_counter_color(count);
    // Generate a text bundle
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: count.to_string(),
                style: TextStyle {
                    color,
                    font: board_assets.bomb_counter_font.clone(),
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
    board_assets: &BoardAssets,
    covered_tiles: &mut HashMap<Coordinates, Entity>,
    safe_start_entity: &mut Option<Entity>,
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
                    color: board_assets.tile_material.color,
                    custom_size: Some(Vec2::splat(size - padding)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * size) + (size / 2.),
                    (y as f32 * size) + (size / 2.),
                    1.,
                ),
                texture: board_assets.tile_material.texture.clone(),
                ..Default::default()
            })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coordinates);

            // Add the cover sprites ?
            cmd.with_children(|parent| {
                let entity = parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(size - padding)),
                            color: board_assets.covered_tile_material.color,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 2.),
                        texture: board_assets.covered_tile_material.texture.clone(),
                        ..Default::default()
                    })
                    .insert(Name::new("Tile Cover"))
                    .id();
                covered_tiles.insert(coordinates, entity);
                if safe_start_entity.is_none() && *tile == Tile::Empty {
                    *safe_start_entity = Some(entity);
                }
            });

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
                            //RFC?
                            texture: board_assets.bomb_material.texture.clone(),
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
                            board_assets,
                            size - padding,
                        ));
                    });
                }
                Tile::Empty => (),
            }
        }
    }
}