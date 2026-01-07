use std::time::Duration;

fn main() {
    // alt_print();    
    message_passing();
}

fn message_passing() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}

fn message_passing_one_prod() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        // The move here is necessary to let the program terminate
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

// This version doesn't have true concurrency, as it only has one async 
// block. There's nothing to alternate between!
fn message_passing_naive() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

fn message_passing_simple() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

fn alt_print() {
    trpl::block_on(async {
        let fut1 = async { trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from task 1!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        })};

        let fut2 = async { for i in 1..10 {
                println!("hi number {i} from task 2!");
                trpl::sleep(Duration::from_millis(500)).await;
        }};
        // tprl is fair - each future is checked equally!
        trpl::join(fut1, fut2).await;
    });
}