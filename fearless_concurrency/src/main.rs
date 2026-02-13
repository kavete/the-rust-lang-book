use std::thread;
use std::time::Duration;
pub mod message_passing;

pub mod shared_state;

fn main() {
    let some_vector = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi {i} from spwaned thread.");
            println!("Some vec {:?}", some_vector);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("Hi {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    message_passing::msg_passing();
    message_passing::multi_producer();

    shared_state::share_mutex();

    handle.join().unwrap();
}
