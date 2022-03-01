use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

// coordinate sum functionality
impl Add for Coordinates {
    type Output = Self; // called a "type aliases" : type the function "works on" , needs to be
                        // defined like the functions do as well in the trait; Q: what's being added?

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
       Self {
           // saturating sub avoids *panic* if the subtraction result is negative
           x: self.x.saturating_sub(rhs.x),
           y: self.y.saturating_sub(rhs.y),
       }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}