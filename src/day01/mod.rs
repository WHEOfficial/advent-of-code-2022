use crate::utils;

pub fn part_one() {
    let lines = utils::files::get_lines("data/day01.txt");
    let mut prev = 0;
    let mut current = 0;

    for l in lines {
        if l == "" {
            if current > prev {prev = current}
            current = 0;
        } else {
            let num: i32 = l.trim().parse().unwrap();
            current += num;
        }
    }

    if current > prev {prev = current}
    println!("Highest total is {}", prev);
}

pub fn part_two() {
    let lines = utils::files::get_lines("data/day01.txt");
    let mut totals = Vec::new();
    let mut sum = 0;

    for l in lines {
        if l == "" {
            totals.push(sum);
            sum = 0;
        } else {
            let num: i32 = l.trim().parse().unwrap();
            sum += num;
        }
    }
    totals.push(sum);
    totals.sort_by(|a, b| b.cmp(a));

    println!("Sum of three highest is {}", totals[0] + totals[1] + totals[2]);
}