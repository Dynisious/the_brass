//! `ships` defines the construction and modification of ships and their components.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/10

use game::*;

pub mod ship_error;
pub mod attacks;
pub mod ship_template;
pub mod ship;
pub mod reduced_ship;

pub use self::ship_error::*;
pub use self::attacks::*;
pub use self::ship_template::*;
pub use self::ship::*;
pub use self::reduced_ship::*;

/// A type alias for a `class` of Ship based on its size.
pub type ShipSize = UInt;
/// A type alias for a unit of mass.
pub type Mass = UInt;
