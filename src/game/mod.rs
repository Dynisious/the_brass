//! `game` is the root module. It is ties all the other elements together.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/09/22

pub mod combat;
pub mod factions;
pub mod properties;
pub mod file_system;

/// A type alias for the standard unsigned integer type used in the game.
pub type UInt = u32;
/// The maximum value of UInt.
pub const UINT_MAX: UInt = ::std::u32::MAX;
