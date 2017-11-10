//! `reduced_ship` defines the `ReducedShip` type, its construction, modification and
//! interactions between them.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/10

use game::*;
use super::ship_template::{HullPoint, ShieldPoint};
use super::attacks::*;
use super::ship::*;

/// A `ReducedShip` represents multiple instances of a `ShipTemplate` simulated using a
/// shared average state.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReducedShip {
    /// The `Ship` instance which represents the average state of all the `Ship`s in this
    /// `ReducedShip`.
    average_ship: Ship,
    /// The number of `Ship`s in this group.
    pub number: UInt
}

impl ReducedShip {
    /// Returns a new `ReducedShip`.
    ///
    /// #Params
    ///
    /// average_ship --- A `Ship` instance which represents the average ship in this group.
    /// number --- The number of `Ship`s in this group.
    pub fn new(average_ship: Ship, number: UInt) -> Self {
        Self {
            average_ship,
            number
        }
    }
    /// Returns true if the `ReducedShip` still has `Ship`s in the group.
    pub fn is_alive(&self) -> bool {
        self.number != 0
    }
    /// Regenerates shields for this `ReducedShip`.
    pub fn regenerate_shields(&mut self) {
        self.average_ship.regenerate_shields()
    }
    /// Resolves damage dealt against this group of `Ship`s and returns any which was not
    /// used to destroy the `Ship`s.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against this `ReducedShip`.
    pub fn resolve_damage(&mut self, mut damage: DamagePoint) -> DamagePoint {
        //The total amount of remaining hull points of all the ships.
        let mut remaining_hull = 0u64;
        //The total amount of remaining shield points of all the ships.
        let mut remaining_shield = 0u64;
        
        //The number of ships left to iterate. if to_iterate was ever greater than damage
        //then `portion` would just be 0 hence the `min` call.
        let mut to_iterate = ::std::cmp::min(self.number, damage);
        //The number of ships which have not been attacked.
        let mut unattacked = self.number;
        //Iterate while there's still ships to iterate and while there's still damage to
        //resolve.
        while to_iterate > 0 && damage != 0 {
            //The portion of damage which will get used against the current ship.
            let portion = damage / to_iterate;
            //Remove the portion from the pool of damage.
            damage -= portion;
            
            //Simulate the portion being used against this ship.
            let simulation = self.average_ship.simulate_damage(portion);
            
            //Check whether the ship died (its hull is 0).
            if simulation.0 == 0 {
                //If the ship died remove a ship from this `ReducedShip`.
                self.number -= 1;
                //If the ship died there's maybe some damage left unused which will be
                //returned to the pool.
                damage += simulation.2;
            //If the ship did not die, there's hull and maybe shield remaining.
            } else {
                //Add the remaining hull to the pool.
                remaining_hull += simulation.0 as u64;
                //Add the remaining shield to the pool.
                remaining_shield += simulation.1 as u64;
            }
            
            //Decrement the number of unattacked ships.
            unattacked -= 1;            
            //Decrement the number of ships left to iterate, ensuring that it is at most
            //`damage` so that portion is non zero.
            to_iterate = ::std::cmp::min(to_iterate - 1, damage);
        }
        
        //Check whether there's any ships left alive.
        if self.is_alive() {
            //Add to the remaining hull the hull of all ships which were not attacked.
            remaining_hull += self.average_ship.get_hull_points() as u64 * unattacked as u64;
            //Add to the remaining shields the shields of all ships which were not attacked.
            remaining_shield += self.average_ship.get_shield_points() as u64 * unattacked as u64;
            
            //Calculate the new average hull.
            self.average_ship.set_hull_points((remaining_hull / self.number as u64) as HullPoint).ok();
            //Calculate the new average shields.
            self.average_ship.set_shield_points((remaining_shield / self.number as u64) as ShieldPoint).ok();
        }
        //Return the unused damage.
        damage
    }
    /// Resolves attacks leveled against this group of `Ship`s and returns any which were
    /// not used to destroy the `Ship`s.
    /// This function does not clear away used attacks in `attacks`
    ///
    /// #Params
    ///
    /// attacks --- The attacks leveled against this `ReducedShip`.
    pub fn resolve_attacks(&mut self, attacks: &mut ReducedAttacks) {
        //The size class of this `ReducedShip`.
        let size_class = (*self.as_ref()).ship_size_class;
        //The iterator over each group of targeted attacks, filtered by those which can
        //target the ships in this `ReducedShip`.
        let mut iter = attacks.iter_mut()
        .filter(|attack| attack.valid_target(size_class));
        
        //Loop while there are still ships left.
        //The loop will also exit if there's no attacks left.
        while self.is_alive() {
            match iter.next() {
                //If there's still attacks left then resolve their damage against this
                //`ReducedShip`.
                Some(attack) => {
                    //If there is still unused damage then `parralel_attacks` is set
                    //accordingly, else it's zeroed.
                    attack.attack.parralel_attacks =
                        self.resolve_damage(attack.attack.sum_damage())
                        / attack.attack.damage_per_attack;
                },
                //If there's no more attacks left then their all resolved.
                None => break
            }
        }
    }
    /// Calculates the attacks produced by all of the ships in this `ReducedShip` in
    /// parralel.
    pub fn get_attacks(&self) -> ReducedAttacks {
        let mut attacks = self.average_ship.attacks.clone();
        attacks.iter_mut().for_each(|attack| attack.attack.parralel_attacks *= self.number);
        attacks
    }
}

impl AsRef<Ship> for ReducedShip {
    fn as_ref(&self) -> &Ship {
        &self.average_ship
    }
}
