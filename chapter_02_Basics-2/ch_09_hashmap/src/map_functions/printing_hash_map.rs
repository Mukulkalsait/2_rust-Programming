use std::collections::HashMap;

pub fn access_map_values(map: &HashMap<String, i32>) {
    let key = "alice";
    if let Some(score) = map.get(key) {
        println!("Found: {} Score is {}", key, score);
    } else {
        println!("{} not found.", key);
    }
}

pub fn for_loop_in_hash_map(map: &HashMap<String, i32>) {
    for (key, value) in map {
        println!("{} : {}", key, value);
    }
}

pub fn print_hash_methods(map: &HashMap<String, i32>) {
    // G: printing direct.
    println!("{:?}", map);

    // G: accessing values.
    access_map_values(map);

    for_loop_in_hash_map(map);
}

/* Y: MUTATION IN HashMap
 * ---------------------------------------------------------------|
 * |  L-18  pub fn print_hash_methods(map: &HashMap<String, i32>)
 * |        map: &HashMap
 * |        & is connected to HashMap.
 * ---------------------------------------------------------------|
 */
