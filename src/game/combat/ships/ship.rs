//! `ship` defines `Ship`, its construction, modification and interactions between `Ship`s.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/10

use game::*;
use super::ship_error::*;
use super::ship_template::*;
use super::attacks::*;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone)]
/// `ShipTemplate` is a representation of a type of Ship.
pub struct Ship {
    /// The `ShipTemplate` this `Ship` is instanciating.
    template: Rc<ShipTemplate>,
    /// The current fuel of this Ship type.
    fuel_units: FuelUnit,
    /// The current hull points (health points) of this Ship type.
    hull_points: HullPoint,
    /// The current shield points of this Ship type.
    shield_points: ShieldPoint
}

impl Ship {
    /// Returns a new `Ship` built from raw parts.
    ///
    /// #Params
    ///
    /// template --- The `ShipTemplate` this `Ship` is instanciating.
    /// hull_points --- The current hull points (health points) of this Ship type.
    /// shield_points --- The current shield points of this Ship type.
    pub unsafe fn from_parts(template: Rc<ShipTemplate>, fuel_units: FuelUnit,
        hull_points: HullPoint, shield_points: ShieldPoint) -> Self {
        Self {
            template,
            fuel_units,
            hull_points,
            shield_points
        }
    }
    /// Attempts to call `Ship::from_parts` if parameters pass checks.
    ///
    /// #Params
    ///
    /// Refer to `Ship::from_parts` for parameters.
    ///
    /// #Errors
    ///
    /// HullError --- hull_points > template.max_hull
    pub fn new(template: Rc<ShipTemplate>, fuel_units: FuelUnit,
        hull_points: HullPoint, shield_points: ShieldPoint) -> Result<Self, ShipError> {
        //Check that fuel is not greater than fuel capacity.
        if fuel_units > template.get_fuel_capacity() {
            Err(FuelError)
        //Check that hull is not greater than max hull.
        } else if hull_points > template.max_hull {
            Err(HullError)
        //Check that shield is not greater than shield capacity.
        } else if shield_points > template.get_shield_capacity() {
            Err(ShieldError)
        //All checks passed, parameters are valid.
        } else {
            Ok(
                unsafe {
                    Ship::from_parts(
                        template,
                        fuel_units,
                        hull_points,
                        shield_points
                    )
                }
            )
        }
    }
    /// Attempts to set the `template` of this `Ship` if it is a valid template according
    /// to `Ship::new`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// FuelError --- self.fuel_units > val.max_hull
    /// HullError --- self.hull > val.max_hull
    /// HullError --- self.shield_points > val.shield_capacity
    pub fn set_template(&mut self, val: Rc<ShipTemplate>) -> Result<(), ShipError> {
        //Check that fuel is not greater than fuel capacity.
        if self.fuel_units > val.get_fuel_capacity() {
            Err(FuelError)
        //Check that hull is not greater than max hull.
        } else if self.hull_points > val.max_hull {
            Err(HullError)
        //Check that shield is not greater than shield capacity.
        } else if self.shield_points > val.get_shield_capacity() {
            Err(ShieldError)
        //All checks passed, template is valid.
        } else {
            self.template = val; Ok(())
        }
    }
    /// Returns the `fuel_units` of this `Ship`.
    pub fn get_fuel_units(&self) -> FuelUnit {
        self.fuel_units
    }
    /// Attempts to set the `fuel_units` of this `Ship`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// FuelError --- val > template.max_hull
    pub fn set_fuel_units(&mut self, val: FuelUnit) -> Result<(), ShipError> {
        if val > self.template.get_fuel_capacity() {
            Err(FuelError)
        } else {
            self.fuel_units = val; Ok(())
        }
    }
    /// Returns the `hull_points` of this `Ship`.
    pub fn get_hull_points(&self) -> HullPoint {
        self.hull_points
    }
    /// Attempts to set the `hull_points` of this `Ship`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// HullError --- val > template.max_hull
    pub fn set_hull_points(&mut self, val: HullPoint) -> Result<(), ShipError> {
        if val > self.template.max_hull {
            Err(HullError)
        } else {
            self.hull_points = val; Ok(())
        }
    }
    /// Returns the `shield_points` of this `Ship`.
    pub fn get_shield_points(&self) -> ShieldPoint {
        self.shield_points
    }
    /// Attempts to set the `shield_points` of this `Ship`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// HullError --- val > template.shield_capacity
    pub fn set_shield_points(&mut self, val: ShieldPoint) -> Result<(), ShipError> {
        if val > self.template.get_shield_capacity() {
            Err(ShieldError)
        } else {
            self.shield_points = val; Ok(())
        }
    }
    /// Returns true if this `Ship` is alive.
    pub fn is_alive(&self) -> bool {
        self.hull_points != 0
    }
    /// Regenerates shields for this `Ship`, capping the shields off at the shield
    /// capacity of `self.template`.
    pub fn regenerate_shields(&mut self) {
        self.shield_points += self.template.get_shield_recovery();
        
        if self.shield_points > self.template.get_shield_capacity() {
            self.shield_points = self.template.get_shield_capacity();
        }
    }
    /// Simulates damage dealt against this `Ship` and returns any which would not used
    /// to destroy this `Ship`.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against this `Ship`.
    pub fn simulate_damage(&mut self, mut damage: DamagePoint) -> (HullPoint, ShieldPoint, DamagePoint) {
        //If there's enough shields to take the damage then there will be no damage to
        //hull and no damage left...
        if damage < self.shield_points {
            (self.hull_points, self.shield_points - damage, 0)
        //Else there will be no more shields and there may be hull damage.
        } else {
            //The shields soak up some damage.
            damage -= self.shield_points;
            
            //If there's enough hull to survive the damage then there will be no damage
            //left over...
            if damage < self.hull_points {
                (self.hull_points - damage, 0, 0)
            //Else there will be no hull and maybe some damage left over.
            } else {
                (0, 0, damage - self.hull_points)
            }
        }
    }
    /// Resolves damage dealt against this `Ship` and returns any which was not used to
    /// destroy this `Ship`.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against this `Ship`.
    pub fn resolve_damage(&mut self, damage: DamagePoint) -> DamagePoint {
        //Simulate the damage.
        let simulation = self.simulate_damage(damage);
        
        //Apply the simulation to the hull.
        self.hull_points = simulation.0;
        //Apply the simulation to the shields.
        self.shield_points = simulation.1;
        //Return the unused damage.
        simulation.2
    }
    /// Resolves attacks leveled against this `Ship` and returns any which was not used
    //  to destroy this `Ship`.
    ///
    /// #Params
    ///
    /// attacks --- The attacks leveled against this `Ship`.
    pub fn resolve_attacks(&mut self, attacks: &mut ReducedAttacks) {
        //The size class of this `Ship`.
        let size_class = self.template.as_ref().ship_size_class;
        //An iterator over all the attacks, filtered by those which can target this `Ship`.
        let mut iter = attacks.iter_mut()
        .filter(|attack| attack.valid_target(size_class));
        
        //Loop which this `Ship` is still alive.
        while self.is_alive() {
            match iter.next() {
                //If there's attacks left...
                Some(attack) => {
                    //Resolve the damage from this group of attacks against this `Ship`.
                    //If any damage was unused, the number of attacks is set to reflect
                    //this; else its zeroed.
                    attack.attack.parralel_attacks = self.resolve_damage(attack.attack.sum_damage())
                        / attack.attack.damage_per_attack;
                },
                //Else all the attacks are resolved.
                None => break
            }
        }
    }
}

impl Deref for Ship {
    type Target = ShipTemplate;
    
    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

impl From<Rc<ShipTemplate>> for Ship {
    fn from(template: Rc<ShipTemplate>) -> Self {
        unsafe {
            Self::from_parts(
                template.clone(),
                template.get_fuel_capacity(),
                template.max_hull,
                template.get_shield_capacity()
            )
        }
    }
}

/// Attempts to spawn a new ship.
/// `None` is returned if a `ShipTemplate` with the passed `typename` is not found.
///
/// #Params
///
/// typename --- The type name of the ship type.
/// faction --- The faction of the Ship.
pub fn build_game_ship(typename: &String, faction: factions::Faction) -> Option<factions::AllignedInstance<Ship>> {
    get_game_templates().get(typename)
    .map(|template| factions::AllignedInstance(
        faction,
        Ship::from(template.clone())
    ))
}

// #[cfg(test)]
// mod tests {
    // use super::*;
    
    // #[test]
    // fn test_ship() {
        // let template = Rc::new(
            // ShipTemplate::new(1, 10, 1, 100, 100, 1, 0, 1, 10)
            // .expect("Failed to create template.")
        // );
        
        // let ship = Ship::new(template.clone(), 11, 100, 100);
        // assert!(
            // ship.expect_err("`Ship::new` failed to error on invalid `fuel_units`."
            // ) == FuelError,
            // "`Ship::new` returned incorrect `ShipError`."
        // );
        
        // let ship = Ship::new(template.clone(), 10, 200, 100);
        // assert!(
            // ship.expect_err("`Ship::new` failed to error on invalid `hull_points`."
            // ) == HullError,
            // "`Ship::new` returned incorrect `ShipError`."
        // );
        
        // let ship = Ship::new(template.clone(), 10, 100, 200);
        // assert!(
            // ship.expect_err("`Ship::new` failed to error on invalid `shield_points`."
            // ) == ShieldError,
            // "`Ship::new` returned incorrect `ShipError`."
        // );
        
        // let mut ship = Ship::new(template.clone(), 10, 100, 100)
        // .expect("`Ship::new` failed to create `Ship`.");
        // unsafe {
            // assert!(
                // ship == Ship::from_parts(template.clone(), 10, 100, 100),
                // "`Ship::new` returned incorrect `Ship`."
            // )
        // };
        
        // assert!(
            // ship.set_template(
                // Rc::new(
                    // ShipTemplate::new(1, 5, 1, 100, 100, 1, 0, 1, 10)
                    // .expect("Failed to create template.")
                // )
            // ).expect_err("`Ship::set_template` failed to error on invalid `fuel_capacity`."
            // ) == FuelError,
            // "`Ship::set_template` returned incorrect `ShipError`."
        // );
        
        // assert!(
            // ship.set_template(
                // Rc::new(
                    // ShipTemplate::new(1, 10, 1, 50, 100, 1, 0, 1, 10)
                    // .expect("Failed to create template.")
                // )
            // ).expect_err("`Ship::set_template` failed to error on invalid `max_hull`."
            // ) == HullError,
            // "`Ship::set_template` returned incorrect `ShipError`."
        // );
        
        // assert!(
            // ship.set_template(
                // Rc::new(
                    // ShipTemplate::new(1, 10, 1, 100, 50, 1, 0, 1, 10)
                    // .expect("Failed to create template.")
                // )
            // ).expect_err("`Ship::set_template` failed to error on invalid `shield_capacity`."
            // ) == ShieldError,
            // "`Ship::set_template` returned incorrect `ShipError`."
        // );
        
        // ship.set_template(
            // Rc::new(
                // ShipTemplate::new(1, 20, 1, 200, 200, 1, 0, 1, 10)
                // .expect("Failed to create template.")
            // )
        // ).expect("`Ship::set_template` failed to set `ShipTemplate` with greater capacities.");
        
        // ship.set_template(template)
        // .expect("`Ship::set_template` failed to set `ShipTemplate` with perfect capacities.");
        
        // assert!(ship.is_alive(), "`Ship::is_alive` failed to register alive.");
        
        // ship.regenerate_shields();
        // assert!(ship.get_shield_points() == ship.get_shield_capacity(), "`Ship::regenerate_shields` exceeded shield capacity.");
        
        // assert!(ship.resolve_damage(50) == 0, "First `Ship::resolve_damage` returned incorrect damage.");
        // assert!(
            // ship.get_shield_points() == 50
            // && ship.get_hull_points() == 100,
            // "First `Ship::resolve_damage` did incorrect damage."
        // );
        
        // ship.regenerate_shields();
        // assert!(ship.get_shield_points() == 51, "`Ship::regenerate_shields` left shields unchanged.");
        
        // ship.regenerate_shields();
        // assert!(ship.resolve_damage(100) == 0, "Second `Ship::resolve_damage` returned incorrect damage.");
        // assert!(
            // ship.get_shield_points() == 0
            // && ship.get_hull_points() == 52,
            // "Second `Ship::resolve_damage` did incorrect damage."
        // );
        
        // assert!(ship.resolve_damage(100) == 48, "Third `Ship::resolve_damage` returned incorrect damage.");
        // assert!(
            // ship.get_shield_points() == 0
            // && ship.get_hull_points() == 0,
            // "Third `Ship::resolve_damage` did incorrect damage."
        // );
        
        // assert!(!ship.is_alive(), "`Ship::is_alive` failed to register death.");
    // }
// }
