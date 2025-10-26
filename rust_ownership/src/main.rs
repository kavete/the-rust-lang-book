//  Safety == Absence of undefined behaviour
// Safe

fn main() {
    let x = true;
    read(x); // Moving the call to read before x is defined makes the program unsafe
}

fn read(y: bool) {
    if y {
        println!("True");
    }
}
