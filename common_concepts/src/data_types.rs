
pub fn scalar_data_types() {
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
