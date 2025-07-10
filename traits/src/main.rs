pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("abir"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Nadia"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
