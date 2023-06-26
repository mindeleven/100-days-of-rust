/// chapter 10, traits, defining shared behavior
/// a trait defines the functionality a particular type has and an share with other types
/// different types share the same behavior if we can call the same methods on them
/// 
/// defining a trait
pub trait Summary {
    // defining the signatures on the methods
    fn summarize(&self) -> String;
}

/// implementing a trait type
/// example: news article & tweet that both should make use of a summarize functionality
/// (1) news article
pub struct NewsArticle {
    pub headline: String,
    pub loation: String,
    pub author: String,
    pub content: String,
}

/// implementing the Summary trait on the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.loation
        )
    }
}

/// (2) tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/// implementing the Summary trait on the Tweet struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}",
            self.username,
            self.content
        )
    }
}

fn main() {
    // calling the trait methods on the instances of the struct
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        headline: String::from("Down and Out"),
        loation: String::from("The Sprawl"),
        author: String::from("Dixie Flatline"),
        content: String::from("No new tale to tell")
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
}
