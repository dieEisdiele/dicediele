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



use rand::prelude::*;

/// Rolls N X-sided dice and returns a vector of the results.
/// 
/// # Arguments
/// 
/// * `n` - How many dice to roll
/// * `x` - How many faces each die should have
/// 
/// # Examples
/// 
/// ```
/// use dicediele::dice::*;
/// 
/// // Roll 2d6 (2 6-sided dice) and print each roll on a separate line
/// let roll_results: Vec<u32> = roll_ndx(2, 6);
/// for result in roll_results {
///     println!("You rolled a {}.", result);
/// }
/// ```
pub fn roll_ndx(n: u32, x: u32) -> Vec<u32> {
    let mut rolls: Vec<u32> = Vec::new();           // create mutable vector to store results in
    let mut rng: ThreadRng = rand::thread_rng();    // cache random number generator
    
    // roll one X-sided die N times
    for _ in 0..n {
        let result: u32 = rng.gen_range(1..=x);     // result of rolling X-sided die
        rolls.push(result);                         // push result of die roll to vector
    }

    return rolls
}
