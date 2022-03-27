use crate::components::Coordinates;

// An event is like a resource but available for 1 frame

#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);
