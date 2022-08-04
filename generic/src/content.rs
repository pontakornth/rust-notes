use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub body: String,
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct ArticleWithLifetime<'a> {
    pub headline: &'a str,
    pub location: &'a str,
    pub author: &'a str,
    pub content: &'a str,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

impl<'a> Summary for ArticleWithLifetime<'a> {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.body)
    }
}


// You can have some function accepts a type that implement a trait.
pub fn summarize(a: &impl Summary) -> String {
    a.summarize()
}

pub fn summarize_generic<T: Summary>(a: &T) -> String {
    a.summarize()
}

// (a: &impl Summary, b: &impl Summary) allows different type that implement Summary
// The generic one forces the same type.
//
// In case that some traits are long, use where
//
fn weird_function<T,U>(a: &T, b: &U) -> () where T: Display + Clone, U: Display + Clone + PartialOrd {}


