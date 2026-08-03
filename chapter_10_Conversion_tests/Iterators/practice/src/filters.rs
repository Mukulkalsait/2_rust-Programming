pub fn filter() {
    // ==========================================
    // 2. FILTERING & MAPPING
    // ==========================================
    println!("\n--- FILTERING & MAPPING ---");

    // filter(predicate): Keeps items satisfying condition
    let filter_res: Vec<i32> = (1..6).filter(|x| x % 2 == 0).collect();
    println!("filter(even): {:?}", filter_res); // [2, 4]

    // map(f): Transforms each item
    let map_res: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 10).collect();
    println!("map(*10): {:?}", map_res); // [10, 20, 30]

    // filter_map(f): Transforms and filters out None
    let strings = vec!["10", "abc", "30"];
    let filter_map_res: Vec<i32> = strings.into_iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("filter_map(parse): {:?}", filter_map_res); // [10, 30]

    // flat_map(f): Maps each item to an iterator and flattens
    let words = vec!["hi", "rust"];
    let flat_map_res: Vec<char> = words.into_iter().flat_map(|w| w.chars()).collect();
    println!("flat_map(chars): {:?}", flat_map_res); // ['h', 'i', 'r', 'u', 's', 't']

    // flatten(): Flattens nested iterators or Options
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flatten_res: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten(): {:?}", flatten_res); // [1, 2, 3, 4]
}
