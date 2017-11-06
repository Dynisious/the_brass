//! `factions` defines the `Faction` type, its creation, modification and related types.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/07

use game::*;
use std::collections::HashMap;
use std::sync::{Once, ONCE_INIT};
use std::hash::{Hash, Hasher};

pub type Faction = UInt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// `Relation` defines how two `Faction`s feel about each other.
pub enum Relation {
    /// The `Faction`s are unaware that the other exists.
    Unaware,
    /// The `Faction`s are neutral towards each other.
    Neutral,
    /// The `Faction`s are friendly with each other.
    Friendly,
    /// The `Faction`s are an enemy of the other.
    Enemy
}
pub use self::Relation::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// Defines a pair of `Faction` values. When comparing (A, B) == (B, A).
pub struct FactionPair(Faction, Faction);

impl FactionPair {
    /// Creates a new `FactionPair` from raw parts without guarentees.
    ///
    /// #Params
    ///
    /// first --- The first `Faction` value.
    /// second --- The second `Faction` value.
    pub unsafe fn from_parts(first: Faction, second: Faction) -> FactionPair {
        FactionPair(first, second)
    }
    /// Creates a new `FactionPair`, checking that it is not relating to itself and that
    /// the tuple (A, B) ensures A < B.
    ///
    /// #Params
    ///
    /// Refer to `FactionPair::from_parts` for parameters.
    pub fn new(first: Faction, second: Faction) -> Option<FactionPair> {
        if first == second {
            None
        } else {
            Some(unsafe {
                if first < second {
                    FactionPair::from_parts(first, second)
                } else {
                    FactionPair::from_parts(second, first)
                }
            })
        }
    }
    /// Converts the `FactionPair` to a u64.
    pub fn as_u64(&self) -> u64 {
        unsafe {
            *(self as *const FactionPair as *const u64)
        }
    }
}

impl Hash for FactionPair {
    fn hash<T: Hasher>(&self, state: &mut T) {
        state.write_u64(self.as_u64())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// An item which is alligned with a particular faction.
pub struct AllignedInstance<T: Sized>(pub Faction, pub T);

impl<T: Sized> AsRef<T> for AllignedInstance<T> {
    fn as_ref(&self) -> &T {
        &self.1
    }
}

static mut GAME_FACTIONS: *mut (Vec<String>, HashMap<FactionPair, Relation>) = 0 as *mut (Vec<String>, HashMap<FactionPair, Relation>);
static INIT_GAME_FACTIONS: Once = ONCE_INIT;

#[cfg(features="hardcoded")]
pub unsafe fn init_game_factions() {
    INIT_GAME_FACTIONS.call_once(
        || {
            let mut factions = (Vec::with_capacity(2), HashMap::with_capacity(2));
            macro_rules! hardcode_faction {
                ($name:tt, $relation:ptrn) => {{
                    factions.0.push($name);
                    factions.1.insert($name, $relation);
                }}
            }
            
            hardcode_faction!(
                "Empire",
                (FactionPair::new(0, 1), Enemy)
            );
            GAME_FACTIONS = Box::into_raw(Box::new(factions))
        }
    )
}
#[cfg(not(features="hardcoded"))]
pub unsafe fn init_game_factions() {
    INIT_GAME_FACTIONS.call_once(
        || GAME_FACTIONS = Box::into_raw(Box::new((Vec::new(), HashMap::new())))
    )
}
pub fn get_game_factions() -> &'static mut (Vec<String>, HashMap<FactionPair, Relation>) {
    unsafe {
        &mut *GAME_FACTIONS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_faction_pair() {
        let pair = FactionPair::new(0, 0);
        assert!(pair == None, "`FactionPair::new` failed to error on self relation.");
        
        let pair = FactionPair::new(0, 1);
        unsafe {
            assert!(pair == Some(FactionPair::from_parts(0, 1)), "`FactionPair::new` failed to create new `FactionPair`.");
        }
        
        let pair = FactionPair::new(1, 0);
        unsafe {
            assert!(pair == Some(FactionPair::from_parts(0, 1)), "`FactionPair::new` failed to swap factions.");
        }
    }
}
