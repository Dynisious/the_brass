//! `combat_instance` defines combat between `Fleet`s.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/09/22

use game::*;
use game::combat::fleets::AllignedFleet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CombatInstance {
    battle: Vec<Battle>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Battle {
    first_fleet: AllignedFleet,
    second_fleet: AllignedFleet
}
