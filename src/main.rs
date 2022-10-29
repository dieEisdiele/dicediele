mod dice;
mod terminal_io;
fn main() {
    println!("Ready to roll.");

    let dice_inputs: [u32; 2] = terminal_io::t_roll_request();

    let rolls: Vec<u32> = dice::roll_ndx(dice_inputs[0], dice_inputs[1]);
    for result in &rolls {
        println!("You rolled a {}.", result);
    }

    let sum: u32 = rolls.iter().sum();
    println!("Your total is {}.", sum);
}
