pub fn interceptors() {
    // ==========================================
    // 4. INTERCEPTION, REFERENCES & STATE
    // ==========================================
    println!("\n--- INTERCEPTION, REFERENCES & STATE ---");

    // inspect(f): Pass-through inspection for debugging
    let inspect_res: Vec<i32> = (1..4).inspect(|x| println!("  [inspect log]: item = {}", x)).map(|x| x * 2).collect();
    println!("inspect() completed: {:?}", inspect_res);

    // scan(initial_state, f): Maintains internal state across steps (e.g., running total)
    let scan_res: Vec<i32> = vec![1, 2, 3, 4]
        .into_iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("scan(running sum): {:?}", scan_res); // [1, 3, 6, 10]

    // copied(): Converts &T to T via Copy trait
    let arr = [10, 20, 30];
    let copied_res: Vec<i32> = arr.iter().copied().collect();
    println!("copied(): {:?}", copied_res); // [10, 20, 30] (owned ints)

    // cloned(): Converts &T to T via Clone trait
    let string_refs = vec![String::from("a"), String::from("b")];
    let cloned_res: Vec<String> = string_refs.iter().cloned().collect();
    println!("cloned(): {:?}", cloned_res); // ["a", "b"] (owned Strings)

    // by_ref(): Borrows an iterator mutably to reuse it afterwards
    let mut numbers = 1..10;
    let first_three: Vec<i32> = numbers.by_ref().take(3).collect();
    let remainder: Vec<i32> = numbers.collect();
    println!("by_ref() consumed first: {:?}", first_three); // [1, 2, 3]
    println!("by_ref() remainder left: {:?}", remainder); // [4, 5, 6, 7, 8, 9]

    // fuse(): Ensures iterator keeps yielding None once finished
    struct CustomFlakyIter {
        count: i32,
    }
    impl Iterator for CustomFlakyIter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            match self.count {
                1 => Some(1),
                2 => None,      // Returns None once
                3 => Some(999), // Bad iterator behavior: returning Some after None!
                _ => None,
            }
        }
    }

    let mut fused = CustomFlakyIter { count: 0 }.fuse();
    println!("fuse() call 1: {:?}", fused.next()); // Some(1)
    println!("fuse() call 2: {:?}", fused.next()); // None
    println!("fuse() call 3: {:?}", fused.next()); // None (guaranteed None due to fuse)
}
