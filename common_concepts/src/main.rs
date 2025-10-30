// Common programming concepts
//

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5; // Declare a mutable variable x
    println!("The value of x is: {x}");
    x = 6; // Raises an errror if  x is immutable

    println!("The value of x is: {}", x);

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing

    let to_shadow = 5;

    let to_shadow = to_shadow + 1;

    {
        let to_shadow = to_shadow * 2;
        println!("The value in the inner scope is {to_shadow}"); // Ouputs 12
    }

    println!("The value in the outerscope is: {to_shadow}"); // Outputs 6

    // Shadowing alows data type change

    let spaces = " "; // String slice

    let spaces = spaces.len(); // It's now an integer (usize)

    println!("Spaces: {spaces}"); // Ouputs spaces
    //

    scalar_data_types();
    compound_data_types();
    height_with_units(5, 'm');

    let five = five();
    println!("Five: {five}");

    control_flow();

    println!("Yoh");
}

fn scalar_data_types() {
    let x: f64 = 2.0;

    let y: f64 = 3.0;

    println!("x and y: {x} and {y}");

    let difference = y - x; // They must both be either f64 or both f32

    println!("y -x {difference}");

    let quotient = y / x; // Returns 1.5 since they are floats 

    println!("Quotient: {quotient}");

    let truncated = -5 / 3; //The value returned is -1
    println!("Truncated: {truncated}");

    let active = true;

    println!("Is active: {active}");

    let active: bool = false; // shadows it to true

    println!("Is active: {active}");

    let _character = 'z'; // Characters use single quotes just like in C
}

fn compound_data_types() {
    // Tuples

    let mut numbers = (500, 6.4, 1);

    //Destructuring a tuple

    let (_x, y, _z) = numbers;

    println!("The value of y is: {y}");

    let five_hundred = numbers.0;

    println!("Five hundred: {five_hundred}");

    numbers.0 = 4; // Can only be mutated to the type of initial value

    println!("the tuple is {:?}", numbers);

    // Arrays

    let days = [1, 2, 3, 4, 5, 6, 7];

    let a: [i32; 5] = [3; 5];

    println!("The array a is {:?}", a);

    println!("Second day: {}", days[1]);
}

fn height_with_units(height: u32, unit: char) {
    println!("The height is: {height} {unit}");
}

fn five() -> i32 {
    5 // Placing a semicolon here turns it to a statement producing an error
}

// Control flow

fn control_flow() {
    let mut number = 3;

    if number % 5 == 0 {
        println!("The number is divisible by 5");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3 but not by 5");
    } else {
        println!("the number is divisible by 2 but not by 5 or 3")
    }

    let real_number = if number >= 0 { true } else { false };

    println!("Real number: {real_number}");

    // Loops
    //

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // Loop labels

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Breaks the inner loop only
            }
            if count == 2 {
                break 'counting_up; // Breaks the counting_up loop
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
