//  Safety == Absence of undefined behaviour
// Safe

fn main() {
    let x = true;
    read(x); // Moving the call to read before x is defined makes the program unsafe

    //Frame -- A mapping from variables to values within a single scope
    let n = 5;

    let _n2 = n; //  n is copied into n2 - This is only possible for simple data types

    let y = plus_one(n);
    println!("The value of y is: {y}");

    // Data can live indefinitely in the heap

    let a = Box::new([0, 1_000_000]);

    let b = a;

    println!("{:?}", b);

    // println!("{:?}", a);  This will not work since the box has been moved out of a into b

    // A boxes owner manages deallocation

    // Almost correct explanation
    // If a variable is bound to a box, when Rust deallocated the variable's frame, then rust
    // deallocates the box's heap memory

    let _a_num = 4;

    make_and_drop(); // After running this function, Rust deallocated both the frame of a_box and
    // the heap

    // Binding two variables to a a box
    //

    let some_box = Box::new([0; 1_000_000]); // some_box owns the box

    let _another_box = some_box; // ownership has moved to another_box
    //
    // Correct explanation
    // If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates
    // the box's heap memory

    // Collections (Vec, String, HashMap) use boxes

    let first = String::from("Ferris");

    let full = add_suffix(first);

    println!("{full}");

    // println!("{first}");  This does not work
    //
    // Cloning avoids Moves

    let name = String::from("Fyodor");
    let name_clone = name.clone();

    let full_name = add_suffix(name_clone);

    println!("{full_name} originally {name}");
}

fn read(y: bool) {
    if y {
        println!("True");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn make_and_drop() {
    let _a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr");
    name
}
