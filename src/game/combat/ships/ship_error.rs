//! `ship_error` defines an Error enum with regards to Ships.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/06

/// An error type relating to Ships.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShipError {
    FuelError,
    ShieldError,
    HullError
}
pub use self::ShipError::*;
