//! `distinct_weapon` defines the `DistinctWeapon` type, it's construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

pub use super::*;

/// A `ReducedWeapon` represents a collection of `DistinctWeapon`s acting together,
/// summising all of there damage and attacks against different target sizes.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReducedWeapon {
    /// The vector of `DistinctWeapon`s which sum up all the damage and attacks produced
    /// by a collection of `DistinctWeapon`s.
    weapons: Vec<DistinctWeapon>
}

impl ReducedWeapon {
    /// Returns a new `ReducedWeapon` without checks.
    ///
    /// weapons --- The summerised `DistinctWeapon`s to use.
    pub unsafe fn from_parts(weapons: Vec<DistinctWeapon>) -> Self {
        ReducedWeapon {
            weapons
        }
    }
    /// Returns a new `ReducedWeapon`, checking the summerised `DistinctWeapon`s to guarrentee:
    ///  * At most one `DistinctWeapon` targets any particular target size.
    ///  * No `DistinctWeapon` produces 0 attacks.
    ///  * No `DistinctWeapon` does 0 damage.
    ///
    /// #Params
    ///
    /// weapons --- The summerised `DistinctWeapon`s to use.
    pub fn new(mut weapons: Vec<DistinctWeapon>) -> Result<Self, WeaponError> {
        for i in 0..weapons.len() {
            for j in (i + 1)..weapons.len() {
                if DistinctWeapon::same_target(&weapons[i], &weapons[j]) {
                    return Err(
                        WeaponError::TargetError
                    )
                } else if weapons[j].damage_per_attack == 0 {
                    return Err(
                        WeaponError::DamageError
                    )
                } else if weapons[j].simultainious_attacks == 0 {
                    return Err(
                        WeaponError::AttacksError
                    )
                }
            }
        }
        weapons.shrink_to_fit();
        Ok(
            unsafe {
                ReducedWeapon::from_parts(
                    weapons
                )
            }
        )
    }
    /// Folds `other` into the stats for this `ReducedWeapon` so that this
    /// `ReducedWeapon` represents both this and `other` combined.
    ///
    /// #Params
    ///
    /// other --- The other `DistinctWeapon` to fold into this one.
    pub fn fold(&mut self, other: &Vec<DistinctWeapon>) {
        for weapon in other.iter() {
            self.add(weapon);
        }
    }
    /// Adds `other` to this `ReducedWeapon` so that this `ReducedWeapon` represents both
    /// this and `other` combined.
    ///
    /// #Params
    ///
    /// other --- The other `DistinctWeapon` to fold into this one.
    pub fn add(&mut self, other_weapon: &DistinctWeapon) {
        for self_weapon in self.weapons.iter_mut() {
            if DistinctWeapon::same_target(self_weapon, other_weapon) {
                self_weapon.fold(other_weapon);
                return;
            }
        }
        self.weapons.push(other_weapon.clone());
    }
    /// Distributes and adds the attacks and damage from this `ReducedWeapon` among the
    /// possible targets in `targets`.
    ///
    /// #Params
    ///
    /// targets --- A `Vec` of tuples representing the points of damage being done
    /// against a particular size of ship and the number of targets of that size being
    /// attacked.
    pub fn distribute_attacks(&self, targets: &mut Vec<DistributedDamage>) {
        for weapon in self.weapons.iter() {
            weapon.distribute_attacks(targets)
        }
    }
}

impl Default for ReducedWeapon {
    fn default() -> Self {
        unsafe {
            Self::from_parts(
                Vec::with_capacity(0)
            )
        }
    }
}

impl AsRef<Vec<DistinctWeapon>> for ReducedWeapon {
    fn as_ref(&self) -> &Vec<DistinctWeapon> {
        &self.weapons
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reduced_weapon() {
        let mut weapons = unsafe {
            ReducedWeapon::from_parts(
                [
                    DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(1, 2, 3)
                ].to_vec()
            )
        };
        
        unsafe {
            assert!(
                ReducedWeapon::new(
                    [DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(0, 2, 3)].to_vec()
                ).err().expect(
                    "`test_reduced_weapon` Failed to error on bad target size."
                ) == WeaponError::TargetError,
                "`test_reduced_weapon` Failed to raise `TargetError`."
            )
        };
        
        unsafe {
            assert!(
                ReducedWeapon::new(
                    [DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(1, 0, 3)].to_vec()
                ).err().expect(
                    "`test_reduced_weapon` Failed to error on bad damage."
                ) == WeaponError::DamageError,
                "`test_reduced_weapon` Failed to raise `DamageError`."
            )
        };
        
        unsafe {
            assert!(
                ReducedWeapon::new(
                    [DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(1, 2, 0)].to_vec()
                ).err().expect(
                    "`test_reduced_weapon` Failed to error on bad attacks."
                ) == WeaponError::AttacksError,
                "`test_reduced_weapon` Failed to raise `AttacksError`."
            )
        };
        
        unsafe {
            assert!(
                weapons == ReducedWeapon::new(
                    [DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(1, 2, 3)].to_vec()
                ).expect(
                    "`test_reduced_weapon` Failed on good input."
                ),
                "`test_reduced_weapon` Failed to construct `ReducedWeapon` on good input."
            )
        };
        
        unsafe {
            weapons.add(
                &DistinctWeapon::from_parts(0, 1, 2)
            );
            assert!(
                weapons == ReducedWeapon::from_parts(
                    [
                        DistinctWeapon::from_parts(0, 1, 4),
                        DistinctWeapon::from_parts(1, 2, 3)
                    ].to_vec()
                ),
                "`test_reduced_weapon` Failed to add `DistinctWeapon`."
            );
        }
        
        unsafe {
            weapons.fold(
                &[
                    DistinctWeapon::from_parts(0, 1, 2),
                    DistinctWeapon::from_parts(1, 2, 3)
                ].to_vec()
            );
            assert!(
                weapons == ReducedWeapon::from_parts(
                    [
                        DistinctWeapon::from_parts(0, 1, 6),
                        DistinctWeapon::from_parts(1, 2, 6)
                    ].to_vec()
                ),
                "`test_reduced_weapon` Failed to fold into `ReducedWeapon`."
            );
        }
        
        let mut targets = Vec::<DistributedDamage>::with_capacity(3);
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(0, 0),
                3
            )
        );
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(1, 0),
                2
            )
        );
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(2, 0),
                1
            )
        );
        weapons.distribute_attacks(&mut targets);
        assert!(
            targets == [
                DistributedDamage::new(
                    TargetedDamage::new(0, 3),
                    3
                ),
                DistributedDamage::new(
                    TargetedDamage::new(1, 10),
                    2
                ),
                DistributedDamage::new(
                    TargetedDamage::new(2, 5),
                    1
                )
            ],
            "`test_reduced_weapon` Failed to distribute attacks."
        );
    }
}
