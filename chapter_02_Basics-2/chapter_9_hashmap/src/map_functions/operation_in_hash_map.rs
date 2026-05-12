use std::collections::HashMap;

pub fn remove_item_from_hash_map(map: &mut HashMap<String, i32>) {
    // Y: insertion.
    map.insert(String::from("alice"), 15);
    map.insert(String::from("alice_2"), 55);
    map.insert(String::from("alice_3"), 35);
    map.insert(String::from("bob"), 12);
    map.insert(String::from("Mukul"), 99);

    // Y: removal
    map.remove("alice");

    // Y: Conditinoal Insertion:
    // .entry() => check key
    // .or_insert() -> if not found add.
    map.entry(String::from("Mukul")).or_insert(99);
    map.entry(String::from("Shruti")).or_insert(75);
    map.entry(String::from("noOne")).or_insert(12);
}
