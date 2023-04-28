use crate::utils;

pub fn part_one() {
    let lines = utils::files::get_lines("data/warmup.txt");
    let mut increases = 0;
    let mut previous = 0;

    for s in lines {
        let num: i32 = s.trim().parse().unwrap();
        if num > previous && previous != 0 { increases += 1; }
        previous = num;
    }
    println!("Increases: {}", increases)
}

pub fn part_two() {
    let lines = utils::files::get_lines("data/warmup.txt");
    let mut increases = 0;

    for i in 0..lines.len() - 3 {
        //println!("{}", lines[i]);
        let n1: i32 = lines[i].trim().parse().unwrap();
        let n2: i32 = lines[i+1].trim().parse().unwrap();
        let n3: i32 = lines[i+2].trim().parse().unwrap();
        let n4: i32 = lines[i+3].trim().parse().unwrap();

        let s1 = n1 + n2 + n3;
        let s2 = n2 + n3 + n4;

        if s2 > s1 {increases += 1}
    }

    println!("Sliding increases: {}", increases)
}