use std::fmt::Display;
use serde::Deserialize;


#[derive(Deserialize)]
pub(crate) enum TransactionType {
    Debit,
    Credit,
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
pub(crate) enum Category {
    Shopping,
    Grocery,
    Restaurants,
    Clothing,
    CoffeeShops,
    Doctor,
    Insurance,
    Utilities,
    Income,
    Transfer,
    CreditCardPayment,
    MortgageOrRent,
    FastFood,
    SpaOrMassage,
    HomeImprovement,
    Music,
    Misc,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Category::Shopping => String::from("Shopping"),
            Category::Grocery => String::from("Grocery"),
            Category::Restaurants => String::from("Restaurants"),
            Category::Clothing => String::from("Clothing"),
            Category::CoffeeShops => String::from("CoffeeShops"),
            Category::Doctor => String::from("Doctor"),
            Category::Insurance => String::from("Insurance"),
            Category::Utilities => String::from("Utilities"),
            Category::Income => String::from("Income"),
            Category::Transfer => String::from("Transfer"),
            Category::CreditCardPayment => String::from("CreditCardPayment"),
            Category::MortgageOrRent => String::from("MortgageOrRent"),
            Category::FastFood => String::from("FastFood"),
            Category::SpaOrMassage => String::from("SpaOrMassage"),
            Category::HomeImprovement => String::from("HomeImprovement"),
            Category::Music => String::from("Music"),
            Category::Misc => String::from("Misc"),
        };
        write!(f, "{}", str)
    }
}

impl Default for Category {
    fn default() -> Category {
        Category::Misc
    }
}

#[derive(Deserialize)]
pub(crate) struct Transaction {
    // pub(crate) date: String,
    // pub(crate) description: String,
    // pub(crate) original_description: String,
    pub(crate) amount: f32,
    pub(crate) transaction_type: TransactionType,
    pub(crate) category: Category,
    // pub(crate) account_name: String,
    // pub(crate) labels: Option<String>,
    // pub(crate) notes: Option<String>,
}


impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            // date: String::new(),
            // description: String::new(),
            // original_description: String::new(),
            amount: 0.00,
            transaction_type: TransactionType::Credit,
            category: Category::Transfer,
            // account_name: String::new(),
            // labels: Some(String::new()),
            // notes: Some(String::new()),
        }
    }
}