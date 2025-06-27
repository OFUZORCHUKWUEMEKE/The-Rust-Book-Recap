use std::fmt::{Debug, Display};

pub trait Summary {
    fn summary(&self) -> String;

    fn summarize_author(&self) -> String {
        format!("@{}", self.summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{} , by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summary())
}

pub fn genericc<T: Summary>(item: &T) {}

pub fn notifys<T: Summary + Display>(item: &T) {}
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn some_func<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialPost {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
