//! `targeted_damage` defines the `TargetedDamage` type, it's construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

pub use super::*;
pub use super::super::ShipSize;

/// A `TargetedDamage` represents damage which is being targeted at a size of ship.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TargetedDamage {
    pub target_size: ShipSize,
    pub damage: DamagePoint
}

/// A `WeaponError` is an error produced if a call to `ReducedShip::new` fails.
impl TargetedDamage {
    /// Creates a new `TargetedDamage` instance.
    ///
    /// #Params
    ///
    /// target_size --- The specific size of ship being targeted.
    /// damage --- The `DamagePoint`s being directed.
    pub fn new(target_size: ShipSize, damage: DamagePoint) -> Self {
        TargetedDamage {
            target_size,
            damage
        }
    }
    /// Decides whether the two `TargetedDamage` instances target the same `ShipSize`.
    ///
    /// #Params
    ///
    /// left --- The first instance of `TargetedDamage`.
    /// right --- The first instance of `TargetedDamage`.
    pub fn same_target(left: &Self, right: &Self) -> bool {
        left.target_size == right.target_size
    }
    /// Attempts to add the damage from `other` to the damage of this `TargetedDamage`.</br>
    /// It will panic if they do not have the same target.
    ///
    /// #Params
    ///
    /// other --- The other `TargetedDamage` to add to this one.
    pub fn fold(&mut self, other: &Self) {
        if Self::same_target(self, other) {
            self.damage += other.damage;
        } else {
            panic!(
                "`TargetedDamage::fold` Failed target sizes are not equal {} != {}.",
                self.target_size.clone(),
                other.target_size
            )
        }
    }
    /// Returns the result of attempting to add the damage from `other` to the damage of
    /// this `TargetedDamage`.
    ///
    /// #Params
    ///
    /// other --- The other `TargetedDamage` to add to this one.
    pub fn add(&self, other: &Self) -> Option<Self> {
        if Self::same_target(self, other) {
            Some(
                TargetedDamage {
                    damage: self.damage + other.damage,
                    ..*self
                }
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_targeted_damage() {
        let mut damage = TargetedDamage::new(0, 10);
        
        assert!(
            TargetedDamage::same_target(&damage, &damage.clone()),
            "`test_targeted_damage` Failed to compare same targets."
        );
        
        assert!(
            !TargetedDamage::same_target(&damage, &TargetedDamage::new(1, 0)),
            "`test_targeted_damage` Failed to compare different targets."
        );
        
        assert!(
            damage.add(&TargetedDamage::new(1, 10)).is_none(),
            "`test_targeted_damage` Failed to error on bad target."
        );
        
        assert!(
            damage.add(&TargetedDamage::new(0, 10)).expect(
                "`test_targeted_damage` Failed to add values on good input."
            ) == TargetedDamage::new(0, 20),
            "`test_targeted_damage` Failed to add values."
        );
        
        damage.fold(&TargetedDamage::new(0, 10));
        assert!(
            damage == TargetedDamage::new(0, 20),
            "`test_targeted_damage` Failed to add values."
        );
    }
}
