use serde::Deserialize;


#[derive(Deserialize)]
pub(crate) enum TransactionType {
    Debit,
    Credit,
}

#[derive(Deserialize)]
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
}

#[derive(Deserialize)]
pub(crate) struct Transaction {
    pub(crate) date: String,
    pub(crate) description: String,
    pub(crate) original_description: String,
    pub(crate) amount: f32,
    pub(crate) transaction_type: TransactionType,
    pub(crate) category: Category,
    pub(crate) account_name: String,
    pub(crate) labels: String,
    pub(crate) notes: String,
}


impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            date: String::new(),
            description: String::new(),
            original_description: String::new(),
            amount: 0.00,
            transaction_type: TransactionType::Credit,
            category: Category::Transfer,
            account_name: String::new(),
            labels: String::new(),
            notes: String::new(),
        }
    }
}