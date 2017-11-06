//! `faction` defines factions within the game.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/5

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
/// `Faction` is a named faction in the game.
pub struct Faction {
    /// The name of the `Faction`.
    name: String
}

impl Faction {
    /// Builds a new `Faction` based on the passed string with some standardised
    /// formatting:
    ///     * First letter is capitalised.
    ///     * First letter follow a space is capitalised.
    ///
    /// #Params
    ///
    /// name_root --- The string to base the faction name on.
    pub fn new(name_root: &str) -> Result<Faction, FactionError> {
        let mut name = String::with_capacity(name_root.trim().len());
        if name_root.is_empty() {
            return Err(FactionError)
        }
        
        {
            let mut char_iter = name_root.chars();
            name.push(
                char_iter
                    .next()
                    .unwrap()
                    .to_uppercase()
                        .next()
                        .unwrap()
            );
            
            loop {
                match char_iter.next() {
                    None => break,
                    Some(c) => {
                        if c == ' ' {
                            while {
                                let upper_char = char_iter
                                    .next()
                                    .unwrap()
                                    .to_uppercase()
                                        .next()
                                        .unwrap();
                                if upper_char != ' ' {
                                    name.push(upper_char);
                                    false
                                } else {
                                    true
                                }
                            } {};
                        }
                    }
                }
            }
        }
        
        Ok(
            Faction {
                name
            }
        )
    }
}

impl Into<String> for Faction {
    fn into(self) -> String {
        self.name
    }
}

impl AsRef<String> for Faction {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
/// An error raised when passing the string to build a `Faction`.
pub struct FactionError;

#[derive(Eq, Clone)]
/// A collection of relations from one `Faction` to many others.
pub struct FactionRelationships {
    /// The `Faction` which is relating to all the others.
    pub core: Faction,
    /// The way the core `Faction` relates to other `Faction`s.
    relationships: RelationMap
}

impl FactionRelationships {
    /// Creates a `FactionRelationships` around the passed `Faction` which is unaware of
    /// any other `Faction`.
    ///
    /// #Params
    ///
    /// core --- The `Faction` which these relationships are centred around.
    pub fn new(core: Faction) -> FactionRelationships {
        FactionRelationships {
            core,
            relationships: RelationMap::new()
        }
    }
    /// Creates a `FactionRelationships` around the passed `Faction` with the passed
    /// relationships.
    ///
    /// #Params
    ///
    /// core --- The `Faction` which these relationships are centred around.
    /// relationships --- The way the `core` `Faction` relates to the other `Faction`s.
    pub fn from_parts(core: Faction, mut relationships: RelationMap) -> FactionRelationships {
        relationships.remove(&core);
        
        FactionRelationships {
            core,
            relationships
        }
    }
    /// Returns the relationship the `core` `Faction` has with the passed `Faction`.
    ///
    /// #Params
    ///
    /// faction --- The `Faction` to retrieve the `Relation` for.
    pub fn get_relation(&self, faction: &Faction) -> Relation {
        use self::Relation::*;
        
        if faction == &self.core {
            OwnFaction
        } else {
            match self.relationships.get(faction) {
                None => Unaware,
                Some(relation) => relation.clone()
            }
        }
    }
    /// Attempts to set the relationship of the `core` `Faction` with the passed
    /// `Faction` and returns the old `Relation`. `None` will be returned if `faction` is
    /// the `core` `Faction`. If no relation previously existed `Unaware` is returned.
    ///
    /// #Params
    ///
    /// faction --- The `Faction` to set the `Relation` too.
    /// relation --- The `Relation` to assign to `faction`.
    pub fn set_relation(&mut self, faction: Faction, relation: Relation) -> Option<Relation> {
        use self::Relation::*;
        
        if faction == self.core {
            None
        } else if relation == Unaware {
            match self.relationships.remove(&faction) {
                None => Some(Unaware),
                relation => relation
            }
        } else {
            match self.relationships.insert(faction, relation) {
                None => Some(Unaware),
                relation => relation
            }
        }
    }
    /// Checks if the two `FactionRelationships` values are `consistent` between eachother.
    /// `consistent` here means that the two factions agree on their `Relation` to
    /// eachother.
    ///
    /// #Params
    ///
    /// left --- The first `FactionRelationships` value to check.
    /// right --- The second `FactionRelationships` value to check.
    pub fn are_consistent(left: &Self, right: &Self) -> bool {
        use self::Relation::Unaware;
        
        let relation = left.get_relation(&right.core);
        if relation == Unaware {
            true
        } else {
            let other_relation = right.get_relation(&left.core);
            if other_relation == Unaware {
                true
            } else {
                relation == other_relation
            }
        }
    }
}

impl PartialEq for FactionRelationships {
    fn eq(&self, other: &Self) -> bool {
        if self.core != other.core {
            false
        } else {
            if self.relationships.len() != other.relationships.len() {
                false
            } else {
                for (faction, relation) in self.relationships.iter() {
                    match other.relationships.get(faction) {
                        None => return false,
                        Some(other_relation) => if relation != other_relation {
                            return false
                        }
                    }
                }
                true
            }
        }
    }
}

impl Into<RelationMap> for FactionRelationships {
    fn into(self) -> RelationMap {
        self.relationships
    }
}

pub type RelationMap = HashMap<Faction, Relation>;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Relation {
    Unaware,
    OwnFaction,
    Allied,
    AtWar
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_faction_relationships() {
        use super::Relation::*;
        
        let core = if let Ok(_) = Faction::new("") {
            panic!("`test_faction_family` Failed to error on invalid Faction string.")
        } else {
            Faction::new("test faction")
                .expect("`test_faction_family` Failed to create Faction from valid string.")
        };
        let faction = Faction::new("second faction")
            .expect("`test_faction_family` Failed to create Faction from second valid string.");
        
        let mut relationships = FactionRelationships::new(core.clone());
        assert!(relationships.get_relation(&core) == OwnFaction,
            "`test_faction_family` Failed to recognise own Faction."
        );
        
        match relationships.set_relation(faction.clone(), Unaware) {
            None => panic!("`test_faction_family` Failed to recognise relation."),
            Some(x) => if x != Unaware {
                panic!("`test_faction_family` Failed to return expect relation.")
            }
        }
        
        relationships.set_relation(
            faction.clone(),
            Allied
        ).expect("`test_faction_family` Failed to set new relation.");
        
        assert!(relationships == FactionRelationships::from_parts(
                core.clone(),
                relationships.clone().into()
            ),
            "`test_faction_family` Failed to build `FactionRelationships` from parts."
        );
        
        let mut second_relationships = FactionRelationships::new(faction);
        second_relationships.set_relation(
            core.clone(),
            AtWar
        );
        
        assert!(!FactionRelationships::are_consistent(
                &relationships,
                &second_relationships
            ), "`test_faction_family` Failed to check for consistent relationships."
        );
        
        assert!(second_relationships.set_relation(
                core.clone(),
                Allied
            ) == Some(AtWar), "`test_faction_family` Failed to set new relationship."
        );
        
        assert!(FactionRelationships::are_consistent(
                &relationships,
                &second_relationships
            ), "`test_faction_family` Failed to evaluate for consistent relationships."
        );
    }
}
