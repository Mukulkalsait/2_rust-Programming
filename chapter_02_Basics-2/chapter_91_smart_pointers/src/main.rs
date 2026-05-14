fn test_boxing() {
    let mut a: String = String::from("hello");
    println!("a: {:?}", a);
    let mut b: Box<String> = Box::new("hello".to_string());
    println!("b: {:?}", b);

    let c = b;
    println!("c: {:?}", c);

    let d = a;
    println!("d: {:?}", d);

    // R: deref will not work if we never REFERENCED...
    //
    // println!("deref b: {:?}", *b); // says b vaalue borroed after move
    // println!("deref b: {:?}", *a); // traits size not allowed for  str.
    //
    // let array1: [&str; 4] = ["234", "234", "4asdf", "weiabe"];
    // let array2: Box<[&str; 4]> = Box::new(["234", "234", "4asdf", "weiabe"]);
}

#[derive(Debug)]
enum List {
    // Cons(i32, List), // R: fail infinite
    Cons(i32, Box<List>),
    Nil,
}

/// observation why we need Box<T>
/// how the recursion is doing , and hnow is it           perfect....
fn infinite_loop_list(list: &List) {
    println!("List => \n{:?}", list);
}

fn print_list(l: &mut List) {
    match l {
        List::Cons(val, next) => {
            println!("vaalue: {}", val);
            print_list(next);
        }
        List::Nil => {
            println!("end");
        }
    }
}

fn main() {
    let mut list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))))))));

    test_boxing();
    infinite_loop_list(&list);
    print_list(&mut list);
}
