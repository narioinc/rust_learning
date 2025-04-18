//using the library in the binary crate
use mylib::{Tweet, NewsArticle, Summary, MessageBoard};
use mylib::{notify, func1};

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("user1"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Doe"),
        content: String::from("This is the content of the news article."),
    };

   

    println!("Tweet: {}", tweet.summarize());
    println!("News Article: {}", news_article.summarize());

    let message_board = MessageBoard::ANNOUNCEMENTS(String::from("New announcement!"));
    println!("Message Board: {}", message_board.summarize());

    notify(&tweet);
    notify(&news_article);
    notify(&message_board);

    let result = func1();
    println!("Result: {}", result.summarize());

    // This shouldnt work, since the compiler will complain that you can only access the results of a 
    // Summary + Display type since func1 returns a type that implements both traits
    //println!("Result: {}", result.display);
}
