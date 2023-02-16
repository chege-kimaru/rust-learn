pub trait Summary {
    // implementation is required
    // fn summarize(&self) -> String;

    // default implementation
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)");
    // }

    fn summarize_author(&self) -> String;

    // default implmentation calling non-default implementation
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
// fn summarize(&self) -> String {
//     format!("{}, by {} ({})", self.headline, self.author, self.location)
// }
// }

// To use default implementation
// impl Summary for NewsArticle {}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}