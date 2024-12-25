#![allow(unused_variables)]

const TAX_RATE: f64 = 23.2;

type Meters = i32;

#[allow(unused_variables)]
fn main() {
    let apples = 5;
    let oranges = 14 + 32;
    let _fruits = apples + oranges;

    // println!("my garden has {} apples.", apples)
    // println!("my garden has {apples} apples and {oranges} oranges.")
    // println!("my garden has {} apples and {} oranges.", apples, oranges)
    println!(
        "my garden has {0} apples and {1} oranges. I can't believe I have {0} apples",
        apples, oranges
    );

    let mut gym_reps = 10;
    gym_reps = 13;

    let grams_of_protein = "100.32";
    let grams_of_protein = 100.32;
    let grams_of_protein = 100;

    let coffee_price = 5.99;
    {
        println!("The price is {coffee_price}")
    }

    let income: i32 = 10;

    #[allow(unused_variables)]
    let mile_race_length: Meters = 1500;
    let two_mile_race_length: Meters = 3200;
}
