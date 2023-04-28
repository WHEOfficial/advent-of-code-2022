// please forgive my rust illiteracy
#![allow(dead_code)]

use std::time::Instant;

mod utils {
    pub mod files;
}

mod warmup;
mod day01;
mod day02;

fn main() {
    let now = Instant::now();
    let one_ans = day02::part_one();
    let two_ans = day02::part_two();
    let elapsed = now.elapsed();

    println!("Part one answer is {0}, part two is {1}", one_ans, two_ans);
    println!("Elapsed: {:.2?}", elapsed);
}