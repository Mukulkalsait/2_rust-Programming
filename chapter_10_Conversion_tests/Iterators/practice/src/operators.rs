pub fn operators() {
    // ==========================================
    // 1. OPERATORS
    // ==========================================
    println!("--- OPERATORS ---");

    // take(n): Takes first n elements
    let take_res: Vec<i32> = (1..10).take(3).collect();
    println!("take(3): {:?}", take_res); // [1, 2, 3]

    // take_while(predicate): Takes elements until condition fails
    let take_while_res: Vec<i32> = vec![1, 2, 5, 1, 3].into_iter().take_while(|&x| x < 4).collect();
    println!("take_while(<4): {:?}", take_while_res); // [1, 2]

    // skip(n): Ignores first n elements
    let skip_res: Vec<i32> = (1..6).skip(2).collect();
    println!("skip(2): {:?}", skip_res); // [3, 4, 5]

    // skip_while(predicate): Skips until condition fails, then yields the rest
    let skip_while_res: Vec<i32> = vec![1, 2, 5, 1, 3].into_iter().skip_while(|&x| x < 4).collect();
    println!("skip_while(<4): {:?}", skip_while_res); // [5, 1, 3]

    // step_by(n): Yields every n-th element
    let step_by_res: Vec<i32> = (0..10).step_by(3).collect();
    println!("step_by(3): {:?}", step_by_res); // [0, 3, 6, 9]

    // peekable(): Look ahead using .peek() without advancing
    let mut peekable = vec![10, 20, 30].into_iter().peekable();
    if let Some(&first) = peekable.peek() {
        println!("peeked first element: {}", first); // 10
    }
    println!("next element consumed: {:?}", peekable.next()); // Some(10)

    // rev(): Reverses iteration direction
    let rev_res: Vec<i32> = (1..4).rev().collect();
    println!("rev(): {:?}", rev_res); // [3, 2, 1]

    // cycle(): Repeats sequence infinitely (used with take to prevent infinite loop)
    let cycle_res: Vec<i32> = vec![1, 2].into_iter().cycle().take(5).collect();
    println!("cycle().take(5): {:?}", cycle_res); // [1, 2, 1, 2, 1]
}

/// use caes of peekable is to peek before consume, hwere we check if its uumnbr or not if not then
/// we dont consume if its number we consume.
pub fn peekker() {
    let input = "123945-23abc456";
    // We make a character iterator peekable
    let mut chars = input.chars().peekable();

    // Loop through the input stream
    while let Some(&ch) = chars.peek() {
        if ch.is_numeric() {
            // We peeked and saw a number! Now we consume it.
            let num = chars.next().unwrap();
            print!("{}", num);
        } else {
            // We peeked and saw 'a' (not a number).
            // We break WITHOUT consuming 'a' so another parser can process it.
            println!(" <-- Parsed complete number!");
            break;
        }
    }

    // 'chars' still has 'a' as its very next item!
    println!("Next remaining item in stream: {:?}", chars.next()); // Some('a')
}

pub fn consicitive_duplicate_detector() {
    let data = vec!['a', 'a', 'a', 'b', 'b', 'c', 'a', 'a', 'b'];
    let mut iter = data.into_iter().peekable();

    while let Some(current) = iter.next() {
        let mut count = 1;

        // Look at the NEXT item without consuming it yet
        while iter.peek() == Some(&current) {
            count += 1;
            iter.next(); // Consume the duplicate
        }

        println!("Character '{}' appeared {} times", current, count);
    }
}
