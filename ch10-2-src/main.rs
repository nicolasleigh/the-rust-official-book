// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x; // dangling references
//     }
//     println!("r: {r}");
// }

// missing lifetime specifier error
// fn longest(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//       x
//   } else {
//       y
//   }
// }

// The `longest` function definition specifying that all the references in the signature must have the same lifetime `'a`
// The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are
// string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice
// returned from the function will live at least as long as lifetime `'a`. In practice, it means that the lifetime of the
// reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations in Struct Definitions
// This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// Lifetime Elision:
// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
// The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
// If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes,
// the compiler will stop with an error. These rules apply to `fn` definitions as well as `impl` blocks.
// 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
// 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned
// to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self
// because this is a method, the lifetime of self is assigned to all output lifetime parameters.

// Lifetime Annotations in Method Definitions
// The lifetime parameter declaration after `impl` and its use after the type name are required,
// but we’re not required to annotate the lifetime of the reference to `self` because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self`
// and `announcement` their own lifetimes. Then, because one of the parameters is `&self`,
// the return type gets the lifetime of `&self`, and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Using the longest function with references to String values that have different concrete lifetimes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Attempting to use result after string2 has gone out of scope -- error!
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // A struct that holds a reference, requiring a lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // The Static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference
    // can live for the entire duration of the program. All string literals have the 'static lifetime,
    // which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
}
