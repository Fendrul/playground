use crate::currency::CurrencyType::MexicanPeso;
use crate::currency::Currency;

mod currency;
mod decomposer;

fn main() {
    let currency_type = MexicanPeso;

    for currency in currency_type.get_currencies() {
        println!("{}: {}", currency.corresponding_line(), currency.value());
    }
}
