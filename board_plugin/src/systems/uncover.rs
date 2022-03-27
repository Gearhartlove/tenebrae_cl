use bevy::log;
use bevy::prelude::*;
use crate::{Bomb, Board, BombNeighbor, Coordinates, Uncover};
use crate::events::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

// First query written
pub fn uncover_tiles(
    mut commands: Commands, // included for entity manipulation
    mut board: ResMut<Board>, //ResMut: uniquye mutable borrow of a resource
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>
) {
    // Iterate through tile covers to uncover
    for (entity, parent) in children.iter() {
        commands
            .entity(entity)
            // despawns potential child entities and will unlink the tile cover from
            // the board tile entity
            .despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.0) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{:?}", e); //RFC?
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            None => ("Tried to uncover an already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }
        if bomb.is_some() {
            log::info!("Boom !");
            // TODO: Add explosion event
        }
        // If the tile is empty
        else if bomb_counter.is_none() {
            // .. Propagate the uncovering by adding the 'Uncover' component to adjacent tiles
            // which will then be removed next frame
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }


}