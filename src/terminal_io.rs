use std::io;

/// Terminal prompt for rolling input. Returns processed user input as an array [N, X].
/// 
/// Asks the user to specify what/how many dice to roll. Expects a value in the format input, where N is the number of X-sided dice to roll.
/// Also parses the input and returns an array [u32;2] containing [N, X].
/// 
/// # Examples
/// ```
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
pub fn t_roll_request() -> [u32; 2] {
    println!("What would you like to roll?");                               // query user

    let mut dice_specs: [u32; 2] = [0; 2];                                  // define output array

    'input_query: loop {
        let mut input: String = String::new();                              // define string to be filled, also clears string on error
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},                                                    // if user input is valid UTF-8, move on
            Err(error) => {
                println!("error: {}", error);
                println!("Please enter a valid UTF-8 input.");              // ask to reinput if format was invalid
                continue 'input_query;
            },
        }
        let input: &str = input.trim();                                     // remove whitespaces from user input

        let str_n_and_x: Vec<&str> = input.split(&['d', 'D']).collect();    // split user input using separators 'd' or 'D'
        if str_n_and_x.len() != 2 {                                         // check that user input was split into two segments
            println!("Please enter a value of the format `input`, where `N` and `X` are positive integers.");
            continue 'input_query;                                          // ask to reinput if format was invalid
        }
        for i in 0..2 {
            dice_specs[i] = match str_n_and_x[i].parse::<u32>() {           // try to parse each segment into u32 and write to output array
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a value of the format `input`, where `N` and `X` are positive integers.");
                    continue 'input_query;                                  // ask to reinput if format was invalid
                }
            };
            if dice_specs[i] <= 0 {                                         // check that input value is greater than zero
                println!("Please enter a value of the format `input`, where `N` and `X` are positive integers.");
                continue 'input_query;                                      // ask to reinput if format was invalid
            }
        }

        println!("Rolling {}...", input);                                   // confirm input value
        break;
    }

    return dice_specs
}
