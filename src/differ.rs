use std::collections::{HashMap, HashSet};

/// Represents the differents between the two files
#[derive(Debug, Default)]
pub struct Diff<'a> {
    pub unique_a: HashMap<&'a str, &'a str>,
    pub unique_b: HashMap<&'a str, &'a str>,
    pub diff: HashMap<&'a str, (&'a str, &'a str)>
}

/// Returns a Diff that contains the different between the two files / maps
pub fn get_diff<'a>(map_a: &'a HashMap<String, String>, map_b: &'a HashMap<String, String>) -> Diff<'a> {
    let keys_a: HashSet<String> = map_a.keys().cloned().collect();
    let keys_b: HashSet<String> = map_b.keys().cloned().collect();

    // Get key/values that are unique in map_a and map_b
    let unique_a = get_unique(map_a, keys_b);
    let unique_b = get_unique(map_b, keys_a);
    
    // Get keys that are different between map_a and map_b 
    let diff = get_common(map_a, map_b);

    Diff {
        unique_a,
        unique_b,
        diff
    }
}

/// Returns a HashMap of the unique keys / values that exist in 'left' but not 'right'
fn get_unique(left: &HashMap<String, String>, right: HashSet<String>) -> HashMap<&str, &str> {
    let mut unique: HashMap<&str, &str> = HashMap::new();
    
    for (key, value) in left.iter() {
        match right.contains(key) {
            true => continue,
            false => unique.insert(key, value)
        };
    }

    unique
}

/// Returns a HashMap of the keys that are found in both files.
/// The resulting HashMap key is the common key, and the value is a tuple
/// containing the 'left' & 'right' values.
fn get_common<'a, 'b>(left: &'a HashMap<String, String>, right: &'b HashMap<String, String>) -> HashMap<&'a str,(&'a str, &'b str)> {
    let mut common: HashMap<&str, (&str, &str)> = HashMap::new();

    for (key, left_value) in left.iter() {
        if let Some(right_value) = right.get(key) {   
            match left_value == right_value {
                true => continue,
                false => common.insert(key, (left_value, right_value))
            };
        }
    }

    common
}