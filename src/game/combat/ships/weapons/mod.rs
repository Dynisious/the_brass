//! `weapons` defines weapons, their construction and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/09/22

use game::*;

pub mod distinct_weapon;
pub mod reduced_weapon;
pub mod weapon_error;
pub mod targeted_damage;
pub mod distributed_damage;

pub use self::distinct_weapon::*;
pub use self::reduced_weapon::*;
pub use self::weapon_error::*;
pub use self::targeted_damage::*;
pub use self::distributed_damage::*;

pub type DamagePoint = UInt;
pub type AttackProjectile = UInt;
