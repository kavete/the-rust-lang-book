fn main() {
    // Match arms

    let x = Some(5);

    let result = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    println!("Result {:?}", result);

    // Conditional if let statements

    let favourite_color: Option<&str> = None;
    let is_tuesday = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using the color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the bg color");
        }
    } else {
        println!("Using blue as the bg color");
    }

    // While let conditional loops

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // For loops

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // Function parameters

    let point = (5, 10);

    print_coordinates(&point);

    // Refutability
    // Irrefutable - Patterns that will match for any possible values

    let _irrefutable = 5;

    // Refutable - Values that can fail to match for some values

    let refutable: Option<i32> = None;

    if let Some(x) = refutable {
        println!("{x}");
    }

    // Pattern syntax

    // Matching literals

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    // Matching named variables

    let x1 = Some(5);
    let y1 = 10;

    match x1 {
        Some(50) => println!("Got 50"),
        Some(y1) => println!("Matched y1 = {y1}"),
        _ => println!("Default case x1 = {:?}", x1),
    }

    println!("At the end x1 = {x1:?} y1 = {y1}");

    // Matching multiple patterns
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current coordinates ({x}, {y})");
}
