/*
A new scope block created with curly brackets is an expression
This expression:
{
    let x = 3;
    x + 1
}
is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement.
Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far.
Expressions do not include ending semicolons.
If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
Keep this in mind as you explore function return values and expressions next.
*/
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}"); // The value of y is: 4

    let x = five();
    println!("The value of x is: {x}");

    // Because if is an expression, we can use it on the right side of a let statement 
    // to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // for range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

// Functions can return values to the code that calls them. 
// We don’t name return values, but we must declare their type after an arrow (->). 
fn five() -> i32 {
    5
}
