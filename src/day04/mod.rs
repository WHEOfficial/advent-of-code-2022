use crate::utils;
use std::ops::Range;

pub fn part_one() -> i32 {
    let lines = utils::files::get_lines("data/day04.txt");
    let mut overlaps = 0;
    
    for l in lines {
        let ranges: Vec<&str> = l.trim().split(",").collect();

        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r2: Vec<&str> = ranges[1].split("-").collect();

        // at this point i begin to realize how much i don't know rust

        // i did try to do it without using ranges and just comparing the mins
        // and maxs of each range but i must've screwed something up somewhere
        // and didn't feel like figuring it out so i went back to the original
        // range idea and got it right

        // whatever works ¯\_(ツ)_/¯
        
        let (start1, end1, start2, end2): (i32, i32, i32, i32) = 
        (r1[0].parse().unwrap(), r1[1].parse().unwrap(), 
        r2[0].parse().unwrap(), r2[1].parse().unwrap());

        let range1: Range<i32> = start1..end1+1;
        let range2: Range<i32> = start2..end2+1;

        let smaller: Range<i32>;
        let bigger: Range<i32>;

        let mut overlapped = true;

        if range1.end - range1.start < range2.end - range2.start {
            smaller = range1;
            bigger = range2;
        } else {
            bigger = range1;
            smaller = range2;
        }

        for i in smaller {
            if !bigger.contains(&i) {
                overlapped = false;
                break;
            }
        }

        if overlapped {overlaps += 1;}
    }

    overlaps
}

pub fn part_two() -> i32 {
    let lines = utils::files::get_lines("data/day04.txt");
    let mut overlaps = 0;
    
    for l in lines {
        let ranges: Vec<&str> = l.trim().split(",").collect();

        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r2: Vec<&str> = ranges[1].split("-").collect();

        let (start1, end1, start2, end2): (i32, i32, i32, i32) = 
        (r1[0].parse().unwrap(), r1[1].parse().unwrap(), 
        r2[0].parse().unwrap(), r2[1].parse().unwrap());

        let range1: Range<i32> = start1..end1+1;
        let range2: Range<i32> = start2..end2+1;

        let mut overlapped = false;

        for i in range1 {
            if range2.contains(&i) {
                overlapped = true;
                break;
            }
        }

        if overlapped {overlaps += 1;}
    }

    overlaps
}

// i figured out how to do it by only comparing the mins and maxs, i think i
// just forgot to convert them to ints, so the better solutions are below:

pub fn better_part_one() -> i32 {
    let lines = utils::files::get_lines("data/day04.txt");
    let mut overlaps = 0;
    
    for l in lines {
        let ranges: Vec<&str> = l.trim().split(",").collect();

        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r2: Vec<&str> = ranges[1].split("-").collect();

        let (start1, end1, start2, end2): (i32, i32, i32, i32) = 
        (r1[0].parse().unwrap(), r1[1].parse().unwrap(), 
        r2[0].parse().unwrap(), r2[1].parse().unwrap());

        if (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1) {
            overlaps += 1;
        }
    }

    overlaps
}

pub fn better_part_two() -> i32 {
    let lines = utils::files::get_lines("data/day04.txt");
    let mut overlaps = 0;
    
    for l in lines {
        let ranges: Vec<&str> = l.trim().split(",").collect();

        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r2: Vec<&str> = ranges[1].split("-").collect();

        let (start1, end1, start2, end2): (i32, i32, i32, i32) = 
        (r1[0].parse().unwrap(), r1[1].parse().unwrap(), 
        r2[0].parse().unwrap(), r2[1].parse().unwrap());

        if (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1){
            overlaps += 1;
        }
    }

    overlaps
}