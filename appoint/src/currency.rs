use std::fmt::Debug;
use enum_iterator::EnumIterator;
use enum_iterator_proc::EnumIterator;

/// This macro generates an enum with an iterator over its variants.
/// it is not used, but i decided to keep it here for the nostalgia of creating my first baby macro
#[allow(unused_macros)]
macro_rules! enum_with_iterator {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $name {
            $($variant),*
        }
        
        impl CurrencyIterator for $name {
            fn iterator() -> impl Iterator<Item = Self> {
                vec![$($name::$variant),*].into_iter()
            }
        }
    };
}

/// Represents different types of currencies.
///
/// Currently, only the Mexican Peso is implemented.
pub enum CurrencyType {
    /// Represents the Mexican Peso currency.
    MexicanPeso,
}


impl CurrencyType {
    /// Returns a vector of currency denominations for the specified currency type.
    ///
    /// # Returns
    ///
    /// A vector of currency denominations implementing the `Currency` trait.
    pub fn get_currencies(&self) -> Vec<impl Currency> {
        match self {
            CurrencyType::MexicanPeso => MexicanCurrency::variants_iter().collect()
        }

    }
}

/// A trait representing common behavior for currency denominations.
pub trait Currency: EnumIterator + Debug {
    /// Returns the numeric value of the currency denomination.
    fn value(&self) -> f32;

    /// Returns a string representation of the currency denomination.
    fn corresponding_line(&self) -> &str;
}

/// Represents denominations of Mexican currency.
#[derive(Debug, EnumIterator)]
pub enum MexicanCurrency {
    Thousand,
    FiveHundred,
    TwoHundred,
    OneHundred,
    Fifty,
    Twenty,
    Ten,
    Five,
    Two,
    One,
    FiftyCentavos,
    TwentyCentavos,
    TenCentavos,
    FiveCentavos,
}

impl Currency for MexicanCurrency {
    /// Returns the numeric value of the Mexican currency denomination.
    ///
    /// # Returns
    ///
    /// A float representing the value in pesos.
    fn value(&self) -> f32 {
        match self {
            MexicanCurrency::Thousand => 1000.0,
            MexicanCurrency::FiveHundred => 500.0,
            MexicanCurrency::TwoHundred => 200.0,
            MexicanCurrency::OneHundred => 100.0,
            MexicanCurrency::Fifty => 50.0,
            MexicanCurrency::Twenty => 20.0,
            MexicanCurrency::Ten => 10.0,
            MexicanCurrency::Five => 5.0,
            MexicanCurrency::Two => 2.0,
            MexicanCurrency::One => 1.0,
            MexicanCurrency::FiftyCentavos => 0.5,
            MexicanCurrency::TwentyCentavos => 0.2,
            MexicanCurrency::TenCentavos => 0.1,
            MexicanCurrency::FiveCentavos => 0.05,
        }
    }

    /// Returns a string representation of the Mexican currency denomination in Spanish.
    ///
    /// # Returns
    ///
    /// A string slice containing the Spanish name of the denomination.
    fn corresponding_line(&self) -> &str {
        match self {
            MexicanCurrency::Thousand => "Billete 1000 pesos",
            MexicanCurrency::FiveHundred => "Billete 500 pesos",
            MexicanCurrency::TwoHundred => "Billete 200 pesos",
            MexicanCurrency::OneHundred => "Billete 100 pesos",
            MexicanCurrency::Fifty => "Billete 50 pesos",
            MexicanCurrency::Twenty => "Billete 20 pesos",
            MexicanCurrency::Ten => "Moneda 10 pesos",
            MexicanCurrency::Five => "Moneda 5 pesos",
            MexicanCurrency::Two => "Moneda 2 pesos",
            MexicanCurrency::One => "Moneda 1 peso",
            MexicanCurrency::FiftyCentavos => "Moneda 50 centavos",
            MexicanCurrency::TwentyCentavos => "Moneda 20 centavos",
            MexicanCurrency::TenCentavos => "Moneda 10 centavos",
            MexicanCurrency::FiveCentavos => "Moneda 5 centavos",            
        }
    }
}


