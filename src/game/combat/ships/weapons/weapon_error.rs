//! `weapon_error` defines the `WeaponError` type, it's construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

/// A `WeaponError` is an error produced if a call to `ReducedShip::new` fails.
#[derive(Debug, PartialEq, Eq)]
pub enum WeaponError {
    /// Produced if one of the `DistinctWeapon`s does 0 damage.
    DamageError,
    /// Produced if one of the `DistinctWeapon`s produces 0 attacks.
    AttacksError,
    /// Produced if the `ReducedWeapon` contains duplicate target sizes.
    TargetError
}
