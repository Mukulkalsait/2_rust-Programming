pub fn combinators() {
    // ==========================================
    // 3. COMBINATORS
    // ==========================================
    println!("\n--- COMBINATORS ---");

    // chain(other): Concatenates two iterators
    let chain_res: Vec<i32> = (1..3).chain(7..9).collect();
    println!("chain(): {:?}", chain_res); // [1, 2, 7, 8]

    // zip(other): Pairs items into tuples
    let letters = vec!['a', 'b', 'c'];
    let nums = vec![1, 2, 3];
    let zip_res: Vec<(char, i32)> = letters.into_iter().zip(nums).collect();
    println!("zip(): {:?}", zip_res); // [('a', 1), ('b', 2), ('c', 3)]

    // enumerate(): Yields (index, value) tuples
    let items = vec!["first", "second"];
    let enum_res: Vec<(usize, &&str)> = items.iter().enumerate().collect();
    println!("enumerate(): {:?}", enum_res); // [(0, "first"), (1, "second")]
}
