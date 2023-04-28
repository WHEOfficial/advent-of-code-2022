// please forgive my rust illiteracy
#![allow(dead_code)]

use std::time::Instant;

mod utils {
    pub mod files;
}

mod warmup;
mod day01;

fn main() {
    let now = Instant::now();
    day01::part_one();
    day01::part_two();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}