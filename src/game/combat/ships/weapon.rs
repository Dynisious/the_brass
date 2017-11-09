//! `weapon` defines the `Weapon` type, its construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/09

use game::*;

pub struct Weapon {
    pub parralel_attacks: UInt,
    pub damage_per_attack: UInt
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
}
