mod transactions;

use chrono::{DateTime, Utc};
use leptos::*;
use transactions::Transaction;
use glob::glob;
use csv::Reader;
use crate::transactions::{Category, TransactionType};
use itertools::{Itertools};

struct AccountData {
    net_worth: f32,
    credit_card_total: f32,
    investments_total: f32,
    loans_total: f32,
    property_total: f32,
}

impl Default for AccountData {
    fn default() -> AccountData {
        AccountData {
            net_worth: 0.00,
            credit_card_total: 0.00,
            investments_total: 0.00,
            loans_total: 0.00,
            property_total: 0.00,
        }
    }
}

struct CategoryData {
    category: Category,
    total_spend: f32,
    date: DateTime<Utc>,
}

impl CategoryData {
    fn new(category:Category, total_spend:f32, date:DateTime<Utc>) -> Self {
        CategoryData {
            category,
            total_spend,
            date,
        }
    }
}

impl Default for CategoryData {
    fn default() -> CategoryData {
        CategoryData {
            category: Category::default(),
            total_spend: 0.00,
            date: DateTime::default(),
        }
    }
}


fn accumulate_account_data(mut acc:AccountData, rec:&Transaction) -> AccountData {
    match rec.transaction_type {
        TransactionType::Credit => {acc.net_worth += rec.amount}
        TransactionType::Debit => {acc.net_worth -= rec.amount}
    }
    acc
}


#[component]
fn App() -> impl IntoView {
    let data: Vec<Transaction> = glob("data/*.csv").expect("No csv files in data dir").fold(Vec::new(), |mut acc, entry| {
            match entry {
                Ok(path) => {
                    let mut reader = Reader::from_path(path).expect("couldn't read file");
                    for record in reader.deserialize() {
                        let record = record.unwrap_or(Transaction::default());
                        acc.push(record);
                    }
                    acc
                },
                Err(_e) => Vec::new(),
            }
        }
    );

    let account_data = data.iter().fold(AccountData::default(), accumulate_account_data);

    let mut grouped_by_category:Vec<(Category, Vec<Transaction>)> = Vec::new();
    for (key, group) in &data.into_iter().group_by(|elt| elt.category) {
        grouped_by_category.push((key, group.collect()));
    }

    let spending_data: Vec<CategoryData> = grouped_by_category.iter().map(|(category, run)| {
        run.iter().fold(CategoryData::new(category.clone(), 0.00, DateTime::default()), |mut acc, record| {
            match record.transaction_type {
                TransactionType::Credit => {acc.total_spend += record.amount}
                TransactionType::Debit => {acc.total_spend -= record.amount}
            }
            acc
        })
    }).collect();

    view! {
        <AccountSummary data=account_data/>
        <SpendingOverview data=spending_data/>
    }
}


#[component]
fn AccountSummary(
    data: AccountData
) -> impl IntoView {
    view! {
        <h2>"Accounts"</h2>
        <span>Net Worth:</span>
        <span>{data.net_worth}</span>
        <span>Credit Cards:</span>
        <span>{data.credit_card_total}</span>
        <span>Investments:</span>
        <span>{data.investments_total}</span>
        <span>Loans:</span>
        <span>{data.loans_total}</span>
        <span>Property:</span>
        <span>{data.property_total}</span>
    }
}

#[component]
fn SpendingOverview(data:Vec<CategoryData>) -> impl IntoView {
    let categories = data.iter()
        .map(|item| {
            view! {
                <li>
                    <span>{item.category.to_string()} :</span>
                    <span>{item.total_spend}</span>
                    <span>{item.date.to_string()}</span>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <h2>"Spending by Category"</h2>
        <ul>{categories}</ul>
    }
}

fn main() {
    mount_to_body(App)
}
