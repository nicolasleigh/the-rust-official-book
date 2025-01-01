// An Example Program Using Structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated Functions:
// All functions defined within an `impl` block are called associated functions because they’re associated with the type
// named after the `impl`. We can define associated functions that don’t have `self` as their first parameter
// (and thus are not methods) because they don’t need an instance of the type to work with.
// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
// The `Self` keywords in the return type and in the body of the function are aliases for the type
// that appears after the `impl` keyword, which in this case is `Rectangle`.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// we want to borrow the struct rather than take ownership of it.
// This way, `main` retains its ownership and can continue using `rect1`
fn area(rect: &Rectangle) -> u32 {
    // note that accessing fields of a borrowed struct instance does not move the field values,
    // which is why you often see borrows of structs
    rect.width * rect.height
}
