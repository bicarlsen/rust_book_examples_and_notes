use std::fmt::Display;

pub trait Summary {
    fn summarize_author( &self ) -> String;
    fn summarize( &self ) -> String {
        format!( "(Ream more from {}...)", self.summarize_author() )
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author( &self ) -> String {
        format!( "{}", self.author )
    }

    fn summarize( &self ) -> String {
        format!( "{}, by {} ({})", self.headline, self.author, self.location )
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author( &self ) -> String {
        format!( "@{}", self.username )
    }
}


// trait as a parameter type
pub fn notify( item: &impl Summary ) {
    println!( "Breaking news! {}", item.summarize() );
}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new( x: T, y: T ) -> Self {
        Self { x, y }
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display( &self ) {
        if self.x >= self.y {
            println!( "The largest member is x = {}", self.x );
        }
        else {
            println!( "The largest member is y = {}", self.y );
        }
    }
}