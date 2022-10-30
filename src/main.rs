//! # dicediele
//! 
//! dicediele is a Rust program for rolling dice and calculating various dice-related probabilities to assist tRPG players and DMs in making appropriate decisions (currently WiP).
//! 
//! ## Features
//! 
//! Allows users to specify a number of parameters which vary by system or between rolls:
//! 
//! - Sides of the dice
//! - Number of dice
//! - Exploding dice and the method of explosion (not yet implemented)
//! - How success is determined (not yet implemented)
//! 
//! ## License
//! 
//! Copyright (C) 2022  Tom Su
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License as published
//! by the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! Contact: <dieeisdiele.ts@gmail.com>



mod dice;
mod terminal_io;
fn main() {
    println!("{}\nWelcome to dicediele\n", terminal_io::SPLASH);
    terminal_io::t_roll_dice_loop();
}
