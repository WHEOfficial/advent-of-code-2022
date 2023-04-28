use crate::utils;

pub fn part_one() -> i32 {
    let lines = utils::files::get_lines("data/day02.txt");
    let mut score = 0;

    for l in lines {
        let choices: Vec<&str> = l.split(" ").collect();

        // there is probably a more efficient way to do this but i guess this
        // works for now
        match choices[1] {
            "X" => {
                score += 1;
                match choices[0] {
                    "A" => {score += 3}
                    "C" => {score += 6}
                    _ => ()
                }
            },
            "Y" => {
                score += 2;
                match choices[0] {
                    "A" => {score += 6}
                    "B" => {score += 3}
                    _ => ()
                }
            },
            "Z" => {
                score += 3;
                match choices[0] {
                    "B" => {score += 6}
                    "C" => {score += 3}
                    _ => ()
                }
            },
            _ => ()
        }
    }

    score
}

pub fn part_two() -> i32 {
    let lines = utils::files::get_lines("data/day02.txt");
    let mut score = 0;

    for l in lines {
        let choices: Vec<&str> = l.split(" ").collect();

        match choices[1] {
            "X" => {
                match choices[0] {
                    "A" => {score += 3}
                    "B" => {score += 1}
                    "C" => {score += 2}
                    _ => ()
                }
            },
            "Y" => {
                score += 3;
                match choices[0] {
                    "A" => {score += 1}
                    "B" => {score += 2}
                    "C" => {score += 3}
                    _ => ()
                }
            },
            "Z" => {
                score += 6;
                match choices[0] {
                    "A" => {score += 2}
                    "B" => {score += 3}
                    "C" => {score += 1}
                    _ => ()
                }
            },
            _ => ()
        }
    }

    score
}