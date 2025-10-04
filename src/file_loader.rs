use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

const SEPARATOR: char = '=';

/// Loads a file at the given path and returns the contents as a String
pub fn read_file(filename: &str) -> Result<HashMap<String, String>> {
    let file_contents = load_file(filename)?;
    let lines = file_contents.lines();
    let mut map: HashMap<String, String> = HashMap::new();

    for line in lines {
        let key_value = line.split_once(SEPARATOR);

        match key_value {
            Some((key, value)) => map.insert(key.to_owned(), value.to_owned()),
            _ => None // Ignore comments 
        };
    }

    Ok(map)
}

fn load_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}