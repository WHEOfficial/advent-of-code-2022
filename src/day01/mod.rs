use crate::utils;

pub fn part_one() -> i32{
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
    prev
}

pub fn part_two() -> i32 {
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

    totals[0] + totals[1] + totals[2]
}