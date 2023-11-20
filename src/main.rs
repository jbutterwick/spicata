mod transactions;

use std::fs;
use leptos::*;
use transactions::Transaction;
use glob::glob;
use csv::Reader;
use crate::transactions::TransactionType;

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
    let account_data = data.iter().fold(AccountData::default(), |mut acc, record| {
        match record.transaction_type {
            TransactionType::Credit => {acc.net_worth += record.amount}
            TransactionType::Debit => {acc.net_worth -= record.amount}
        }
        acc
    });
    let spending_data = data.iter().fold(AccountData::default(), |mut acc, record| {
        match record.transaction_type {
            TransactionType::Credit => {acc.net_worth += record.amount}
            TransactionType::Debit => {acc.net_worth -= record.amount}
        }
        acc
    });
    let trends_data = data.iter().fold(AccountData::default(), |mut acc, record| {
        match record.transaction_type {
            TransactionType::Credit => {acc.net_worth += record.amount}
            TransactionType::Debit => {acc.net_worth -= record.amount}
        }
        acc
    });

    view! {
        <AccountSummary data=account_data/>
        <SpendingOverview data=spending_data/>
        <TrendsSummary data=trends_data/>
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
fn SpendingOverview(data:AccountData) -> impl IntoView {
    view! {
        <h2>"Spending"</h2>
        <span>{data.net_worth}</span>
    }
}

#[component]
fn TrendsSummary(data:AccountData) -> impl IntoView {
    view! {
        <h2>"Trends"</h2>
        <span>{data.net_worth}</span>
    }
}
fn main() {
    leptos::mount_to_body(App)
}
