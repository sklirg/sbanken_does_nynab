use reqwest::{StatusCode, Method};
use ynab::data::{accounts_response_to_accounts, budgets_response_to_budgets};
use ynab::helpers::{build_api_client};
use ynab::model::{Account, Budget};
use std::io::Read;

const BUDGETS_API: &str = "https://api.youneedabudget.com/v1/budgets";

pub fn get_ynab_budgets() -> Vec<Budget> {
    let url: reqwest::Url = match reqwest::Url::parse(BUDGETS_API) {
        Ok(val) => val,
        Err(error) => panic!("Failed to parse url: {}", error),
    };

    let resp = do_api_request(url, Method::GET);
    let budgets = budgets_response_to_budgets(resp);
    debug!("Budgets: {:#?}", budgets);
    return budgets;
}

pub fn get_accounts(budget_id: String) -> Vec<Account> {
    let fmt_url = format!("{}/{}/accounts", BUDGETS_API, budget_id);
    let resp = do_api_request(parse_url(&fmt_url), Method::GET);
    let accounts = accounts_response_to_accounts(resp);

    trace!("Accounts: {:#?}", accounts);

    return accounts;
}

fn parse_url(url: &str) -> reqwest::Url {
    return match reqwest::Url::parse(url) {
        Ok(val) => val,
        Err(error) => panic!("Failed to parse url: {}", error),
    };
}

fn do_api_request(url: reqwest::Url, method: Method) -> String {
    let client = build_api_client();

    let request = match method {
        Method::GET => client
            .get(url)
            .send(),
        Method::POST => client
            .post(url)
            .send(),
        unhandled_method => panic!("Unhandled HTTP method: {}", unhandled_method),
    };

    let mut resp = match request {
        Ok(resp) => resp,
        Err(error) => {
            error!("{}", error);
            panic!("Response failed");
        },
    };

    match resp.status() {
        StatusCode::OK => trace!("HTTP OK"),
        status => warn!("Unhandled status code: {}", status),
    }

    let mut body = String::new();
    match resp.read_to_string(&mut body) {
        Ok(data) => {
            trace!("Receieved {} data", data);
        },
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return body;
}
