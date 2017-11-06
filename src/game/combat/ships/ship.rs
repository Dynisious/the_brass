//! `ship` defines `Ship`, its construction, modification and interactions between `Ship`s.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/06

use game::*;
use super::ship_error::*;
use super::ship_template::*;
use std::sync::Arc;
use std::rc::Rc;

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
    /// Resolves damage dealt against this `Ship` and returns any which was not used to
    /// destroy this `Ship`.
    ///
    /// #Params
    ///
    /// damage --- The damage leveled against this `Ship`.
    pub fn resolve_damage(&mut self, mut damage: DamagePoint) -> DamagePoint {
        if damage < self.shield_points {
            self.shield_points -= damage; 0
        } else {
            damage -= self.shield_points;
            self.shield_points = 0;
            
            if damage < self.hull_points {
                self.hull_points -= damage; 0
            } else {
                damage -= self.hull_points;
                self.hull_points = 0;
                damage
            }
        }
    }
    /// Regenerates shields for this `Ship`, capping the shields off at the shield
    /// capacity of `self.template`.
    pub fn regenerate_shields(&mut self) {
        self.shield_points += self.template.get_shield_recovery();
        
        if self.shield_points > self.template.get_shield_capacity() {
            self.shield_points = self.template.get_shield_capacity();
        }
    }
}

impl AsRef<ShipTemplate> for Ship {
    fn as_ref(&self) -> &ShipTemplate {
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
pub fn spawn_ship(typename: &String, faction: factions::Faction) -> Option<factions::AllignedInstance<Ship>> {
    unsafe {
        get_game_templates().get(typename)
        .map(|template| factions::AllignedInstance(
            faction,
            Ship::from(
                Rc::from_raw(
                    Arc::into_raw(template.clone())
                )
            )
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ship() {
        let template = Rc::new(
            ShipTemplate::new(1, 10, 1, 100, 100, 1, 0, 1, 10)
            .expect("Failed to create template.")
        );
        
        let ship = Ship::new(template.clone(), 11, 100, 100);
        assert!(
            ship.expect_err("`Ship::new` failed to error on invalid `fuel_units`."
            ) == FuelError,
            "`Ship::new` returned incorrect `ShipError`."
        );
        
        let ship = Ship::new(template.clone(), 10, 200, 100);
        assert!(
            ship.expect_err("`Ship::new` failed to error on invalid `hull_points`."
            ) == HullError,
            "`Ship::new` returned incorrect `ShipError`."
        );
        
        let ship = Ship::new(template.clone(), 10, 100, 200);
        assert!(
            ship.expect_err("`Ship::new` failed to error on invalid `shield_points`."
            ) == ShieldError,
            "`Ship::new` returned incorrect `ShipError`."
        );
        
        let mut ship = Ship::new(template.clone(), 10, 100, 100)
        .expect("`Ship::new` failed to create `Ship`.");
        unsafe {
            assert!(
                ship == Ship::from_parts(template.clone(), 10, 100, 100),
                "`Ship::new` returned incorrect `Ship`."
            )
        };
        
        assert!(
            ship.set_template(
                Rc::new(
                    ShipTemplate::new(1, 5, 1, 100, 100, 1, 0, 1, 10)
                    .expect("Failed to create template.")
                )
            ).expect_err("`Ship::set_template` failed to error on invalid `fuel_capacity`."
            ) == FuelError,
            "`Ship::set_template` returned incorrect `ShipError`."
        );
        
        assert!(
            ship.set_template(
                Rc::new(
                    ShipTemplate::new(1, 10, 1, 50, 100, 1, 0, 1, 10)
                    .expect("Failed to create template.")
                )
            ).expect_err("`Ship::set_template` failed to error on invalid `max_hull`."
            ) == HullError,
            "`Ship::set_template` returned incorrect `ShipError`."
        );
        
        assert!(
            ship.set_template(
                Rc::new(
                    ShipTemplate::new(1, 10, 1, 100, 50, 1, 0, 1, 10)
                    .expect("Failed to create template.")
                )
            ).expect_err("`Ship::set_template` failed to error on invalid `shield_capacity`."
            ) == ShieldError,
            "`Ship::set_template` returned incorrect `ShipError`."
        );
        
        ship.set_template(
            Rc::new(
                ShipTemplate::new(1, 20, 1, 200, 200, 1, 0, 1, 10)
                .expect("Failed to create template.")
            )
        ).expect("`Ship::set_template` failed to set `ShipTemplate` with greater capacities.");
        
        ship.set_template(template)
        .expect("`Ship::set_template` failed to set `ShipTemplate` with perfect capacities.");
        
        assert!(ship.is_alive(), "`Ship::is_alive` failed to register alive.");
        
        ship.regenerate_shields();
        assert!(ship.get_shield_points() == ship.as_ref().get_shield_capacity(), "`Ship::regenerate_shields` exceeded shield capacity.");
        
        assert!(ship.resolve_damage(50) == 0, "First `Ship::resolve_damage` returned incorrect damage.");
        assert!(
            ship.get_shield_points() == 50
            && ship.get_hull_points() == 100,
            "First `Ship::resolve_damage` did incorrect damage."
        );
        
        ship.regenerate_shields();
        assert!(ship.get_shield_points() == 51, "`Ship::regenerate_shields` left shields unchanged.");
        
        ship.regenerate_shields();
        assert!(ship.resolve_damage(100) == 0, "Second `Ship::resolve_damage` returned incorrect damage.");
        assert!(
            ship.get_shield_points() == 0
            && ship.get_hull_points() == 52,
            "Second `Ship::resolve_damage` did incorrect damage."
        );
        
        assert!(ship.resolve_damage(100) == 48, "Third `Ship::resolve_damage` returned incorrect damage.");
        assert!(
            ship.get_shield_points() == 0
            && ship.get_hull_points() == 0,
            "Third `Ship::resolve_damage` did incorrect damage."
        );
        
        assert!(!ship.is_alive(), "`Ship::is_alive` failed to register death.");
    }
}
