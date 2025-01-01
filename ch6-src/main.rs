// Listing 6-2: A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Defining an enum with variants such as the ones in Listing 6-2 is similar to
// defining different kinds of struct definitions, except the enum doesn’t use the `struct` keyword
// and all the variants are grouped together under the Message type.
// The following structs could hold the same data that the preceding enum variants hold:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum and Its Advantages Over Null Values
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // The match Control Flow Construct
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // Let’s change the rules of the game: now, if you roll anything other than a 3 or a 7,
        // you must roll again. We no longer need to use the catch-all value,
        // so we can change our code to use _ instead of the variable named other:
        _ => reroll(),
        // Finally, we’ll change the rules of the game one more time so that nothing else happens on your turn
        // if you roll anything other than a 3 or a 7. We can express that by using the unit value (the empty tuple type)
        _ => (),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    // Concise Control Flow with `if let`
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // We could write this in a shorter way using `if let`.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
