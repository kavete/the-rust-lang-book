pub fn play_with_strings() {
    // Create a new empty string
    let mut s = String::new();

    // Create a string literal
    let data = "initial contents";

    // Convert a string literal &str to String
    let s2 = data.to_string();

    println!("{s2}");

    // Create a string directly from a literal
    let s3 = String::from("Hey What's cooking");

    // Updating strings
    let mut game = String::from("foot");

    // Push a string
    game.push_str("ball");

    // game == football
    println!("{game}");

    let mut lol = String::from("lol");

    // Push a single character
    lol.push('!');

    println!("{lol}");

    // Concatanation
    //
    let random_text = lol + &game;

    // lol is no longer usable as it has been moved to random_text
    // Trying to print it will lead to an error

    // println!("{lol}");

    println!("{random_text}");

    // Using format! macro

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let tictactoe = format!("{t1}-{t2}-{t3}");

    println!("{tictactoe}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // Iterating over strings

    for c in "Здр".chars() {
        println!("{c}");
    }

    // Will print the bytes
    for b in "Здр".bytes() {
        println!("{b}");
    }
}
