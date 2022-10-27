use rand::prelude::*;

pub fn sum_n_dice(n: u32, faces: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut rng: ThreadRng= rand::thread_rng();
    
    for _ in 0..n {
        let result: u32 = rng.gen_range(1..=faces);
        sum = sum + result;
    }

    return sum
}   