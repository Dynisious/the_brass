//! `reduced_ship` defines the `ReducedShip` type, its construction, modification and
//! interactions between them.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/07

pub use game::*;
pub use super::ship_template::{DamagePoint, HullPoint, ShieldPoint};
pub use super::ship::*;

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
    /// Resolves damage dealt against this group of `Ship`s and returns any which was not
    /// used to destroy the `Ship`s.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against this `ReducedShip`.
    pub fn resolve_damage(&mut self, mut damage: DamagePoint) -> DamagePoint {
        let mut total_hull = 0u64;
        let mut total_shield = 0u64;
        
        let mut iterated = self.number;
        let mut unattacked = self.number;
        while iterated > 0 && damage != 0 {
            let portion = damage / iterated;
            damage -= portion;
            
            if portion != 0 {
                let simulation = self.average_ship.simulate_damage(portion);
                
                if simulation.0 == 0 {
                    self.number -= 1;
                } else {
                    total_hull += simulation.0 as u64;
                }
                
                total_shield += simulation.1 as u64;
                damage += simulation.2;
                unattacked -= 1;
            }
            
            iterated -= 1;
        }
        
        total_hull += self.average_ship.get_hull_points() as u64 * unattacked as u64;
        total_shield += self.average_ship.get_shield_points() as u64 * unattacked as u64;
        
        if self.is_alive() {
            self.average_ship.set_hull_points((total_hull / self.number as u64) as HullPoint).ok();
            self.average_ship.set_shield_points((total_shield / self.number as u64) as ShieldPoint).ok();
        }
        damage
    }
    /// Regenerates shields for this `ReducedShip`.
    pub fn regenerate_shields(&mut self) {
        self.average_ship.regenerate_shields()
    }
}

impl AsRef<Ship> for ReducedShip {
    fn as_ref(&self) -> &Ship {
        &self.average_ship
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
}
