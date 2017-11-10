//! `attacks` defines the attack types, their construction, modification and interactions.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/10

use game::*;
use super::ShipSize;
use std::iter::Iterator;
use std::cmp::Ordering;

pub type DamagePoint = UInt;

/// A `TargetedAttack` is an `Attack` with a smallest size of target allowed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
pub struct TargetedAttack {
    /// The `Attack` for this `TargetedAttack`.
    pub attack: Attack,
    /// The smallest size of target this can target.
    pub smallest_target: ShipSize
}

impl TargetedAttack {
    /// Creates a new `Attack` from parts.
    ///
    /// #Params
    ///
    /// attack --- The `Attack` for this `TargetedAttack`.
    /// smallest_target --- The smallest size of target this can attack.
    pub fn new(attack: Attack, smallest_target: ShipSize) -> Self {
        Self {
            attack,
            smallest_target
        }
    }
    /// Returns true if the passed size of target is a valid target for this
    /// `TargetedAttack`.
    ///
    /// #Params
    ///
    /// target_size --- The size of the target in question.
    pub fn valid_target(&self, target_size: ShipSize) -> bool {
        self.smallest_target <= target_size
    }
    /// Returns true if the passed `TargetedAttack` has the same smallest target as this
    /// `TargetedAttack`.
    ///
    /// #Params
    ///
    /// other --- The other `TargetedAttack` to compare against.
    pub fn same_target(&self, other: &Self) -> bool {
        self.smallest_target == other.smallest_target
    }
}

impl PartialOrd for TargetedAttack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TargetedAttack {
    fn cmp(&self, other: &Self) -> Ordering {
        //Ordering is done on the smallest target.
        match self.smallest_target.cmp(&other.smallest_target) {
            //Equality is resolved by ordering damage per attack.
            Ordering::Equal => match self.attack.damage_per_attack.cmp(&other.attack.damage_per_attack) {
                //Equality is resolved by ordering parralel attacks.
                Ordering::Equal => self.attack.parralel_attacks.cmp(&other.attack.parralel_attacks),
                ord => ord
            },
            ord => ord
        }
    }
}

/// An `Attack` is a number of parralel attack projectiles with a damage per attack.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
pub struct Attack {
    /// The number of parralel attacks for this `Attack`.
    pub parralel_attacks: UInt,
    /// The damage dealt by each attack.
    pub damage_per_attack: DamagePoint
}

impl Attack {
    /// Creates a new `Attack` from parts.
    ///
    /// #Params
    ///
    /// parralel_attacks --- The number of parralel attacks for this `Attack`.
    /// damage_per_attack --- The damage dealt by each attack.
    pub fn new(parralel_attacks: UInt, damage_per_attack: DamagePoint) -> Self {
        Self {
            parralel_attacks,
            damage_per_attack
        }
    }
    /// Attempts to merge another `Attack` into this `Attack` if they deal the same
    /// amount of damage per attack else it returns ownership of `other`.
    ///
    /// #Params
    ///
    /// other --- The other `Attack` to merge into this one.
    pub fn merge(&mut self, other: Self) -> Option<Self> {
        //If they deal the same damage per attack then they can be merged...
        if self.same_damage(&other) {
            //Merging means adding the attacks from `other` into this `Attack`.
            self.parralel_attacks += other.parralel_attacks; None
        //Else they cannot be merged.
        } else {
            //Return ownership of `other`.
            Some(other)
        }
    }
    /// Sums up all the damage dealt by each of the attacks of this `Attack`.
    pub fn sum_damage(&self) -> DamagePoint {
        self.parralel_attacks * self.damage_per_attack
    }
    /// Returns true if `other` deals the same damage per attack as this `Attack`.
    pub fn same_damage(&self, other: &Self) -> bool {
        self.damage_per_attack == other.damage_per_attack
    }
}

/// A collection of `TargetedAttack`s ordered by the size of their smallest target and
/// without duplicates of smallest target.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct ReducedAttacks {
    /// The `Vec` of `TargetedAttack`s.
    attacks: Vec<TargetedAttack>
}

impl ReducedAttacks {
    /// Creates a new `ReducedAttacks` without checking for guarentees.
    ///
    /// #Params
    ///
    /// attacks --- The `Vec` of `TargetedAttack`s.
    pub unsafe fn from_parts(attacks: Vec<TargetedAttack>) -> Self {
        Self {
            attacks
        }
    }
    /// Creates a new `ReducedAttacks`, checking to guarentee that the `TargetedAttack`s
    /// are ordered by the size of their smallest target and that damage per attack is
    /// not duplicated for each target size.
    ///
    /// #Params
    ///
    /// refer to `from_parts` for parameters.
    pub fn new(mut attacks: Vec<TargetedAttack>) -> Self {
        //Sort all of the `TargetedAttack`s.
        attacks.sort_unstable();
        //Merge all attacks which attack the same smallest target with the same damage
        //per target to remove them.
        //Check that the two attacks share the same smallest target.
        attacks.dedup_by(|prev, attack| if prev.same_target(attack) {
            //If they share the same smallest target and can be merged then remove
            //`attack`.
            prev.attack.merge(attack.attack) == None
        //If the two attacks do not share the same smallest target then `attack` should be kept.
        } else {
            false
        });
        //Deallocate unused memory.
        attacks.shrink_to_fit();
        
        unsafe {
            Self::from_parts(attacks)
        }
    }
    /// Add a `TargetedAttack` to this `ReducedAttacks`.
    ///
    /// #Params
    ///
    /// attack --- The `TargetedAttack` to add to this `TargetedAttack`.
    pub fn add_attack(&mut self, attack: TargetedAttack) {
        //Search for an existing `TargetedAttack` with the same smallest target and damage per attack...
        match self.attacks.binary_search_by(|existing| match existing.smallest_target.cmp(&attack.smallest_target) {
                Ordering::Equal => existing.attack.damage_per_attack.cmp(&attack.attack.damage_per_attack),
                ord => ord
            }) {
            //If a `TargetedAttack` exists then simply add `attack`s attacks too it...
            Ok(index) => { self.attacks[index].attack.parralel_attacks += attack.attack.parralel_attacks; },
            //Otherwise insert `attack` as a new `TargetedAttack` in this `ReducedAttacks`.
            Err(index) => self.attacks.insert(index, attack)
        }
    }
    /// Adds all the `TargetedAttack`s in `attacks` into this `ReducedAttacks`.
    ///
    /// #Params
    ///
    /// attacks --- The `TargetedAttack`s to add to this `ReducedAttacks`.
    pub fn add_attacks(&mut self, attacks: &[TargetedAttack]) {
        attacks.iter().for_each(|attack| self.add_attack(*attack));
    }
    /// Returns an iterator over the `TargetedAttack`s of this `ReducedAttacks`.
    pub fn iter(&self) -> ::std::slice::Iter<TargetedAttack> {
        self.attacks.iter()
    }
    /// Returns a mutable iterator over the `TargetedAttack`s of this `ReducedAttacks`.
    pub fn iter_mut(&mut self) -> ::std::slice::IterMut<TargetedAttack> {
        self.attacks.iter_mut()
    }
    /// Removes all of the `TargetedAttack`s which have no parralel attacks.
    pub fn clear_used_attacks(&mut self) {
        self.attacks.retain(|attack| attack.attack.parralel_attacks != 0);
    }
}
