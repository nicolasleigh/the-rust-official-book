use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("Crash!")

    // Using a match expression to handle the Result variants that might be returned
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}")
            }
        },
    };

    // another way to write the same logic
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // Propagating Errors
    fn read_username_from_file_1() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Chaining method calls after the `?` operator
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // Using `fs::read_to_string` instead of opening and then reading the file
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // Creating Custom Types for Validation - By using the guessing game in ch2:
    // loop {
    //     // --snip--
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }
    //     match guess.cmp(&secret_number) {
    //       // --snip--
    //     }
    // }
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
            Guess { value }
        }
        // This public method is necessary because the value field of the Guess struct is private.
        // It’s important that the value field be private so code using the Guess struct is not allowed
        // to set value directly: code outside the module must use the Guess::new function to create an instance of Guess,
        // thereby ensuring there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
