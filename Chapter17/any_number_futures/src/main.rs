use std::{thread, time::Duration};
use trpl::Either;

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms} ms");
}

// Because you only have two await points, it'll only switch
// after a finishes
fn mock_concurrency() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started");
            slow("b", 30);
            slow("b", 10);
            slow("b", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished");
        };

        trpl::select(a, b).await;
    });
}

// 3 await points => proper interleaving!
fn sleep_async_concurrency() {
    let one_ms = Duration::from_millis(1);

    trpl::block_on(async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started");
            slow("b", 30);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 20);
            trpl::sleep(one_ms).await;
            println!("'b' finished");
        };

        trpl::select(a, b).await;
    });
}

// Use yield now to speed up execution
fn async_concurrency() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started");
            slow("b", 30);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 20);
            trpl::yield_now().await;
            println!("'b' finished");
        };

        trpl::select(a, b).await;
    });
}

// If the future takes longer than max time, return an error
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn try_future() {
    trpl::block_on(async {
    // Closure, returns the string after waiting
    let slow = async {
        // Customize the duration to determine whether the function fails 
        // or passes
        trpl::sleep(Duration::from_secs(5)).await;
        "Finally finished"
    };
    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }

    }});
}

fn main() {
    // async_concurrency(); 
    try_future();
}
