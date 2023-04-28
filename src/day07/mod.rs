use crate::utils;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
struct File {
    name: String,
    size: i32,    
}

impl File {
    pub fn new(name: &String, size: i32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

fn get_size(dir: &String, dirs: &HashMap<String, Vec<String>>, sizes: &HashMap<String, i32>) -> i32 {
    let mut total_size = 0;

    total_size += sizes.get(dir).unwrap();

    total_size
}

pub fn part_one() -> i32 {
    let lines = utils::files::get_lines("data/day07.txt");

    let mut total = 0;
    let mut dir_sum = 0;

    let mut directories: HashMap<String, Vec<String>> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut files: HashMap<String, Vec<File>> = HashMap::new();
    let mut sizes: HashMap<String, i32> = HashMap::new();

    let mut pwd: String = "/".to_string();

    directories.insert("/".to_string(), Vec::new());

    for l in lines {
        let args: Vec<&str> = l.split(" ").collect();

        match args[0] {
            "$" => {
                if args[1] == "cd" {
                    match args[2] {
                        "/" => {pwd = "/".to_string();},
                        ".." => {
                            pwd = parents.get(&pwd).unwrap().to_string();
                        },
                        _ => {
                            pwd = args[2].to_string();
                        }
                    }
                }
            },
            "dir" => {
                let dir_name = args[1].to_string();

                match directories.entry(pwd.clone()) {
                    Entry::Vacant(e) => {e.insert(vec![dir_name.clone()]);},
                    Entry::Occupied(mut e) => e.get_mut().push(dir_name.clone())
                }

                match parents.entry(dir_name.clone()) {
                    Entry::Vacant(e) => {e.insert(pwd.clone());},
                    Entry::Occupied(_) => ()
                }
            },
            a => {
                if a.parse::<i32>().is_ok() {
                    let size: i32 = a.parse().unwrap();
                    let name = args[1].to_string();
                    let file = File::new(&name, size);

                    match files.entry(pwd.clone()) {
                        Entry::Vacant(e) => {e.insert(vec![file]);},
                        Entry::Occupied(mut e) => e.get_mut().push(file)
                    }

                    *sizes.entry(pwd.to_owned()).or_default() += size;
                }
            }
        }
    }

    println!("{:?}", get_size(&"/".to_string(), &directories, &sizes));

    0
}

pub fn part_two() -> i32 {
    let lines = utils::files::get_lines("data/day07.txt");

    0
}