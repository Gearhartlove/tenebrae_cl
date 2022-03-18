use bevy::prelude::Component;

/// Bomb Component
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inespectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;