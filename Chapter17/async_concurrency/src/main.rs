use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from task 1!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..10 {
                println!("hi number {i} from task 2!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        }
    );
}
