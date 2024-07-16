use enum_slicer::IntoEnumSlice;
use enum_slicer_proc::EnumSlice;

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
pub enum CurrencyType {
    MexicanPeso,
    JapaneseYen,
    ChineseYuan,
}

impl CurrencyType {
    /// Returns a vector of currency denominations for the specified currency type.
    ///
    /// # Returns
    ///
    /// A vector of currency denominations implementing the `Currency` trait.
    pub fn get_currencies(&self) -> Vec<&dyn Currency> {
        match self {
            CurrencyType::MexicanPeso => currency_slice_to_vec(MexicanCurrency::variants_slice()),
            CurrencyType::JapaneseYen => currency_slice_to_vec(JapaneseCurrency::variants_slice()),
            CurrencyType::ChineseYuan => currency_slice_to_vec(ChineseCurrency::variants_slice()),
        }
    }
}

fn currency_slice_to_vec<T: Currency>(currency: &[T]) -> Vec<&dyn Currency> {
    currency
        .iter()
        .map(|currency| currency as &dyn Currency)
        .collect()
}

/// A trait representing common behavior for currency denominations.
pub trait Currency {
    /// Returns the numeric value of the currency denomination.
    fn value(&self) -> f64;

    /// Returns a string representation of the currency denomination.
    fn corresponding_line(&self) -> &str;
}

/// Represents denominations of Mexican currency.
#[derive(EnumSlice)]
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
    fn value(&self) -> f64 {
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

#[derive(EnumSlice)]
pub enum JapaneseCurrency {
    TenThousand,
    FiveThousand,
    TwoThousand,
    OneThousand,
    FiveHundred,
    OneHundred,
    Fifty,
    Ten,
    Five,
    One,
}

impl Currency for JapaneseCurrency {
    /// Returns the numeric value of the Japanese currency denomination.
    ///
    /// # Returns
    ///
    /// A float representing the value in pesos.
    fn value(&self) -> f64 {
        match self {
            JapaneseCurrency::TenThousand => 10000.0,
            JapaneseCurrency::FiveThousand => 5000.0,
            JapaneseCurrency::TwoThousand => 2000.0,
            JapaneseCurrency::OneThousand => 1000.0,
            JapaneseCurrency::FiveHundred => 500.0,
            JapaneseCurrency::OneHundred => 100.0,
            JapaneseCurrency::Fifty => 50.0,
            JapaneseCurrency::Ten => 10.0,
            JapaneseCurrency::Five => 5.0,
            JapaneseCurrency::One => 1.0,
        }
    }

    /// Returns a string representation of the Japanese currency denomination in Spanish.
    ///
    /// # Returns
    ///
    /// A string slice containing the Spanish name of the denomination.
    fn corresponding_line(&self) -> &str {
        match self {
            JapaneseCurrency::TenThousand => "10000円",
            JapaneseCurrency::FiveThousand => "5000円",
            JapaneseCurrency::TwoThousand => "2000円",
            JapaneseCurrency::OneThousand => "1000円",
            JapaneseCurrency::FiveHundred => "500円",
            JapaneseCurrency::OneHundred => "100円",
            JapaneseCurrency::Fifty => "50円",
            JapaneseCurrency::Ten => "10円",
            JapaneseCurrency::Five => "5円",
            JapaneseCurrency::One => "1円",
        }
    }
}

#[derive(EnumSlice, Clone, Copy)]
pub enum ChineseCurrency {
    OneHundred,
    Fifty,
    Twenty,
    Ten,
    Five,
    One,
    FiveJiao,
    OneJiao,
    FiveFen,
    TwoFen,
    OneFen,
}

impl Currency for ChineseCurrency {
    fn value(&self) -> f64 {
        match self {
            ChineseCurrency::OneHundred => 100.0,
            ChineseCurrency::Fifty => 50.0,
            ChineseCurrency::Twenty => 20.0,
            ChineseCurrency::Ten => 10.0,
            ChineseCurrency::Five => 5.0,
            ChineseCurrency::One => 1.0,
            ChineseCurrency::FiveJiao => 0.5,
            ChineseCurrency::OneJiao => 0.1,
            ChineseCurrency::FiveFen => 0.05,
            ChineseCurrency::TwoFen => 0.02,
            ChineseCurrency::OneFen => 0.01,
        }
    }

    fn corresponding_line(&self) -> &str {
        match self {
            ChineseCurrency::OneHundred => "100元",
            ChineseCurrency::Fifty => "50元",
            ChineseCurrency::Twenty => "20元",
            ChineseCurrency::Ten => "10元",
            ChineseCurrency::Five => "5元",
            ChineseCurrency::One => "1元",
            ChineseCurrency::FiveJiao => "5角",
            ChineseCurrency::OneJiao => "1角",
            ChineseCurrency::FiveFen => "5分",
            ChineseCurrency::TwoFen => "2分",
            ChineseCurrency::OneFen => "1分",
        }
    }
}
