use trpl::{Either, Html};



// Compiles into (approximately) this
// use std::future::Future;
// fn __page_title(url: &str) -> impl Future<Output = Option<String>> {
//     // Move captures a closure's environment by value => the block
//     // is actually a function definition!
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

// The state machine it uses:
// enum PageTitleFuture<'a> {
//     Initial { url : &'a str },
//     GetAwaitPoint { url : &'a str }, 
//     TextAwaitPoint { response : trpl::Response },
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        
        println!("{url} returned first!");
        match maybe_title {
            Some(title) => println!("Its title was \"{title}\""),
            None => println!("No title found"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await.text().await;
    // Gets the inner HTML of the title
    let title = Html::parse(&response)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}