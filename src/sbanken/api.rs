extern crate hyper;
extern crate serde_json;

use self::serde_json::{from_str};
use reqwest::{StatusCode};
use std::io::Read;
use sbanken::model::{Account, BearerTokenResponse, Transaction};
use sbanken::data::{accounts_response_to_account, transactions_response_to_transactions};
use sbanken::helpers::{build_authorization_http_client, build_api_client};

// const SBANKEN_HOST : &str = "https://api.sbanken.no";
// const SBANKEN_AUTH : &str = "/identityserver/connect/token";
// const SBANKEN_API : &str = "/bank/api";
// const ACCOUNTS : &str = "/v1/accounts";
// const TRANSACTIONS : &str = "/v1/transactions";

const AUTH_ENDPOINT: &str = "https://auth.sbanken.no/identityserver/connect/token";
const ACCOUNTS_API: &str = "https://api.sbanken.no/bank/api/v1/accounts";
const TRANSACTIONS_API: &str = "https://api.sbanken.no/bank/api/v1/accounts";

// @ToDo: Inform that changing permission requires new key.

fn fetch_client_credentials() -> String {
    let client = build_authorization_http_client();

    let body = [("grant_type", "client_credentials")];
    
    info!("Executing authorization token request");
    let http_resp = client
        .post(AUTH_ENDPOINT)
        .form(&body)
        .send();

    let mut resp = match http_resp {
        Ok(resp) => resp,
        Err(error) => {
            error!("{}", error);
            panic!("{}", error);
        }
    };

    let mut body = String::new();
    match resp.read_to_string(&mut body) {
        Ok(_) => {},
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    let json_body : BearerTokenResponse = match from_str(&body) {
        Ok(resp) => resp,
        Err(error) => {
            error!("{}", error);
            panic!("{}", error);
        },
    };

    debug!("Retrieved bearer token");

    return json_body.access_token;
}

fn fetch_accounts() -> Vec<Account> {
    let bearer_token = fetch_client_credentials();
    let client = build_api_client(bearer_token);

    info!("Executing accounts request");
    let http_resp = client
        .get(ACCOUNTS_API)
        .send();

    let mut resp = match http_resp {
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
            debug!("Receieved {} data", data);
        },
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return accounts_response_to_account(body);
}

fn fetch_transactions(account_id: String) -> Vec<Transaction> {
    let bearer_token = fetch_client_credentials().to_owned();

    let client = build_api_client(bearer_token);

    info!("Executing transactions request");
    let http_resp = client
        .get(&format!("{}/{}", TRANSACTIONS_API, account_id))
        .send();

    let mut resp = match http_resp {
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
        Ok(_) => {},
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return transactions_response_to_transactions(body);
}

pub fn fetch_transactions_from_sbanken() -> Vec<Transaction> {
    let accounts = fetch_accounts();

    let mut transactions : Vec<Transaction> = Vec::new();

    for account in accounts {
        debug!("Account ID {}", account.account_id);
        let t = fetch_transactions(account.account_id);

        for transaction in t {
            debug!("Transaction {}: {} => {}", transaction.transaction_id, transaction.text, transaction.amount);
            transactions.push(transaction);
        }
    }

    return transactions;
}
