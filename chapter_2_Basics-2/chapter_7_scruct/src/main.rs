#![allow(dead_code)] // at the top.

fn struct_learning() {
    structs_basics_1();
    structs_basics_2();
}

fn structs_basics_1() {
    // Y: structs_basics_1: prepratoins-------------

    #[derive(Debug)] // Without this {:?} will not work in struct.
    struct Person {
        surname: String,
        house_no: u8,
    }

    // Y:  Structs in Structs
    #[derive(Debug)] // Without this {:?} will not work in struct.
    struct PersonCreated1 {
        name: String,
        age: u16,
        info: Person,
    }

    //-----------------------------------------------

    let peter = PersonCreated1 {
        name: String::from("Peter"),
        age: 27,
        info: Person {
            surname: String::from("Parker"),
            house_no: 104,
        },
    };

    // Print debug struct
    println!("{:?}", peter); // Working
}

fn structs_basics_2() {
    // Y: structs_basics_2: prepratoins-------------

    struct Unit; // A unit struct
    struct Pair(i32, f32); // A tuple struct
    struct Point {
        x: f32,
        y: f32,
    }

    // Y:  Structs in Structs : Coordinates of Rectangle using 'Point 👆🏽'
    struct Rectangle {
        top_left: Point,
        // top_right: Point,
        // bottom_left: Point,
        bottom_right: Point,
    }

    //-----------------------------------------------

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 }; // decleared : Point
    let another_point = Point { x: 10.3, y: 0.2 }; // auto detected : Point

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // IMP: Destructure the point using a `let` binding
    // here we dectruction struct with
    //   Point { x:<name-a>, y:<name-b> } = point(name of var struct)
    //   so this will take vallues of
    //      point.x => name-a
    //      point.y => name-b

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn advanced_standard_fmt_1() {
    // This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`.
    #[allow(dead_code)]
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation required to make this `struct` printable with `fmt::Debug`.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}

fn main() {
    struct_learning();
    // advanced_standard_fmt_1();
}
