use crate::utils;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::vec;

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

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    parent: Option<Box<&'a mut Directory<'a>>>,
    files: Vec<File>,
    dirs: Vec<Directory<'a>>
}

impl<'a> Directory<'a> {
    pub fn new(name: &String, parent: Option<Box<&'a mut Directory<'a>>>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            files: vec![],
            dirs: vec![]
        }
    }

    pub fn add_dir(&mut self, dir: Directory<'a>) {
        self.dirs.push(dir);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

fn get_size(dir: &String, dirs: &HashMap<String, Vec<String>>, sizes: &HashMap<String, i32>) -> i32 {
    let mut total_size = 0;

    if dirs.contains_key(dir) {
        let children = dirs.get(dir).unwrap();

        for d in children {
            total_size += get_size(&d, dirs, sizes);
        }
    }

    total_size += sizes.get(dir).unwrap();

    total_size
}

pub fn part_one() -> i32 {
    let lines = utils::files::get_lines("data/day07.txt");

    let mut total = 0;

    let mut directories: HashMap<String, Vec<String>> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut files: HashMap<String, Vec<File>> = HashMap::new();
    let mut sizes: HashMap<String, i32> = HashMap::new();

    let mut pwd: String = "/".to_string();

    directories.insert("/".to_string(), Vec::new());

    let mut root = Directory::new(&"/".to_string(), None);
    let parent = Some(Box::new(&mut root));

    root.add_dir(Directory::new(&"a".to_string(), parent));
    println!("{:?}", root);

    // for l in lines {
    //     println!("{}", l);
    //     let args: Vec<&str> = l.split(" ").collect();

    //     match args[0] {
    //         "$" => {
    //             if args[1] == "cd" {
    //                 match args[2] {
    //                     "/" => {pwd = "/".to_string();},
    //                     ".." => {
    //                         println!("{:?}, {:?}", pwd, parents);
    //                         pwd = parents.get(&pwd).unwrap().to_string();
    //                     },
    //                     _ => {
    //                         pwd = args[2].to_string();
    //                     }
    //                 }
    //             }
    //         },
    //         "dir" => {
    //             let dir_name = args[1].to_string();

    //             match directories.entry(pwd.clone()) {
    //                 Entry::Vacant(e) => {e.insert(vec![dir_name.clone()]);},
    //                 Entry::Occupied(mut e) => e.get_mut().push(dir_name.clone())
    //             }

    //             match parents.entry(dir_name.clone()) {
    //                 Entry::Vacant(e) => {e.insert(pwd.clone());},
    //                 Entry::Occupied(_) => ()
    //             }
    //         },
    //         a => {
    //             if a.parse::<i32>().is_ok() {
    //                 let size: i32 = a.parse().unwrap();
    //                 let name = args[1].to_string();
    //                 let file = File::new(&name, size);

    //                 match files.entry(pwd.clone()) {
    //                     Entry::Vacant(e) => {e.insert(vec![file]);},
    //                     Entry::Occupied(mut e) => e.get_mut().push(file)
    //                 }

    //                 *sizes.entry(pwd.to_owned()).or_default() += size;
    //             }
    //         }
    //     }
    // }

    // for (d, s) in &sizes {
    //     let size = get_size(&d, &directories, &sizes);
    //     if size <= 100000 {
    //         total += size;
    //     }
    // }

    total
}

pub fn part_two() -> i32 {
    let lines = utils::files::get_lines("data/day07.txt");

    0
}