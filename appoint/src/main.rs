use crate::currency::{Currency, CurrencyType};
use crate::currency::CurrencyType::{JapaneseYen, MexicanPeso};

mod currency;
mod decomposer;

fn main() {
    let currency_type = MexicanPeso;

    print_currencies(currency_type);
    
    let currency_type = JapaneseYen;
    
    print_currencies(currency_type);
}

fn print_currencies(currency_type: CurrencyType) {
    for currency in currency_type.get_currencies() {
        println!("{}: {}", currency.corresponding_line(), currency.value());
    }
}