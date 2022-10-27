mod dice;
fn main() {
    println!("Ready to roll.");

    let result: u32 = dice::sum_n_dice(2, 6);
    println!("You rolled a {}.", result);
}
