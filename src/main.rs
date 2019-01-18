#[macro_use]
extern crate log;
extern crate log4rs;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod helpers;
mod sbanken;
mod ynab;

use sbanken::model::{Transaction};
use ynab::api::{get_ynab_budgets};

const SKIP_SBANKEN: bool = true;
const SKIP_YNAB: bool = false;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Starting app");

    if !SKIP_SBANKEN {
        info!("Starting transaction fetcher");
        let all_transactions = fetch_transactions();

        for transaction in &all_transactions {
            trace!("{:#?}", transaction);
        }
    }

    if !SKIP_YNAB {
        info!("Starting YNAB sync");
        update_ynab();
    }

    info!("Done.");
}

fn fetch_transactions() -> Vec<Transaction> {
    info!("Fetching transactions.");
    return sbanken::api::fetch_transactions_from_sbanken();
}

fn update_ynab() {
    get_ynab_budgets();
}
