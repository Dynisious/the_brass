//! `distributed_damage` defines the `DistributedDamage` type, it's construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

pub use super::*;

/// `DistributedDamage` represents `DamagePoint`s distributed across several targets of the
/// same `ShipSize`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DistributedDamage {
    /// The `TargetedDamage` distributed across the targets.
    pub damage: TargetedDamage,
    /// The number of ships being targeted by the damage.
    pub targets: UInt
}

impl DistributedDamage {
    /// Creates a new `DistributedDamage` instance.
    ///
    /// #Params
    ///
    /// damage --- The `TargetedDamage` distributed across the targets.
    /// targets --- The ships being targeted by the damage.
    pub fn new(damage: TargetedDamage, targets: UInt) -> Self {
        DistributedDamage {
            damage,
            targets
        }
    }
}
