use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    println!("Hello, world!");
    let post = SocialPost {
        username: String::from("horsey"),
        content: String::from("haru urara wa shinimashita"),
        reply: String::from("ehhh hontou???"),
        repost: true,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("A man has fallen into a river in Lego City"),
        location: String::from("Lego city"),
        author: String::from("Onion News Network"),
        content: String::from("Insert AI slop here..."),
    };
}
