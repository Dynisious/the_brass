//! `ship_template` defines `ShipTemplate`, its construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/09

use game::*;
use super::{ShipSize, Mass};
use super::ship_error::*;
use std::sync::{Arc, Once, ONCE_INIT, Mutex, MutexGuard};
use std::io::{self, Read};
use std::path::Path;

pub type FuelUnit = UInt;
pub type HullPoint = UInt;
pub type ShieldPoint = UInt;
pub type DamagePoint = UInt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
/// `ShipTemplate` is a representation of a type of Ship.
pub struct ShipTemplate {
    /// The size class of this Ship type.
    pub ship_size_class: ShipSize,
    /// The maximum units of fuel carried by this Ship type.
    fuel_capacity: FuelUnit,
    /// The number of units of fuel used by this Ship type in one period.
    fuel_use: FuelUnit,
    /// The maximum hull points (health points) of this Ship type.
    pub max_hull: HullPoint,
    /// The maximum shield points of this Ship type.
    shield_capacity: ShieldPoint,
    /// The number of shield points regenerated by this Ship type in one period.
    shield_recovery: ShieldPoint,
    /// The maximum units of Mass this Ship type can transport.
    pub cargo_capacity: Mass,
    /// The smallest size of Ship this Ship type can target.
    pub smallest_target: ShipSize,
    /// The ammount of damage dealt by this Ship type.
    pub attack_damage: DamagePoint
}

impl ShipTemplate {
    /// Returns a new `ShipTemplate` built from raw parts.
    ///
    /// #Params
    ///
    /// ship_size_class --- The size class of this Ship type.
    /// fuel_capacity --- The maximum units of fuel carried by this Ship type.
    /// fuel_use --- The number of units of fuel used by this Ship type in one period.
    /// defence_rating --- The percentage of attacks which this Ship type evades or absorbs without consequence.
    /// hull_points --- The maximum hull points (health points) of this Ship type.
    /// shield_points --- The maximum shield points of this Ship type.
    /// shield_recovery --- The number of shield points regenerated by this Ship type in one period.
    /// cargo_capacity --- The maximum units of Mass this Ship type can transport.
    pub unsafe fn from_parts(ship_size_class: ShipSize, fuel_capacity: UInt,
        fuel_use: UInt, max_hull: UInt, shield_capacity: UInt, shield_recovery: UInt,
        cargo_capacity: Mass, smallest_target: ShipSize, attack_damage: HullPoint) -> Self {
        Self {
            ship_size_class,
            fuel_capacity,
            fuel_use,
            max_hull,
            shield_capacity,
            shield_recovery,
            cargo_capacity,
            smallest_target,
            attack_damage
        }
    }
    /// Attempts to call `ShipTemplate::from_parts` if parameters pass checks.
    ///
    /// #Params
    ///
    /// Refer to `ShipTemplate::from_parts` for parameters.
    ///
    /// #Errors
    ///
    /// FuelError --- fuel_use > fuel_capacity
    /// ShieldError --- shield_recovery > shield_points
    pub fn new(ship_size_class: ShipSize, fuel_capacity: UInt, fuel_use: UInt,
        hull_points: UInt, shield_points: UInt, shield_recovery: UInt,
        cargo_capacity: Mass, smallest_target: ShipSize, attack_damage: HullPoint) -> Result<Self, ShipError> {
        //Check that fuel use is not greater than fuel capacity.
        if fuel_use > fuel_capacity {
            Err(FuelError)
        //Check that shield_recovery is not greater than shield capacity
        } else if shield_recovery > shield_points {
            Err(ShieldError)
        //All checks passed, parameters are valid.
        } else {
            Ok(
                unsafe {
                    ShipTemplate::from_parts(
                        ship_size_class,
                        fuel_capacity,
                        fuel_use,
                        hull_points,
                        shield_points,
                        shield_recovery,
                        cargo_capacity,
                        smallest_target,
                        attack_damage
                    )
                }
            )
        }
    }
    /// Returns the `fuel_capacity` of this `ShipTemplate`.
    pub fn get_fuel_capacity(&self) -> FuelUnit {
        self.fuel_capacity
    }
    /// Attempts to set the `fuel_capacity` of this `ShipTemplate`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// FuelError --- val > fuel_capacity
    pub fn set_fuel_capacity(&mut self, val: FuelUnit) -> Result<(), ShipError> {
        if self.fuel_use > val {
            Err(FuelError)
        } else {
            self.fuel_capacity = val; Ok(())
        }
    }
    /// Returns the `fuel_use` of this `ShipTemplate`.
    pub fn get_fuel_use(&self) -> FuelUnit {
        self.fuel_use
    }
    /// Attempts to set the `fuel_use` of this `ShipTemplate`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// FuelError --- fuel_use > val
    pub fn set_fuel_use(&mut self, val: FuelUnit) -> Result<(), ShipError> {
        if val > self.fuel_capacity {
            Err(FuelError)
        } else {
            self.fuel_use = val; Ok(())
        }
    }
    /// Returns the `shield_capacity` of this `ShipTemplate`.
    pub fn get_shield_capacity(&self) -> ShieldPoint {
        self.shield_capacity
    }
    /// Attempts to set the `shield_capacity` of this `ShipTemplate`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// ShieldError --- shield_recovery > val
    pub fn set_shield_capacity(&mut self, val: ShieldPoint) -> Result<(), ShipError> {
        if self.shield_recovery > val {
            Err(ShieldError)
        } else {
            self.shield_capacity = val; Ok(())
        }
    }
    /// Returns the `shield_recovery` of this `ShipTemplate`.
    pub fn get_shield_recovery(&self) -> ShieldPoint {
        self.shield_recovery
    }
    /// Attempts to set the `shield_recovery` of this `ShipTemplate`.
    ///
    /// #Params
    ///
    /// val --- The new value to set.
    ///
    /// #Errors
    ///
    /// ShieldError --- val > shield_capacity
    pub fn set_shield_recovery(&mut self, val: ShieldPoint) -> Result<(), ShipError> {
        if val > self.shield_capacity {
            Err(ShieldError)
        } else {
            self.shield_recovery = val; Ok(())
        }
    }
    /// Returns true if this `ShipTemplate` can target the passed `ShipTemplate`.
    ///
    /// #Params
    ///
    /// target --- The `ShipTemplate` to attempt to target.
    pub fn can_target(&self, target: &Self) -> bool {
        self.smallest_target <= target.ship_size_class
    }
}

#[derive(Debug, Eq, Clone)]
/// A `ShipTemplate` with a name.
pub struct NamedTemplate(String, Arc<ShipTemplate>);

impl PartialEq for NamedTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl AsRef<ShipTemplate> for NamedTemplate {
    fn as_ref(&self) -> &ShipTemplate {
        &self.1
    }
}

pub struct TemplateBuf {
    templates: Vec<NamedTemplate>,
    max_loaded: usize
}

impl TemplateBuf {
    pub unsafe fn from_parts(templates: Vec<NamedTemplate>, max_loaded: usize) -> Self {
        Self {
            templates,
            max_loaded
        }
    }
    pub fn new(max_loaded: usize) -> Self {
        unsafe {
            Self::from_parts(Vec::with_capacity(max_loaded), max_loaded)
        }
    }
    pub fn loaded(&self) -> usize {
        self.templates.len()
    }
    pub fn max_loaded(&self) -> usize {
        self.max_loaded
    }
    pub fn set_max_loaded(&mut self, max_loaded: usize) {
        self.templates.truncate(max_loaded);
        self.max_loaded = max_loaded
    }
    pub fn get(&mut self, name: &String) -> Option<Arc<ShipTemplate>> {
        const SHIPS_PATH: &str = "./res/ships/";
        
        self.templates.iter()
        .find(|template| &template.0 == name)
        .map(|template| template.1.clone()
        ).or_else(|| {
            let mut path_string = String::from(SHIPS_PATH);
            path_string.push_str(name);
            path_string.push_str(".ship");
            
            match load_template(path_string.as_ref()) {
                Ok(template) => {
                    eprintln!("    \"{}\" Successfully loaded.", name);
                    let template = Arc::new(template);
                    
                    if self.max_loaded == self.loaded() {
                        eprintln!("    \"{}\" Successfully unloaded.", self.templates[0].0);
                        self.templates[0] = NamedTemplate(name.clone(), template.clone());
                    } else {
                        self.templates.push(NamedTemplate(name.clone(), template.clone()));
                    }
                    Some(template)
                },
                Err(e) => { eprintln!("    Failed to load: {:?}", e); None }
            }
        })
    }
    pub fn unload(&mut self, name: &String) {
        let mut index = 0;
        while index < self.loaded() {
            if &self.templates[index].0 == name {
                self.templates.swap_remove(index);
                break;
            } else {
                index += 1;
            }
        }
    }
}

fn load_template(file_path: &Path) -> Result<ShipTemplate, Result<io::Error, ::toml::de::Error>> {
    ::std::fs::File::open(file_path)
    .and_then(|mut file| {
        let mut content = String::new();
        
        file.read_to_string(&mut content)
        .map(|_| content)
    }).map_err(|e| Ok(e)
    ).and_then(|content| ::toml::from_str(content.as_str()
        ).map_err(|e| Err(e))
    )
}

static mut GAME_TEMPLATES: *mut Mutex<TemplateBuf> = 0 as *mut Mutex<TemplateBuf>;
static INIT_GAME_TEMPLATES: Once = ONCE_INIT;
pub unsafe fn init_game_templates() {
    INIT_GAME_TEMPLATES.call_once(|| {
        GAME_TEMPLATES = Box::into_raw(Box::new(Mutex::new(TemplateBuf::new(10))));
    })
}
pub fn get_game_templates() -> MutexGuard<'static, TemplateBuf> {
    unsafe {
        (*GAME_TEMPLATES).lock().expect("Ship Templates Mutex Poisoned!!!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ship_template() {
        unsafe {
            let template = ShipTemplate::new(1, 10, 1, 100, 100, 1, 0, 1, 0);
            assert!(
                template.expect("`ShipTemplate::new` failed to create `ShipTemplate`."
                ) == ShipTemplate::from_parts(1, 10, 1, 100, 100, 1, 0, 1, 0),
                "`ShipTemplate::new` returned incorrect `ShipTemplate`."
            );
        }
        
        let template = ShipTemplate::new(1, 10, 1, 100, 0, 1, 0, 1, 0);
        assert!(
            template.expect_err("`ShipTemplate::new` failed to error on invalid `shield_recovery`."
            ) == ShieldError,
            "`ShipTemplate::new` returned incorrect `ShipError`."
        );
        
        let template = ShipTemplate::new(1, 0, 1, 100, 100, 1, 0, 1, 0);
        assert!(
            template.expect_err("`ShipTemplate::new` failed to error on invalid `fuel_use`."
            ) == FuelError,
            "`ShipTemplate::new` returned incorrect `ShipError`."
        );
    }
}
