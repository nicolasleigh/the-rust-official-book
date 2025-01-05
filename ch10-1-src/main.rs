// Not using generic
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// Generic In Function Definitions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// Generic In Struct Definitions
struct Point_1<T> {
    x: T,
    y: T,
}
struct Point_2<T, U> {
    x: T,
    y: U,
}
// Generic In Enum Definitions
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// Generic In Method Definitions
// Note that we have to declare T just after `impl` so we can use T to specify that we’re implementing methods on the type `Point<T>`.
// By declaring T as a generic type after `impl`, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point_1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// An `impl` block that only applies to a struct with a particular concrete type for the generic type parameter T
impl Point_1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Traits: Defining Shared Behavior:
// Defining a Trait
pub trait Summary {
    fn summarize(&self) -> String;
}
// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
        format!("{}: {}", self.username, self.content)
    }
}

// Default implementations with other methods
pub trait Summary_2 {
    fn summarize_author(&self) -> String;
    fn summarize_2(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// To use this version of Summary, we only need to define summarize_author when we implement the trait on a type:
impl Summary_2 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// The `impl Trait` syntax works for straightforward cases but is actually syntax sugar
// for a longer form known as a trait bound; it looks like this:
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
use std::fmt::{Debug, Display};
pub fn notify_3(item: &(impl Summary + Display)) {}
pub fn notify_4<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    32
}
// we can use a `where` clause to make it clearer
fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

// Returning Types That Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// the type `Pair<T>` always implements the `new` function to return a new instance of `Pair<T>`.
// But in the next impl block, `Pair<T>` only implements the `cmp_display` method 
// if its inner type T implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing.
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations 
// and are used extensively in the Rust standard library. 
// For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait.
// impl<T: Display> ToString for T {
//     // --snip--
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer_and_float = Point_2 { x: 5, y: 4.0 };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
