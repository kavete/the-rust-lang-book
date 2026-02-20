use std::time::Duration;

pub fn concurrent() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi {i} from task 1");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("Hi {i} from main task");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}

pub fn join_async() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("Hi {i} from fut1");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Hi {i} from fut2");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await;
    });
}

pub fn message_passing() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("Hi");

        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();

        println!("received: {received}");
    });
}
