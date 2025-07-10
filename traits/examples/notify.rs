pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }
    // fn summarize(&self) -> String {
    //     return format!("{}, by {}", self.headline, self.author);
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        return format!("Read more from {}... ", self.summarize_author());
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {}
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
    notify(&article);
    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
