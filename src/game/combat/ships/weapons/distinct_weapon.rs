//! `distinct_weapon` defines the `DistinctWeapon` type, it's construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

pub use super::*;
pub use super::super::ShipSize;
use game::properties::*;
use game::file_system::{self, FileInterface, Path};

const NAME_STR: &'static str = "name";
const TARGET_SIZE_STR: &'static str = "target_size";
const DAMAGE_PER_ATTACK_STR: &'static str = "damage_per_attack";
const SIMULTAINIOUS_ATTACKS_STR: &'static str = "simultainious_attacks";
const FIELD_COUNT: usize = 3;

/// A `DistinctWeapon` represents a single weapon: it's intended target, the damage per
/// attack and the number of attacks produced at once.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DistinctWeapon {
    /// The minimum size of target which this `DistinctWeapon` can attack.
    pub target_size: ShipSize,
    /// The points of damage each attack represents.
    pub damage_per_attack: DamagePoint,
    /// The number of attacks produced at once.
    pub simultainious_attacks: AttackProjectile
}

impl DistinctWeapon {
    /// Returns a new `DistinctWeapon` without checks.
    ///
    /// #Params
    ///
    /// target_size --- The smallest target size this `DistinctWeapon` can target.
    /// damage_per_attack --- The points of damage each attack represents.
    /// simultainious_attacks --- The number of attacks produced at once.
    pub unsafe fn from_parts(target_size: UInt, damage_per_attack: UInt,
        simultainious_attacks: UInt) -> Self {
        DistinctWeapon {
            target_size,
            damage_per_attack,
            simultainious_attacks
        }
    }
    /// Returns a new `DistinctWeapon`, checking inputs to guarrentee:
    ///  * Does not produce 0 attacks.
    ///  * Does not do 0 damage.
    ///
    /// #Params
    ///
    /// target_size --- The smallest target size this `DistinctWeapon` can target.
    /// damage_per_attack --- The points of damage each attack represents.
    /// simultainious_attacks --- The number of attacks produced at once.
    pub fn new(target_size: UInt, damage_per_attack: UInt,
        simultainious_attacks: UInt) -> Result<Self, WeaponError> {
        if damage_per_attack == 0 {
            return Err(
                WeaponError::DamageError
            )
        } else if simultainious_attacks == 0 {
            return Err(
                WeaponError::AttacksError
            )
        } else {
            Ok(
                unsafe {
                    DistinctWeapon::from_parts(
                        target_size,
                        damage_per_attack,
                        simultainious_attacks
                    )
                }
            )
        }
    }
    /// Attempts to make a call to `DistinctWeapon::new` using the passed properties as
    /// parameters, erroring if any of the parameters either do not exist as properties
    /// or parse incorrectly.
    ///
    /// #Params
    ///
    /// props --- The properties to use as parameters for the constructor call.
    pub fn from_properties(props: &PropertyPairs) -> Result<Self, WeaponError> {
        Self::new(
            str::parse(
                props.get(TARGET_SIZE_STR)
                    .ok_or(WeaponError::TargetError)?
            ).ok().ok_or(WeaponError::TargetError)?,
            str::parse(
                props.get(DAMAGE_PER_ATTACK_STR)
                    .ok_or(WeaponError::TargetError)?
            ).ok().ok_or(WeaponError::TargetError)?,
            str::parse(
                props.get(SIMULTAINIOUS_ATTACKS_STR)
                    .ok_or(WeaponError::TargetError)?
            ).ok().ok_or(WeaponError::TargetError)?
        )
    }
    /// Creates a `Properties` from this `DistinctWeapon`.
    pub fn into_properties(&self) -> Properties {
        let mut props = PropertyPairs::with_capacity(FIELD_COUNT);
        
        props.insert(
            String::from(TARGET_SIZE_STR),
            self.target_size.to_string()
        );
        props.insert(
            String::from(DAMAGE_PER_ATTACK_STR),
            self.damage_per_attack.to_string()
        );
        props.insert(
            String::from(SIMULTAINIOUS_ATTACKS_STR),
            self.simultainious_attacks.to_string()
        );
        
        Properties::from(props)
    }
    /// Decides whether the two `DistinctWeapon` instances target the same `ShipSize`.
    ///
    /// #Params
    ///
    /// left --- The first instance of `DistinctWeapon`.
    /// right --- The first instance of `DistinctWeapon`.
    pub fn same_target(left: &Self, right: &Self) -> bool {
        left.target_size == right.target_size
    }
    /// Folds `other` into the stats for this `DistinctWeapon` so that this
    /// `DistinctWeapon` represents both this and `other` combined.
    ///
    /// #Params
    ///
    /// other --- The other `DistinctWeapon` to fold into this one.
    pub fn fold(&mut self, other: &Self) -> Option<&Self> {
        if self.target_size != other.target_size {
            None
        } else {
            //Use `damage_per_attack` to store the total damage done at once.
            self.damage_per_attack *= self.simultainious_attacks;
            //Add the attacks done by the other `DistinctWeapon` to this one.
            self.simultainious_attacks += other.simultainious_attacks;
            //Set `damage_per_attack` to be the average damage per attack of both this
            //`DistinctWeapon` and `other`.
            self.damage_per_attack = (
                other.simultainious_attacks * other.damage_per_attack
                + self.damage_per_attack
            ) / self.simultainious_attacks;
            Some(
                self
            )
        }
    }
    /// Distributes and adds the attacks and damage from this `DistinctWeapon` among the
    /// possible targets in `targets`.
    ///
    /// #Params
    ///
    /// targets --- A `Vec` of tuples representing the points of damage being done
    /// against a particular size of ship and the number of targets of that size being
    /// attacked.
    pub fn distribute_attacks(&self, targets: &mut Vec<DistributedDamage>) {
        let mut total_targets = targets.iter().fold(0,
            |sum, ref damage| {
                if damage.damage.target_size >= self.target_size {
                    sum + damage.targets
                } else {
                    sum
                }
            }
        );
        
        if total_targets != 0 {
            let mut attacks_to_distribute = self.simultainious_attacks;
            let mut iter = targets.iter_mut();

            loop {
                let ref mut damage = match iter.next() {
                    Some(e) => e,
                    None => break
                };
                
                if damage.damage.target_size >= self.target_size {
                    let attacks = attacks_to_distribute / total_targets * damage.targets;
                    
                    damage.damage.damage += self.damage_per_attack * attacks;
                    attacks_to_distribute -= attacks;
                    total_targets -= damage.targets;
                    
                    if attacks_to_distribute == 0 {
                        break;
                    }
                }
            }
        }
    }
}

impl FileInterface for DistinctWeapon {
    type Output = (String, Self);
    type Error = Error;
    
    fn from_file(file_path: &Path) -> Result<Self::Output, Self::Error> {
        let props = Properties::from_file(file_path)?.props;
        
        let name = String::from(
            file_path.file_stem().unwrap().to_str().unwrap()
        );
        let weapon = Self::from_properties(&props)?;
        Ok((name, weapon))
    }
    fn write_file(&self, file_path: &Path) -> Result<(), Self::Error> {
        self.into_properties().write_file(file_path).map_err(Error::FileError)
    }
}

/// An error returned when calling functions from `FileInterface`.
pub enum Error {
    /// Indicates an error occoured while creating a `DistinctWeapon` from `Properties`.
    WeaponError(WeaponError),
    /// Indicates an error while reading or writing to a file.
    FileError(file_system::Error)
}

impl From<file_system::Error> for Error {
    fn from(error: file_system::Error) -> Error {
        Error::FileError(error)
    }
}

impl From<WeaponError> for Error {
    fn from(error: WeaponError) -> Error {
        Error::WeaponError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distinct_weapon() {
        assert!(
            DistinctWeapon::new(0, 0, 0).err().unwrap() == WeaponError::DamageError,
            "`test_distinct_weapon` Failed to error on `damage_per_attack` while constructing `DistinctWeapon`."
        );
        
        assert!(
            DistinctWeapon::new(1, 1, 0).err().unwrap() == WeaponError::AttacksError,
            "`test_distinct_weapon` Failed to error on `simultainious_attacks` while constructing `DistinctWeapon`."
        );
        
        let mut weapon = DistinctWeapon::new(2, 2, 3).expect(
            "`test_distinct_weapon` Failed to construct `DistinctWeapon` on valid parts."
        );
        
        assert!(
            weapon == unsafe {
                DistinctWeapon::from_parts(2, 2, 3)
            },
            "`test_distinct_weapon` Failed to construct `DistinctWeapon`."
        );
        
        weapon.fold(
            unsafe {
                &DistinctWeapon::from_parts(2, 3, 2)
            }
        );
        assert!(
            weapon == unsafe {
                DistinctWeapon::from_parts(2, 2, 5)
            },
            "`test_distinct_weapon` Failed to fold `DistinctWeapon`."
        );
        
        let mut targets = Vec::<DistributedDamage>::with_capacity(3);
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(1, 0),
                3
            )
        );
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(2, 0),
                2
            )
        );
        targets.push(
            DistributedDamage::new(
                TargetedDamage::new(3, 0),
                1
            )
        );
        weapon.distribute_attacks(&mut targets);
        assert!(
            targets == [
                DistributedDamage::new(
                    TargetedDamage::new(1, 0),
                    3
                ),
                DistributedDamage::new(
                    TargetedDamage::new(2, 4),
                    2
                ),
                DistributedDamage::new(
                    TargetedDamage::new(3, 6),
                    1
                )
            ],
            "`test_distinct_weapon` Failed to distribute attacks."
        );
        
        let props = weapon.into_properties().props;
        for &(key, value) in [(TARGET_SIZE_STR, "2"), (DAMAGE_PER_ATTACK_STR, "2"), (SIMULTAINIOUS_ATTACKS_STR, "5")].iter() {
            assert!(
                Some(value) == props.get(key).map(
                    |s| s.as_str()
                ),
                "`test_distinct_weapon` Failed to create `Properties` from `DistinctWeapon`."
            )
        }
        
        assert!(
            Ok(weapon) == DistinctWeapon::from_properties(&props),
            "`test_distinct_weapon` Failed to create `DistinctWeapon` from `Properties`."
        );
    }
}
