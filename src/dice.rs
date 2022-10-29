use rand::prelude::*;

/// Rolls N X-sided dice and returns a vector of the results.
/// 
/// # Arguments
/// 
/// * `n` - How many dice to roll
/// * `x` - How many faces each die should have
/// 
/// # Examples
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
