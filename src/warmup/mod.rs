use crate::utils;

pub fn part_one() {
    let lines = utils::files::get_lines("data/warmup.txt");
    let mut increases = 0;
    let mut previous = 0;

    for s in lines {
        let num: i32 = s.parse().unwrap();
        if num > previous && previous != 0 { increases += 1; }
        previous = num;
    }
    println!("Increases: {}", increases)
}

pub fn part_two() {
    let lines = utils::files::get_lines("data/warmup.txt");
    let mut increases = 0;

    for i in 0..lines.len() - 3 {
        let n1: i32 = lines[i].parse().unwrap();
        let n2: i32 = lines[i+1].parse().unwrap();
        let n3: i32 = lines[i+2].parse().unwrap();
        let n4: i32 = lines[i+3].parse().unwrap();

        let s1 = n1 + n2 + n3;
        let s2 = n2 + n3 + n4;

        if s2 > s1 {increases += 1}
    }

    println!("Sliding increases: {}", increases)
}