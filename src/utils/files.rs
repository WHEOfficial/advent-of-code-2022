use std::fs;

pub fn get_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("File could not be read");
    
    contents.split("\n").map(|s| s.to_string()).collect()
}