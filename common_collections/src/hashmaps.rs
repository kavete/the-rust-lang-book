use std::collections::HashMap;

pub fn playing_with_hashmaps() {
    //
    // Creating a new hashmap

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);

    // Accessing values in a hashmap

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score:?}");

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    // Hashmaps and ownership

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value); // field_value and field_name are moved into the
    // hashmap
    //
    // Overwriting values

    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Adding only if a key isn't present

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{scores:?}");

    // Updating a value based on the old value
    //

    let text = "hello world wonderful world";

    let mut occurances = HashMap::new();

    for word in text.split_whitespace() {
        let count = occurances.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{occurances:?}");
}
