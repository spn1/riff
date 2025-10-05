use std::collections::{HashMap, HashSet};

pub struct Diff {
    unique_a: HashMap<String, String>,
    unique_b: HashMap<String, String>,
    diff: HashMap<String, (String, String)>
}

pub fn get_diff(map_a: &HashMap<String, String>, map_b: &HashMap<String, String>) {
    let keys_a: HashSet<String> = map_a.keys().cloned().collect();
    let keys_b: HashSet<String> = map_b.keys().cloned().collect();

    // Get key/values that are unique in map_a and map_b
    let unique_a = get_unique(&map_a, keys_b);
    let unique_b = get_unique(&map_b, keys_a);

    println!("unique a: {:?}", unique_a);
    println!("unique b: {:?}", unique_b);
    
    // Get keys that are the same between map_a and map_b 
    let common = get_common(map_a, map_b);

    println!("Common: {:?}", common);

}

fn get_unique(left: &HashMap<String, String>, right: HashSet<String>) -> HashMap<&String, &String> {
    let mut unique: HashMap<&String, &String> = HashMap::new();
    
    for (key, value) in left.iter() {
        match right.contains(key) {
            true => continue,
            false => unique.insert(key, value)
        };
    }

    unique
}

fn get_common<'a, 'b>(left: &'a HashMap<String, String>, right: &'b HashMap<String, String>) -> HashMap<&'a String,(&'a String, &'b String)> {
    let mut common: HashMap<&String, (&String, &String)> = HashMap::new();

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