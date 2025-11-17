pub mod strings {

    pub fn play_with_strings() {
        let mut s = String::new();

        let data = "initial contents";

        let s2 = data.to_string();

        println!("{s2}");

        let s3 = String::from("Hey What's cooking");

        // Updating strings

        let mut game = String::from("foot");

        game.push_str("ball");

        println!("{game}");

        let mut lol = String::from("lol");

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

        for b in "Здр".bytes() {
            println!("{b}");
        }
    }
}
