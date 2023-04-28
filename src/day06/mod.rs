use crate::utils;

pub fn part_one() -> i32 {
    let input = &utils::files::get_lines("data/day06.txt")[0];

    for i in 0..input.len() - 3 {
        let slice = &input[i..i+4];
        let mut duplicate = false;

        let mut temp = String::new();
        for c in slice.chars() {
            if !temp.contains(c) {
                temp.push(c);
            } else { duplicate = true; break; }
        }

        if !duplicate {
            return (i + 4) as i32;
        }
    }

    0
}

pub fn part_two() -> i32 {
    let input = &utils::files::get_lines("data/day06.txt")[0];

    for i in 0..input.len() - 13 {
        let slice = &input[i..i+14];
        let mut duplicate = false;

        let mut temp = String::new();
        for c in slice.chars() {
            if !temp.contains(c) {
                temp.push(c);
            } else { duplicate = true; break; }
        }

        if !duplicate {
            return (i + 14) as i32;
        }
    }

    0
}