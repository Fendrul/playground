use std::io::stdin;
use std::str::FromStr;

use crate::currency::CurrencyType::{ChineseYuan, JapaneseYen, MexicanPeso};

mod currency;
fn main() {
    print!(
        "Welcome to the currency decomposer!\n\n\
            Please select a currency to decompose:\n\
            1. Mexican Peso\n\
            2. Japanese Yen\n\
            3. Chinese Yuan\n\n"
    );

    let input = capture_input_bounded(1, 3);
    let currency_type = match input {
        1 => MexicanPeso,
        2 => JapaneseYen,
        3 => ChineseYuan,
        _ => unreachable!("Invalid input"),
    };

    println!("\nEnter the amount to decompose :");
    let mut amount_to_decompose: f64 = capture_input();

    let mut results = Vec::new();
    for currency in currency_type.get_currencies() {
        let quotient = amount_to_decompose / currency.value();

        if quotient >= 1.0 {
            amount_to_decompose -= quotient.floor() * currency.value();
            results.push((currency, quotient as i32));
        }
    }

    println!("\nThe amount decomposed is:");
    for (currency, count) in results {
        println!("{}: {}", currency.corresponding_line(), count);
    }

    println!("\nRemainder: {:.2}", amount_to_decompose);
}

/// Captures user input and parses it into the specified type.
///
/// This function repeatedly prompts the user for input until a valid value of type `T` is entered.
/// It uses the `FromStr` trait to parse the input string into the desired type.
///
/// # Type Parameters
///
/// * `T`: The type to parse the input into. Must implement the `FromStr` trait.
///
/// # Returns
///
/// Returns the parsed value of type `T`.
///
/// # Panics
///
/// This function will panic if it fails to read from standard input.
///
/// # Examples
///
/// ```
/// let number: i32 = capture_input();
/// println!("You entered: {}", number);
///
/// let float: f64 = capture_input();
/// println!("You entered: {}", float);
/// ```
fn capture_input<T: FromStr>() -> T {
    loop {
        let input = read_input_string();

        break if let Ok(num) = input.trim().parse::<T>() {
            num
        } else {
            println!("Please enter a valid number.\n");
            continue;
        };
    }
}

/// Captures user input within a specified range of integer values.
///
/// This function repeatedly prompts the user for input until a valid integer within the specified
/// range is entered.
///
/// # Arguments
///
/// * `min_input_value`: The minimum acceptable value (inclusive).
/// * `max_input_value`: The maximum acceptable value (inclusive).
///
/// # Returns
///
/// Returns the parsed integer value within the specified range.
///
/// # Panics
///
/// This function will panic if it fails to read from standard input.
///
/// # Examples
///
/// ```
///let age = capture_input_bounded(0, 120); println!("Your age is: {}", age); ///
///
/// let rating = capture_input_bounded(1, 5);
///
/// println!("You rated: {} stars", rating);
///```
fn capture_input_bounded(min_input_value: i32, max_input_value: i32) -> i32 {
    loop {
        let input = read_input_string();

        break match input.trim().parse::<i32>() {
            Ok(num) if num >= min_input_value && num <= max_input_value => num,
            _ => {
                println!("Please enter a valid number.\n");
                continue;
            }
        };
    }
}

fn read_input_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}
