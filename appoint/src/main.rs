use std::collections::BTreeMap;
use std::io::stdin;
use std::str::FromStr;

use crate::currency::CurrencyType::{JapaneseYen, MexicanPeso};

mod currency;
fn main() {
    print!("Welcome to the currency decomposer!\n\n\
            Please select a currency to decompose:\n\
            1. Mexican Peso\n\
            2. Japanese Yen\n\n");

    let input = capture_input_bounded(1, 2);
    let currency_type = match input {
        1 => MexicanPeso,
        2 => JapaneseYen,
        _ => unreachable!("Invalid input"),
    };

    println!("Enter the amount to decompose :");
    let mut amount_to_decompose: f32 = capture_input();

    let mut map = BTreeMap::new();
    for currency in currency_type.get_currencies() {
        let quotient = amount_to_decompose / currency.value();

        if quotient >= 1.0 {
            amount_to_decompose -= quotient.floor() * currency.value();
            map.insert(currency, quotient as i32);
        }
    }

    println!("The decomposed amount is:");
    for (currency, count) in map {
        println!("{}: {}", currency.corresponding_line(), count);
    }

    if amount_to_decompose > 0.0 {
        println!();

        println!("Amount that couldn't be decomposed: {}", amount_to_decompose);
    }
}

fn capture_input<T: FromStr>() -> T {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        break match input.trim().parse::<T>() {
            Ok(num) => num,
            _ => {
                println!("Please enter a valid number.\n");
                continue;
            }
        };
    }
}

fn capture_input_bounded(min_input_value: i32, max_input_value: i32) -> i32 {
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to read line");

        break match input.trim().parse::<i32>() {
            Ok(num) if num >= min_input_value && num <= max_input_value => num,
            _ => {
                println!("Please enter a valid number.\n");
                continue;
            }
        };
    }
}