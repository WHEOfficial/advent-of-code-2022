use crate::utils;
use regex::Regex;

// i miss python

pub fn part_one() -> String {
    let lines = utils::files::get_lines("data/day05.txt");

    let mut tops = String::new();

    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    let num_crates = lines[0].len() / 3;
    let mut crate_piles: Vec<Vec<char>> = Vec::with_capacity(num_crates);
    for i in 0..num_crates {
        crate_piles.push(Vec::new());
    }

    let mut getting_crates = true;

    for l in lines {
        if getting_crates {
            if l == "" {
                getting_crates = false;
                continue;
            }

            for i in (0..l.len()).step_by(4) {
                let char = l.chars().nth(i+1).unwrap();
                if char != ' ' && !char.is_numeric() {
                    crate_piles[i / 4].insert(0, char);
                }
            }
        } else {
            let capture = re.captures(&l).unwrap();
            let (amount, from, to): (usize, usize, usize) = (
                capture.get(1).map_or("", |m| m.as_str())
                    .parse().unwrap(), 
                capture.get(2).map_or("", |m| m.as_str())
                    .parse().unwrap(),
                capture.get(3).map_or("", |m| m.as_str())
                    .parse().unwrap()
            );
            
            for i in 0..amount {
                let moved = crate_piles[from - 1].pop().unwrap();
                crate_piles[to - 1].push(moved);
            }
        }
    }

    for mut c in crate_piles {
        if c.len() > 0 {
            tops.push(c.pop().unwrap());
        }   
    }

    tops
}

pub fn part_two() -> String {
    let lines = utils::files::get_lines("data/day05.txt");

    let mut tops = String::new();

    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    let num_crates = lines[0].len() / 3;
    let mut crate_piles: Vec<Vec<char>> = Vec::with_capacity(num_crates);
    for i in 0..num_crates {
        crate_piles.push(Vec::new());
    }

    let mut getting_crates = true;

    for l in lines {
        if getting_crates {
            if l == "" {
                getting_crates = false;
                continue;
            }

            for i in (0..l.len()).step_by(4) {
                let char = l.chars().nth(i+1).unwrap();
                if char != ' ' && !char.is_numeric() {
                    crate_piles[i / 4].insert(0, char);
                }
            }
        } else {
            let capture = re.captures(&l).unwrap();
            let (amount, from, to): (usize, usize, usize) = (
                capture.get(1).map_or("", |m| m.as_str())
                    .parse().unwrap(), 
                capture.get(2).map_or("", |m| m.as_str())
                    .parse().unwrap(),
                capture.get(3).map_or("", |m| m.as_str())
                    .parse().unwrap()
            );
            
            let length = crate_piles[to - 1].len();
            for i in 0..amount {
                let moved = crate_piles[from - 1].pop().unwrap();
                crate_piles[to - 1].insert(length, moved);
            }
        }
    }

    for mut c in crate_piles {
        if c.len() > 0 {
            tops.push(c.pop().unwrap());
        }   
    }

    tops
}