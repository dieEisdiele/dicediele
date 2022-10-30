//     dicediele is a Rust program for rolling dice and calculating various dice-related probabilities to assist tRPG players and DMs in making appropriate decisions.
//     Copyright (C) 2022  Tom Su

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.

//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

//     Contact: dieeisdiele.ts@gmail.com



use std::io;
// use std::error::Error;
use crate::dice;



/// A logo to print to terminal on startup
pub const SPLASH: &str = r#"
              _    _                                 ,.::""""::.,         
             | |  (_)                           ,.::"''  ***   ''"::.,    
         ____| |   _    _____    ______      ;::"''      ***        ''":; 
        /  _   |  | |  /  ___\  /  __  \    ::       ***      ***       ::
       |  (_)  |  | | |  (____ |  (_/ /__   ::':.,   ***      ***   ,.:'::
        \____/\_\ |_|  \_____/  \_______/   ::  ''"::.,        ,.::"''  ::
                            _               ::       ''"::;;::"''       ::
        _    _             | |              ::            ::  ***       ::
       | |  (_)            | |              ::    ****    ::  ***       ::
   ____| |   _    ______   | |   ______     ::     ***    ::        *** ::
  /  _   |  | |  /  __  \  | |  /  __  \    ::            ::        *** ::
 |  (_)  |  | | |  (_/ /__ | | |  (_/ /__    "::.,        ::        ,.::" 
  \____/\_\ |_|  \_______/ |_|  \_______/       ''"::.,   ::   ,.::"''    
                                                     ''"::;;::"''
"#;



/// Creates a loop where the user will be prompted to specify which dice to roll, be presented with the outcomes, and asked if they would like to repeat.
/// 
/// After specifying which dice to roll, the user will be given the value of each roll as well as summary values such as the highest rolled value and total value.
/// They will then be asked if they would like to roll again and can either accept to repeat the loop or decline to exit it.
/// Alternatively, they can exit early by tying `menu`.
/// 
/// This function is a WiP and could be split into smaller functions handling sections of the loop to hand more control to main.rs
pub fn t_roll_dice_loop() {
    'roll_dice: loop {
        let dice_inputs: [u32; 2] = t_roll_request();                   // ask for user input
        if dice_inputs == [0, 0] {                                      // t_roll_request() returns [0, 0] if the user enters `menu`
            break;                                                      // exit to menu
        }
    
        let rolls: Vec<u32> = dice::roll_ndx(dice_inputs[0], dice_inputs[1]);       // pass user input to dice module to roll requested dice
        for result in &rolls {                                          // print each roll on separate lines
            println!("You rolled a {}.", result);
        }
    
        let highest: &u32 = match rolls.iter().max() {                  // calculate highest roll with error handling
            Some(num) => num,
            None => {
                println!("Something has gone wrong (there is no highest rolled value). Please try rolling again.");
                continue;
            }
        };
        println!("Your highest roll is a {}.", highest);                // print highest roll
        let total: u32 = rolls.iter().sum();                            // calculate total of rolls
        println!("\nYour total is {}.", total);                         // print total

        println!("\nRoll again? y/n");                                  // ask if user would like to repeat
        loop {
            let mut y_n: String = String::new();                        // prepare string for user input
            match io::stdin().read_line(&mut y_n) {
                Ok(_) => {
                    let y_n: String = y_n.trim().to_lowercase();        // remove whitespace from user input and convert to lowercase (removes case specificity)
                    if y_n == "y" || y_n == "yes" {                     // repeat outer loop if user inputs `y` or `yes`
                        break;
                    } else if y_n == "n" || y_n == "no" {               // quits to menu if user inputs `n` or `no`
                        break 'roll_dice;
                    } else {                                            // prompts user again if they did not enter a valid value
                        println!("Please enter `y` or `n`.");
                        continue;
                    }
                },
                Err(error) => {                                  // prompts user again if they entered non UTF-8 characters
                    println!("error: {}", error);
                    println!("Please enter a valid UTF-8 input.");
                    continue;
                }
            }
        }
    }
}

/// Terminal prompt for rolling input. Returns processed user input as an array.
/// 
/// Asks the user to specify what/how many dice to roll. Expects a value in the format input, where N is the number of X-sided dice to roll.
/// Also parses the input and returns a `[u32;2]` array containing `[N, X]`. Alternatively, returns `[0,0]` if the user types `menu`.
/// 
/// # Examples
/// 
/// ```rust
/// use dicediele::terminal_io::*;
/// use dicediele::dice::*;
/// 
/// // Ask the user which dice they would like to roll and store their input
/// let dice_inputs: [u32; 2] = terminal_io::t_roll_request();
/// 
/// // Pass the user input to a function which rolls input
/// let rolls: Vec<u32> = dice::roll_input(dice_inputs[0], dice_inputs[1]);
/// 
/// // Print the results of each die as well as the total on separate lines
/// for result in &rolls {
///     println!("You rolled a {}.", result);
/// }
/// let sum: u32 = rolls.iter().sum();
/// println!("Your total is {}.", sum); 
/// ```
fn t_roll_request() -> [u32; 2] {
    println!("What would you like to roll?");                               // query user

    let mut dice_specs: [u32; 2] = [0; 2];                                  // define output array

    'input_query: loop {
        let mut input: String = String::new();                              // define string to be filled, also clears string on error
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},                                                    // if user input is valid UTF-8, move on
            Err(error) => {
                println!("error: {}", error);
                println!("Please enter a valid UTF-8 input, or enter `menu` to return to menu.");              // ask to reinput if format was invalid
                continue;
            },
        }
        let input: &str = input.trim();                                     // remove whitespaces from user input

        if input == "menu" {
            break;
        }

        // TODO Turn the following into a function which returns Result<[u32; 2], Error>
        // Goal is to allow a single match block when calling the function to handle the println! and continue block that is used repeatedly:
        // let dice_specs = match process_user_input(args) {
        //     Ok(array) => array;
        //     Err(_) => {
        //         println!("Please enter a value of the format `input`, where `N` and `X` are positive integers.");
        //         continue;
        //     }
        // };
        let generic_error_msg: &str = "Please enter a value of the format `NdX`, where `N` and `X` are positive integers, or enter `menu` to return to menu.";

        let str_n_and_x: Vec<&str> = input.split(&['d', 'D']).collect();    // split user input using separators 'd' or 'D'
        if str_n_and_x.len() != 2 {                                         // check that user input was split into two segments
            println!("The divider between your two values should be the letter `d`.");
            println!("{}", generic_error_msg);
            continue;                                                       // ask to reinput if format was invalid
        }
        for i in 0..2 {
            dice_specs[i] = match str_n_and_x[i].parse::<u32>() {           // try to parse each segment into u32 and write to output array
                Ok(num) => num,
                Err(_) => {
                    println!("The number on each side of the letter `d` should be a positive integer.");
                    println!("{}", generic_error_msg);
                    continue 'input_query;                                  // ask to reinput if format was invalid
                }
            };
            if dice_specs[i] <= 0 {                                         // check that input value is greater than zero
                println!("The number on each side of the letter `d` must be greater than (not equal to) zero.");
                println!("{}", generic_error_msg);
                continue 'input_query;                                      // ask to reinput if format was invalid
            }
        }

        println!("Rolling {}...\n", input);                                   // confirm input value
        break;
    }

    return dice_specs
}

// fn process_user_input(user_input: &str, output_array: [u32; 2]) -> Result<[u32; 2], Box<Error>> {
//     let str_n_and_x: Vec<&str> = user_input.split(&['d', 'D']).collect();
//     if str_n_and_x.len() != 2 {
//         return Error
//     }
//     for i in 0..2 {
//         output_array[i] = str_n_and_x[i].parse::<u32>()?;
//         };
//     if output_array[i] <= 0 {
//         return Error
//     }
// }
