use std::fmt::{Display, Debug};

trait Summary {
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted {}", self.username, self.content)
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

/*fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}*/

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_multiple<T: Summary + Display>(item1: &T, item2: &T) {}

fn some_function<T, U>(t: &T, u: &U) 
    where T: Display + Clone, U: Clone + Debug
{}

fn return_basic_tweet() -> impl Summary {
    Tweet {
        username: "discoding".to_string(),
        content: "Hello World".to_string(),
        reply: false,
        retweet: false
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x is the largest: {}", self.x);
        } else {
            println!("y is the largest: {}", self.y);
        }
    }
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("Express Gradient"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());

    let news_article: NewsArticle = NewsArticle {
        headline: String::from("Elon Musk becomes the richest person on the planet"),
        content: String::from("This happened because of sudden increase in Tesla shares by 4.8%"),
        author: String::from("Express Gradient"),
        location: String::from("Remote"),
    };

    println!("{}", news_article.summarize());

    notify(&news_article);

    println!("{}", return_basic_tweet().summarize());
}