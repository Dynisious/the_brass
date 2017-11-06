//! `ships` defines the construction and modification of ships and their components.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/09/22

use game::*;
mod ships;
pub mod weapons;

pub use self::ships::*;

/// A type alias for a `class` of Ship based on its size.
pub type ShipSize = UInt;
/// A type alias for a unit of mass.
pub type Mass = UInt;
