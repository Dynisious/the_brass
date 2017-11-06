//! `fleets` defines groups of ships.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/22

use game::*;
use game::factions::Faction;
use super::ships::Ship;
use super::ships::weapons::*;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Eq, Clone)]
/// An `AllignedFleet` is a `Fleet` with a `Faction` value.
pub struct AllignedFleet {
    /// The `Fleet` in of this `AllignedFleet`.
    fleet: Fleet,
    /// The `Faction` in of this `AllignedFleet`.
    faction: Faction
}

impl AllignedFleet {
    /// Creates a new `AllignedFleet`.
    ///
    /// #Params
    ///
    /// fleet --- The `Fleet` in of this `AllignedFleet`.
    /// faction --- The `Faction` in of this `AllignedFleet`.
    pub fn new(fleet: Fleet, faction: Faction) -> Self {
        Self {
            fleet,
            faction
        }
    }
    /// Resolves the passed attacks against this `AllignedFleet`.
    ///
    /// #Params
    ///
    /// attacks --- The attacks to be resolved.
    pub fn resolve_attacks(&mut self, attacks: &mut Iterator<Item = &mut TargetedDamage>) {
        self.fleet.resolve_attacks(attacks);
    }
}

#[derive(Debug, Eq, Clone)]
/// A `Fleet` is a collection of `ShipGroup`s. Attacks are resolved on each group in order,
/// hence groups further to the back are safer.
pub struct Fleet {
    /// The `ShipGroup`s in this `Fleet`.
    groups: Vec<ShipGroup>
}

impl Fleet {
    /// Creates a new `Fleet`.
    ///
    /// #Params
    ///
    /// groups --- The `ShipGroup`s in this `Fleet`.
    pub fn new(groups: Vec<ShipGroup>) -> Self {
        Self {
            groups
        }
    }
    /// Resolves the passed attacks against the `ShipGroup`s in this `Fleet`, resolving
    /// attacks on groups in their order in the `Fleet`.
    ///
    /// #Params
    ///
    /// attacks --- The attacks to be resolved.
    pub fn resolve_attacks(&mut self, attacks: &mut Iterator<Item = &mut TargetedDamage>) {
        self.groups.iter_mut().filter(
            |item| item.types_count() != 0
        ).for_each(
            |group| group.resolve_attacks(attacks)
        )
    }
}

impl PartialEq for Fleet {
    fn eq(&self, other: &Self) -> bool {
        self.groups.iter().all(
            |current| other.groups.iter().any(
                |compare| current == compare
            )
        )
    }
}

impl Borrow<Vec<ShipGroup>> for Fleet {
    fn borrow(&self) -> &Vec<ShipGroup> {
        &self.groups
    }
}

impl AsRef<Vec<ShipGroup>> for Fleet {
    fn as_ref(&self) -> &Vec<ShipGroup> {
        &self.groups
    }
}

#[derive(Debug, Eq, Clone)]
/// A `ShipGroup` represents a collection of many ships of different types.
pub struct ShipGroup {
    /// The `Vec` of different `RepeatedShip`s in this `ShipGroup`.
    ships: Vec<RepeatedShip>
}

impl ShipGroup {
    /// Creates a `ShipGroup` from parts without guarrentees.
    ///
    /// #Params
    ///
    /// ships --- The ships which make up this `ShipGroup`.
    pub unsafe fn from_parts(ships: Vec<RepeatedShip>) -> Self {
        Self {
            ships
        }
    }
    /// Creates a `ShipGroup` with guarrentees that:
    /// * There will be no `RepeatedShip`s with no ships.
    /// * All `RepeatedShip`s using the same template will be merged.
    ///
    /// #Params
    ///
    /// ships --- The ships which make up this `ShipGroup`.
    pub fn new(mut ships: Vec<RepeatedShip>) -> Self {
        //The index of the current `RepeatedShip`.
        let mut index = 0;
        loop {
            //...All `RepeatedShip`s have been checked.
            if index >= ships.len() {
                break;
            }
            
            //If `counter` is 0, there is no ships and this can be removed.
            if ships[index].counter == 0 {
                ships.swap_remove(index);
            //If there are ships make sure there are no other `RepeatedShip`s using the same template.
            } else {
                //If index of the `RepeatedShip` being compared with.
                //Any indexes less than this one will have already compared themselves
                //against this index.
                let mut compare = index + 1;
                loop {
                    //...All `RepeatedShip`s have been compared with.
                    if compare >= ships.len() {
                        break;
                    }
                    
                    //If the two `RepeatedShip`s have the same template, merge the `RepeatedShip`s.
                    if ships[index].same_template(&ships[compare]) {
                        let temp = ships.swap_remove(compare);
                        ships[index].merge(temp);
                    //If the two `RepeatedShip`s do not share their template, compare the next index.
                    } else {
                        compare += 1;
                    }
                }
                
                index += 1;
            }
        }
        
        unsafe {
            Self::from_parts(ships)
        }
    }
    /// Add the passed ships to this `ShipGroup`.
    ///
    /// #Params
    ///
    /// ships --- The ships to be added to the `ShipGroup`.
    pub fn add_ships(&mut self, ships: RepeatedShip) {
        //If there are ships in `ships`, they need to be added.
        if ships.counter != 0 {
            //If a `RepeatedShip` shares the same template, merge `ships` into it.
            if let Some(item) = self.ships.iter_mut().find(|item| ships.same_template(item)) {
                item.merge(ships);
                return;
            }
            //If `find` failed then add `ships` as a new `RepeatedShip`.
            self.ships.push(ships);
        }
    }
    /// Resolves the passed attacks against this `ShipGroup`, distributing the damage
    /// from each attack among all valid targets.
    ///
    /// #Params
    ///
    /// attacks --- The attacks to be resolved.
    pub fn resolve_attacks(&mut self, attacks: &mut Iterator<Item = &mut TargetedDamage>) {
        //Distribute each of the attacks.
        attacks.for_each(
            |attack| {
                //The number of types which can be targeted by `attack` in `self.ships`.
                let mut valid_types = self.ships.iter().fold(0,
                    |sum, ship| if ship.ship_average.template.is_valid_for(attack) {
                        sum + 1
                    } else {
                        sum
                    }
                );
                
                //Only attempt to distribute attacks if there are possilbe targets.
                if valid_types != 0 {
                    //The index of the current `RepeatedShip` being attacked.
                    let mut index = 0;
                    loop {
                        //...All valid types have been attacked or there is no more damage to resolve.
                        if valid_types != 0 || attack.damage == 0 {
                            break;
                        }
                        
                        //If this ship type is a valid target for `attack`.
                        if self.ships[index].ship_average.template.is_valid_for(attack) {
                            //Take a portion of the attacks damage to use.
                            let passed_damage = attack.damage / valid_types;
                            //Decrement the valid targets.
                            valid_types -= 1;
                            //Remove the damage portion from the damage pool.
                            attack.damage -= passed_damage;
                            //Add the remaining damage back into the damage pool.
                            attack.damage += self.ships[index].resolve_damage(passed_damage);
                            
                            //If there are no ships left, remove the `RepeatedShip`.
                            if self.ships[index].counter == 0 {
                                self.ships.swap_remove(index);
                            }
                        //If this type is not a valid target, move to the next `RepeatedShip`.
                        } else {
                            index += 1;
                        }
                    }
                }
            }
        );
    }
    /// Returns the number of ship types in this `ShipGroup`.
    pub fn types_count(&self) -> usize {
        self.ships.len()
    }
}

impl PartialEq for ShipGroup {
    fn eq(&self, other: &Self) -> bool {
        self.ships.iter().all(
            |current| other.ships.iter().any(
                |compare| current == compare
            )
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A `RepeatedShip` represents multiple instances of a type of ship by averaging all of
/// their state to a single instance of the ship type.
pub struct RepeatedShip {
    /// A single `Ship` instance to represent the average state of all the ships.
    pub ship_average: Ship,
    /// The number of ships represented by this `RepeatedShip`.
    counter: UInt
}

impl RepeatedShip {
    /// Creates a new `RepeatedShip`.
    ///
    /// #Params
    ///
    /// self.ship_average --- A single `Ship` instance to represent the average state of all the ships.
    /// counter --- The number of ships represented by this `RepeatedShip`.
    pub fn new(ship_average: Ship, counter: UInt) -> Self {
        Self {
            ship_average,
            counter
        }
    }
    /// Distributes attacks from a `ReducedWeapon` across `targets` for all instances of
    /// the ships represented by this `RepeatedShip` in the optimal way.
    ///
    /// #Params
    ///
    /// weapons --- The `ReducedWeapon` which is distributing attacks.
    /// targets --- The targets to distribute_attacks across.
    fn distribute_attacks(&self, weapons: &ReducedWeapon, targets: &mut Vec<DistributedDamage>) {
        let mut damage_distributed = targets.iter().map(|item| {
            let mut item = item.clone();
            item.damage.damage = 0;
            item
        }).collect();
        
        weapons.distribute_attacks(&mut damage_distributed);
        
        for item in damage_distributed.iter_mut() {
            item.damage.damage *= self.counter;
        }
        for (target_damage, new_damage) in targets.iter_mut().zip(damage_distributed.iter_mut()) {
            target_damage.damage.damage += new_damage.damage.damage;
        }
    }
    /// Distributes offencive attacks across the passed targets.
    /// See `RepeatedShip::distribute_attacks` for details.
    ///
    /// #Params
    ///
    /// targets --- The targets to distribute_attacks across.
    pub fn distribute_offense(&self, targets: &mut Vec<DistributedDamage>) {
        self.distribute_attacks(&self.ship_average.template.offence_weapons, targets)
    }
    /// Distributes defencive attacks across the passed targets.
    /// See `RepeatedShip::distribute_attacks` for details.
    ///
    /// #Params
    ///
    /// targets --- The targets to distribute_attacks across.
    pub fn distribute_defence(&self, targets: &mut Vec<DistributedDamage>) {
        self.distribute_attacks(&self.ship_average.template.defence_weapons, targets)
    }
    /// Resolves damage dealt against the ships represented by this `RepeatedShip`.
    /// See `Ship::simulate_damage` for details.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against the ships.
    pub fn resolve_damage(&mut self, mut damage: DamagePoint) -> DamagePoint {
        //The total ammount of `ShieldPoint`s remaining from all of the ships which were
        //dealt damage.
        let mut remaining_shield_points = 0;
        //The total ammount of `HullPoint`s remaining from all of the ships which were
        //dealt damage.
        let mut remaining_hull_points = 0;
        //The number of ships which are yet to be dealt damage in this attack.
        let mut undamaged = self.counter;
        //The number of ships which have been damaged in this attack but have survived.
        let mut still_alive = 0;
        
        loop {
            //If all ships have been dealt damage then the attack is resolved.
            if undamaged == 0 {
                break;
            //If there is no more damage to deal then the attack is resolved.
            } else if damage == 0 {
                break;
            }
            
            //The portion of the damage which is being leveled against this instance of the ship.
            let passed_damage = damage / undamaged;
            //Remove the portion being used from the damage pool.
            damage -= passed_damage;
            
            //Simulate the attack against this ships instance.
            let (shield_points, hull_points, remaining_damage) = self.ship_average.simulate_damage(passed_damage);
            
            //If the ship is dead...
            if hull_points == 0 {
                //Decrease the number of instances by one.
                self.counter -= 1;
                //Return remaining damage to the damage pool.
                damage += remaining_damage;
            //If the ship is not dead.
            } else {
                //Accumulate the remaining shield points.
                remaining_shield_points += shield_points;
                //Accumulate the remaining hull points.
                remaining_hull_points += hull_points;
                //Increment the number of surviving ships.
                still_alive += 1;
            }
            
            //Decrement the number of undamaged ships.
            undamaged -= 1;
        }
        
        //Calculate the new average shields by averaging the lost shields across all remaining instances.
        self.ship_average.shield_points += (remaining_shield_points - (still_alive * self.ship_average.shield_points)) / self.counter;
        //Calculate the new average hull by averaging the lost hull across all remaining instances.
        self.ship_average.hull_points += (remaining_hull_points - (still_alive * self.ship_average.hull_points)) / self.counter;
        
        //Return the remaining damage.
        damage
    }
    /// Returns true if the two `RepeatedShip`s instances of the same `ShipTemplate`.
    ///
    /// #Params
    ///
    /// other --- The other `RepeatedShip` to compare against.
    pub fn same_template(&self, other: &Self) -> bool {
        self.ship_average.template == other.ship_average.template
    }
    /// Merges another `RepeatedShip` into this one, adjusting `ship_average` to
    /// represent all the ships.
    ///
    /// #Params
    ///
    /// other --- The `RepeatedShip` to merge into this one.
    pub fn merge(&mut self, other: Self) {
        //The denominator to rescale the averages over the two instances to averages over
        //the merged instances.
        let denom = (self.counter + other.counter) as i64;
        
        //Calculate the new average fuel units.
        self.ship_average.fuel_units = ((self.ship_average.fuel_units as i64 * self.counter as i64 / denom)
            + (other.ship_average.fuel_units as i64 * other.counter as i64 / denom)) as UInt;
        
        //Calculate the new average hull points.
        self.ship_average.hull_points = ((self.ship_average.hull_points as i64 * self.counter as i64 / denom)
            + (other.ship_average.hull_points as i64 * other.counter as i64 / denom)) as UInt;
        
        //Calculate the new average shield points.
        self.ship_average.shield_points = ((self.ship_average.shield_points as i64 * self.counter as i64 / denom)
            + (other.ship_average.shield_points as i64 * other.counter as i64 / denom)) as UInt;
        
        self.counter += other.counter;
    }
    /// Attempts to merge another `RepeatedShip` into this one. If `other` does not share
    /// the same template then it is returned in `Some`.
    ///
    /// #Params
    ///
    /// other --- The `RepeatedShip` to merge into this one.
    pub fn checked_merge(&mut self, other: Self) -> Option<Self> {
        //If the `RepeatedShip`s shares the same template, merge the ships.
        if self.same_template(&other) {
            self.merge(other); None
        //If the `RepeatedShip`s do not share the same template, return `other`.
        } else {
            Some(other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_fleet() {
    }
}
