//! `the_brass` is a game where you are the leader of a space colony from Earth.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/11/09

extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::io;
use std::sync::Mutex;

mod game;

use self::game::*;

static mut STAY_ALIVE: bool = true;
static mut ALL_SHIPS: *mut Mutex<Vec<factions::AllignedInstance<combat::ships::ReducedShip>>>
    = 0 as *mut Mutex<Vec<factions::AllignedInstance<combat::ships::ReducedShip>>>;

macro_rules! get_all_ships {
    () => {
        unsafe {
            &mut *ALL_SHIPS
        }
    }
}

fn main() {
    unsafe {
        combat::ships::ship_template::init_game_templates();
        factions::init_game_factions();
        ALL_SHIPS = Box::into_raw(Box::new(Mutex::new(Vec::new())))
    }
    
    let game_thread = thread::spawn(game_loop);
    
    command_loop();
    game_thread.join().expect("Failed to join the `game_thread`.");
}

fn command_loop() {
    loop {
        let mut line = String::with_capacity(255);
        if let Ok(_) = io::stdin().read_line(&mut line) {
            line = line.trim().parse().unwrap();
            
            if line.split(' ').next().unwrap().to_lowercase() == "kill" {
                unsafe {
                    STAY_ALIVE = false;
                }
            } else if line.starts_with("spawn_ship ") {
                spawn_ship(line)
            } else if line.split(' ').next().unwrap().to_lowercase() == "kill_ships" {
                get_all_ships!().lock().unwrap().clear();
            } else {
                print_help(line);
            }
        }
        
        unsafe {
            if !STAY_ALIVE {
                break;
            }
        }
    }
}

fn print_help(line: String) {
    println!("Do not recognise command: \"{}\". Try:", line);
    println!("    spawn_ship `typename` `faction` `quantity` --- Attempts to spawn Ships using the passed parameters.");
    println!("                                    kill_ships --- Despawns all Ships.");
    println!("                                          kill --- Terminates the program.");
}

fn spawn_ship(line: String) {
    let args = line.chars().skip("spawn_ship ".len());
    let chars = args.clone().skip(1).take_while(|c| *c != "\"".chars().next().unwrap());
    let mut typename = String::with_capacity(chars.size_hint().0);
    String::extend(&mut typename, chars);
    
    let args = args.skip(typename.len() + 3);
    let chars = args.clone().take_while(|c| *c != ' ');
    let mut faction_string = String::with_capacity(chars.size_hint().0);
    String::extend(&mut faction_string, chars);
    
    if let Ok(faction) = faction_string.parse::<factions::Faction>() {
        let args = args.skip(faction_string.len() + 1);
        let chars = args.clone();
        let mut quantity = String::with_capacity(chars.size_hint().0);
        String::extend(&mut quantity, chars);
        let quantity = if let Ok(quantity) = quantity.parse::<UInt>() {
            quantity
        } else {
            1
        };
        
        if let Some(factions::AllignedInstance(faction, ship)) = combat::ships::build_game_ship(&typename, faction) {
            let mut all_ships = get_all_ships!().lock().unwrap();
            all_ships.push(factions::AllignedInstance(faction, combat::ships::ReducedShip::new(ship, quantity)));
        } else {
            println!("`spawn_ship` must have a valid type name as its first argument.\n");
        }
    } else {
        println!("`spawn_ship` expects a positive number as it's second argument, got \"{}\".\n", faction_string);
    }
}

fn game_loop() {
    let mut last_length = 0;
    loop {
        let mut all_ships = get_all_ships!().lock().expect("Failed to get Ships...");
        
        all_ships.iter_mut()
        .for_each(|ref mut ship| ship.1.regenerate_shields());
        
        all_ships.iter()
        .enumerate()
        .map(|(index, pair)| (index, pair.0, pair.1.as_ref().attack_damage * pair.1.number))
        .collect::<Vec<(usize, factions::Faction, UInt)>>().iter()
        .for_each(|&(atkr_index, faction, damage)| {
            if let Some((dfnd_index, defender)) = all_ships.iter_mut()
                .enumerate()
                .filter(|&(_, ref defender)| factions::FactionPair::new(defender.0, faction).map_or(false,
                    |pair| *factions::get_game_factions().1.entry(pair)
                        .or_insert(factions::Enemy) == factions::Enemy
                )).next() {
                println!("    Ship{} attacked Ship{}", atkr_index, dfnd_index);
                defender.1.resolve_damage(damage);
            }
        });
        
        let mut fight_won = all_ships.len() != 0;
        let mut index = 0;
        let mut num = 0;
        while index < all_ships.len() {
            if !all_ships[index].1.is_alive() {
                println!("    Ship{} died!!!", num);
                all_ships.swap_remove(index);
                if all_ships.len() == 0 {
                    println!("    All Ships dead!!!");
                    fight_won = false;
                }
            } else {
                if all_ships[index].0 != all_ships[0].0 {
                    fight_won = false;
                }
                index += 1;
                if index >= all_ships.len() && fight_won && all_ships.len() != last_length {
                    println!("    Fight won by {} with {} left...", all_ships[0].0, all_ships.iter().fold(0, |sum, ship| sum + ship.1.number));
                }
            }
            num += 1;
        }
        
        last_length = all_ships.len();
        
        unsafe {
            if !STAY_ALIVE {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
