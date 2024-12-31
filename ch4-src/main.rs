// The Stack and the Heap:

// All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

// The heap is less organized: when you put data on the heap, you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use,
// and returns a pointer, which is the address of that location.
// This process is called allocating on the heap and is sometimes abbreviated as just allocating
// (pushing values onto the stack is not considered allocating).
// Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack,
// but when you want the actual data, you must follow the pointer.

fn main() {
    let s = "hello";
    // String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.
    // One reason is that they’re immutable. Another is that not every string value can be known when we write our code:
    // for example, what if we want to take user input and store it?
    // For these situations, Rust has a second string type, `String`.
    let s = String::from("hello");
    // The double colon :: operator allows us to namespace this particular `from` function under the String type
    // rather than using some sort of name like `string_from`.

    // This kind of string can be mutated:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
    // With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap,
    // unknown at compile time, to hold the contents. This means:
    //        The memory must be requested from the memory allocator at runtime.
    //        We need a way of returning this memory to the allocator when we’re done with our String.
    // That first part is done by us: when we call `String::from`, its implementation requests the memory it needs.
    // However, the second part is different. Unlike garbage collector language, Rust takes a different path:
    // the memory is automatically returned once the variable that owns it goes out of scope.
    // When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`,
    // and it’s where the author of `String` can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.

    let x = 5;
    let y = x;
    // We can probably guess what this is doing: “bind the value 5 to x;
    // then make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5.

    let s1 = String::from("hello");
    let s2 = s1;
    // This looks very similar, so we might assume that the way it works would be the same:
    // that is, the second line would make a copy of the value in s1 and bind it to s2.
    // But this isn’t quite what happens.
    // To ensure memory safety (free from double free error), after the line let s2 = s1;,
    // Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // Because Rust also invalidates the first variable, instead of being called a `shallow copy`, it’s known as a `move`.

    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
