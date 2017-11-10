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
}

#[cfg(test)]
mod tests {
}
