// dicediele is a Rust program for rolling dice and calculating various
// dice-related probabilities to assist tRPG players and DMs in making
// appropriate decisions.
// Copyright (C) 2022  Tom Su

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Please also see README.md for a list of crates used in this program and
// their respective copyright and licensing information. 

// Contact: dieeisdiele.ts@gmail.com



/// Constants for the CLI to print.
mod display;

use std::io;
// use std::error::Error;
// use clap::Parser;
// use crate::dice;



//TODO Rewrite t_roll_dice_loop() to incorporate new functions
/// Creates a loop where the user will be prompted to specify which dice to roll, be presented with the outcomes, and asked if they would like to repeat.
/// 
/// After specifying which dice to roll, the user will be given the value of each roll as well as summary values such as the highest rolled value and total value.
/// They will then be asked if they would like to roll again and can either accept to repeat the loop or decline to exit it.
/// Alternatively, they can exit early by tying `menu`.
/// 
/// This function is a WiP and could be split into smaller functions handling sections of the loop to hand more control to main.rs
// pub fn t_roll_dice_loop() {
//     'roll_dice: loop {
//         let dice_inputs: [u32; 2] = t_roll_request();                   // ask for user input
//         if dice_inputs == [0, 0] {                                      // t_roll_request() returns [0, 0] if the user enters `menu`
//             break;                                                      // exit to menu
//         }
    
//         let rolls: Vec<u32> = dice::roll_ndx(dice_inputs[0], dice_inputs[1]);       // pass user input to dice module to roll requested dice
//         for result in &rolls {                                          // print each roll on separate lines
//             println!("You rolled a {}.", result);
//         }
    
//         let highest: &u32 = match rolls.iter().max() {                  // calculate highest roll with error handling
//             Some(num) => num,
//             None => {
//                 println!("Something has gone wrong (there is no highest rolled value). Please try rolling again.");
//                 continue;
//             }
//         };
//         println!("\nYour highest roll is a {}.", highest);                // print highest roll
//         let total: u32 = rolls.iter().sum();                            // calculate total of rolls
//         println!("Your total is {}.", total);                         // print total

//         println!("\nRoll again? y/n");                                  // ask if user would like to repeat
//         loop {
//             let mut y_n: String = String::new();                        // prepare string for user input
//             match io::stdin().read_line(&mut y_n) {
//                 Ok(_) => {
//                     let y_n: String = y_n.trim().to_lowercase();        // remove whitespace from user input and convert to lowercase (removes case specificity)
//                     if y_n == "y" || y_n == "yes" {                     // repeat outer loop if user inputs `y` or `yes`
//                         break;
//                     } else if y_n == "n" || y_n == "no" {               // quits to menu if user inputs `n` or `no`
//                         break 'roll_dice;
//                     } else {                                            // prompts user again if they did not enter a valid value
//                         println!("Please enter `y` or `n`.");
//                         continue;
//                     }
//                 },
//                 Err(error) => {                                  // prompts user again if they entered non UTF-8 characters
//                     println!("error: {}", error);
//                     println!("Please enter a valid UTF-8 input.");
//                     continue;
//                 }
//             }
//         }
//     }
// }

/// Terminal prompt for rolling input. Returns user input as a string.
/// 
/// Asks the user to specify what/how many dice to roll. Trims whitespace from the input before returning.
/// 
/// # Examples
/// 
/// ```rust
/// use dicediele::terminal_io::*;
/// 
/// // Ask the user which dice they would like to roll and store their input
/// let dice_inputs: String = terminal_io::t_roll_request();
/// ```
pub fn t_roll_request() -> String {
    println!("What would you like to roll?");       // query user

    let mut input = String::new();                  // define string to be filled, also clears string on error

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => break,                         // if user input is valid UTF-8, move on
            Err(error) => {
                println!("error: {}", error);
                println!("Please enter a valid UTF-8 input, or enter `menu` to return to menu.");       // ask to reinput if format was invalid
                input.clear();
                continue;
            },
        }
    }

    let input = input.trim().to_string();           // remove whitespaces from user input
    input
}
