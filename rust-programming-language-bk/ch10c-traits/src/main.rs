/// chapter 10, traits, defining shared behavior
/// a trait defines the functionality a particular type has and an share with other types
/// different types share the same behavior if we can call the same methods on them
/// 
/// defining a trait
pub trait Summary {
    // defining the signatures on the methods
    fn summarize(&self) -> String;
}

/// defining another trait for multiple trait bounds example
pub trait Display {
    // defining the signatures on the methods
    fn display(&self) -> String;
}

/// traits with default implementations
pub trait Teaser {
    fn summarize_author(&self) -> String;

    // default implementations can call methods in the same trait that
    // don't have default implementations
    fn summarize_teaser(&self) -> String {
        format!(
            "Read more from {}....",
            self.summarize_author()
        )
    }

    fn teaserize(&self) -> String {
        String::from("(Read more...)")
    }
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

/// implementing the Teaser trait on the NewsArticle struct
impl Teaser for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

/// implementing the Teaser trait on the Tweet struct
/// default implementation can be overwritten
impl Teaser for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn teaserize(&self) -> String {
        String::from("Read more from ") + &self.username.to_string()
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
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        loation: String::from("Pittsburgh, PA, USA"),
        author: String::from("Dixie Flatline"),
        content: String::from("No new tale to tell, anything happened as expacted.")
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.teaserize());
    println!("Teaserize with tweet: {}", tweet.teaserize());
    println!("Teaser new tweet: {}", tweet.summarize_teaser());

}

/// traits as parameters
/// no concrete type for item parameter 
/// but impl keyword and trait name
/// and instance of NewsArticle or Tweet could be passed
/// types that don't implement Summary won't compile
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// longer form of the example above ("trait bound syntax"):
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// function with two parameters that implement summary
/// function can have different implementations of summary as parameters
pub fn notify_v3(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
    println!("More breaking news! {}", item2.summarize());
}

/// trait bound syntax is needed if we want to force both parameters
/// to have the same type
pub fn notify_v4<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("More breaking news! {}", item2.summarize());
}

/// specifying multiple trait bounds with the + syntax
pub fn notify_v5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify_v6<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


