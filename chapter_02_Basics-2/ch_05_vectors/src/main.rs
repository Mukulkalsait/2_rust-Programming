fn main() {
    ways_to_create_vectors();
    pushing_into_vector();
    accessing_from_vector();
    some_special_errors();
    iteration_on_vectors();
}

fn ways_to_create_vectors() {
    // B: General way.
    let vec1: Vec<i32> = Vec::new();
    println!("{:?}", vec1);

    // B: Macro style.
    let vec2: Vec<i8> = vec![1, 2, 3]; // Y: explisit. 
    println!("{:?}", vec2);

    let vec3 = vec![1, 2, 3]; // Y: non specified.
    println!("with values = {:?}", vec3);

    let vec3: Vec<i64> = Vec::new(); // Y: vec3 redecleared with different type.
    println!("redecleared = {:?}", vec3);
}

fn pushing_into_vector() {
    let mut vec1 = vec![];
    vec1.push(4); // IMP: this decided teh vectors type as we did not specified.
    println!("{:?}", vec1);

    let mut vec2 = Vec::new();

    vec2.push('a'); // IMP: this decided teh vectors type as we did not specified.
    vec2.push('b');
    vec2.push('c');
    vec2.push('d');
    println!("{:?}", vec2);

    let mut vec3 = vec![];
    vec3.push(3); // IMP: this decided teh vectors type as we did not specified.
    vec3.push(4);
    vec3.push(5);
    println!("{:?}", vec3);
}

fn accessing_from_vector() {
    let index_v = 3;

    let vec1 = vec![1, 2, 3, 4, 5];

    // IMP: A
    let third_element: &i32 = &vec1[2];
    println!("The third element in vec1 is = {}", third_element);
    // B: just to know if our third_element is present

    // IMP: B
    match vec1.get(index_v) {
        Some(third) => println!(
            "The {} th element is extracted successfully: {}",
            index_v + 1, // B: index start from 0. hence billow "index_v + 1"
            third
        ),
        None => println!("The {} th element is not present", index_v + 1),
    }

    // IMP: C
    let third_value = vec1.get(index_v);
    println!("{:?}", third_value);

    /* Y: EXPLANATION:
     *   ------------------------------------------
     *   A -> if A goes out of boundry = error. panic
     *   B -> if B goes out of boundry = nouthing happens.
     *   C -> its compact version of B used like A.
     */
}

fn some_special_errors() {
    let mut vec1 = vec![1, 2, 3, 4, 5];

    let first = &vec1[0];
    println!(
        "first will initiate and used here {} then droped if not used again.",
        first
    );

    // vec1.push(6);

    println!(
        "Now we are trying to get val of first here again = {}
but changing vector size might change the place of it inside heap.
This will create huge problem as the reference we take in 'first' will have wrong pointers,
if the orignal vec1 moved to another place deu to low space. 
rust will not let that happen.",
        first
    );
}

fn iteration_on_vectors() {
    let vec1 = vec!['a', 'g', '😎', '9', ' ', '_'];

    for val in &vec1 {
        println!("{}", val);
    }
    println!("{:?}", vec1);
}
