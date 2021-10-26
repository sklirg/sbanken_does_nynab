#[macro_use]
extern crate log;
extern crate log4rs;
extern crate regex;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod helpers;
mod sbanken;
mod ynab;

use sbanken::model::Transaction;
use std::collections::HashMap;
use ynab::api::post_transactions;
use ynab::model::{sbanken_to_ynab_transaction, Transaction as YnabTransaction};

const SKIP_SBANKEN: bool = false;
const SKIP_YNAB: bool = false;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Starting app");

    let mut all_transactions: Option<HashMap<String, Vec<Transaction>>> = None;

    if !SKIP_SBANKEN {
        info!("Starting transaction fetcher");
        all_transactions = fetch_transactions();

        for transaction in &all_transactions {
            trace!("Processing transaction {:#?}", transaction);
        }
    }

    if !SKIP_YNAB {
        info!("Starting YNAB sync");
        match all_transactions {
            Some(txs) => update_ynab(txs),
            None => (),
        }
    }

    info!("Done.");
}

fn fetch_transactions() -> Option<HashMap<String, Vec<Transaction>>> {
    info!("Fetching transactions.");
    return sbanken::api::fetch_transactions_from_sbanken();
}

fn update_ynab(accounts: HashMap<String, Vec<Transaction>>) {
    // let budgets = get_ynab_budgets();

    // for budget in budgets {
    //     get_accounts(budget.id);
    //     // sbanken_to_ynab_transaction
    // }

    let mut remap: HashMap<String, Vec<YnabTransaction>> = HashMap::new();

    for account in accounts.keys() {
        let mut converted_transactions: Vec<YnabTransaction> = Vec::new();
        let transactions: &Vec<Transaction> = match accounts.get(account) {
            Some(t) => t,
            None => panic!("Getting transactions from hashmap failed"),
        };

        debug!("Doing things to {}", account);

        // Sbanken decided to drop transactionId
        // filter(|t| t.transaction_id != "0" && t.transaction_id != "")
        for transaction in transactions.into_iter() {
            let converted_transaction = sbanken_to_ynab_transaction(&transaction, account);
            trace!("Converted transaction {:#?}", converted_transaction);
            converted_transactions.push(converted_transaction);
        }

        remap.insert(account.to_owned(), converted_transactions);
    }

    trace!("Remapped: {:#?}", remap);

    // Debug
    // let one_account_key = match remap.keys().last() {
    //     Some(key) => key,
    //     None => panic!("No key!"),
    // };
    let one_account_key = "9AA21EBBB8B1361B6462E15649695DDD";

    let one_account = match remap.get(one_account_key) {
        Some(t) => t,
        None => panic!("No transactions in list"),
    };

    let one_transaction: YnabTransaction = match one_account.last() {
        Some(t) => t.to_owned(),
        None => panic!("Whelp! No transaction"),
    };

    // let one_transaction: YnabTransaction = match one_account.into_iter().filter(|t| t.import_id == "sbanken:118347958781857893").last() {
    //     Some(t) => t.to_owned(),
    //     None => panic!("Transaction is gone :o"),
    // };

    debug!("One transaction: {:#?}", one_transaction);

    // post_transaction("1c19292f-14d5-4e0f-9384-656c45899458", one_account_key, one_transaction);
    // post_transaction("", transactions);

    info!("Posting transactions to YNAB");
    for account in remap.keys() {
        let transactions = match remap.get(account) {
            Some(t) => t.to_owned(),
            None => panic!("No transactions in converted map"),
        };

        post_transactions(transactions);
    }
}
