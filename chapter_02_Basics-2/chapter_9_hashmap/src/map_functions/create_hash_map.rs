use std::collections::HashMap;

pub fn create_hash_map() -> [HashMap<String, i32>; 2] {
    let map_type_defined: HashMap<String, i32> = HashMap::new(); // 1. TYPE DEFINED
    let map_type_undefined = HashMap::new(); // 2. TYPE UN-Defined.
    [map_type_defined, map_type_undefined]
}

// Y: both are NON MUTABLE. WHY?
