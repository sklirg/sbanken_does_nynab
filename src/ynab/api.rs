use reqwest::{StatusCode, Method};
use ynab::config::get_config;
// use ynab::data::{accounts_response_to_accounts, budgets_response_to_budgets};
use ynab::helpers::{build_api_client};
use ynab::model::{Transaction, TransactionsRequest};
use std::io::Read;

const BUDGETS_API: &str = "https://api.youneedabudget.com/v1/budgets";
// const TRANSACTION_API: &str = "https://api.youneedabudget.com/v1/budgets/{}/transactions";

// pub fn get_ynab_budgets() -> Vec<Budget> {
//     let resp = do_api_request(parse_url(BUDGETS_API), Method::GET);
//     let budgets = budgets_response_to_budgets(resp);
//     debug!("Budgets: {:#?}", budgets);
//     return budgets;
// }

// pub fn get_accounts(budget_id: String) -> Vec<Account> {
//     let fmt_url = format!("{}/{}/accounts", BUDGETS_API, budget_id);
//     let resp = do_api_request(parse_url(&fmt_url), Method::GET);
//     let accounts = accounts_response_to_accounts(resp);

//     trace!("Accounts: {:#?}", accounts);

//     return accounts;
// }

// pub fn post_transaction(budget_id: &str, account_id: &str, transaction: Transaction) {
//     let fmt_url = format!("{}/{}/transactions", BUDGETS_API, budget_id);

//     let mut transactions: Vec<Transaction> = Vec::new();
//     transactions.push(transaction);

//     let transaction_request_body = TransactionsRequest{
//         transactions: transactions,
//     };

//     let body = do_api_post_transaction_request(parse_url(&fmt_url), transaction_request_body);

//     trace!("Posted transaction {:#?}", body);
// }

pub fn post_transactions(transactions: Vec<Transaction>) {
    let config = get_config();
    let fmt_url = format!("{}/{}/transactions", BUDGETS_API, config.budget_id);

    let transaction_request_body = TransactionsRequest{
        transactions: transactions,
    };

    let body = do_api_post_transaction_request(parse_url(&fmt_url), transaction_request_body);

    trace!("Posted transactions {:#?}", body);
}

fn parse_url(url: &str) -> reqwest::Url {
    return match reqwest::Url::parse(url) {
        Ok(val) => val,
        Err(error) => panic!("Failed to parse url: {}", error),
    };
}

fn do_api_post_transaction_request(url: reqwest::Url, body: TransactionsRequest) -> String {
    let client = build_api_client();

    let request = client
            .post(url)
            .json(&body)
            .send();

    let mut resp = match request {
        Ok(resp) => resp,
        Err(error) => {
            error!("{}", error);
            panic!("Response failed");
        },
    };

    match resp.status() {
        StatusCode::OK => trace!("HTTP OK"),
        status => warn!("Unhandled status code: {} ({:#?})", status, resp),
    }

    let mut body = String::new();
    match resp.read_to_string(&mut body) {
        Ok(data) => {
            trace!("Receieved {} data ({})", data, body);
        },
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return body;
}

// fn do_api_request(url: reqwest::Url, method: Method) -> String {
//     let client = build_api_client();

//     let request = match method {
//         Method::GET => client
//             .get(url)
//             .send(),
//         Method::POST => client
//             .post(url)
//             .send(),
//         unhandled_method => panic!("Unhandled HTTP method: {}", unhandled_method),
//     };

//     let mut resp = match request {
//         Ok(resp) => resp,
//         Err(error) => {
//             error!("{}", error);
//             panic!("Response failed");
//         },
//     };

//     match resp.status() {
//         StatusCode::OK => trace!("HTTP OK"),
//         status => warn!("Unhandled status code: {}", status),
//     }

//     let mut body = String::new();
//     match resp.read_to_string(&mut body) {
//         Ok(data) => {
//             trace!("Receieved {} data", data);
//         },
//         Err(error) => {
//             error!("Failed to read response to string: {}", error);
//         }
//     }

//     return body;
// }
