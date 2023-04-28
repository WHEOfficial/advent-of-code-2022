use crate::utils;

pub fn part_one() -> u32 {
    let lines = utils::files::get_lines("data/day03.txt");

    let mut sum = 0;
    for l in lines {
        let line = l.trim();
        let first = &line[..line.len() / 2];
        let second = &line[line.len() / 2..];

        for c in first.chars() {
            if second.contains(c) {
                let mut o = c as u32;
                if o >= 97 {
                    o -= 96;
                } else if o >= 64 {
                    o -= 38;
                }
                sum += o;
                break;
            }
        }
    }

    sum
}

pub fn part_two() -> u32 {
    let lines = utils::files::get_lines("data/day03.txt");

    let mut sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let line0 = lines[i].trim();
        let line1 = lines[i + 1].trim();
        let line2 = lines[i + 2].trim();

        let mut intermediate = String::new();

        for c in line0.chars() {
            if line1.contains(c) && !intermediate.contains(c) {
                intermediate.push(c);
            }
        }

        for c in intermediate.chars() {
            if line2.contains(c) {
                let mut o = c as u32;
                if o >= 97 {
                    o -= 96;
                } else if o >= 64 {
                    o -= 38;
                }
                sum += o;
                break;
            }
        }
    }

    sum
}