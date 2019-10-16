extern crate hyper;
extern crate serde_json;

use self::serde_json::{from_str};
use reqwest::{StatusCode};
use std::collections::HashMap;
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
const ACCOUNTS_API: &str = "https://api.sbanken.no/exec.bank/api/v1/accounts";
const TRANSACTIONS_API: &str = "https://api.sbanken.no/exec.bank/api/v1/transactions";

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

    // @ToDo: Handle NON 200 OK status code

    if resp.status() != 200 {
        warn!("Client Credentials request failed with non 200 status code");
    }
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
            trace!("Receieved {} data", data);
        },
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return accounts_response_to_account(body);
}

fn fetch_transactions(account_id: &str) -> Vec<Transaction> {
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
        Ok(data) => {
            trace!("Receieved {} data", data);
        },
        Err(error) => {
            error!("Failed to read response to string: {}", error);
        }
    }

    return transactions_response_to_transactions(body);
}

pub fn fetch_transactions_from_sbanken() -> HashMap<String, Vec<Transaction>> {
    let accounts = fetch_accounts();

    let mut accounts_with_transactions: HashMap<String, Vec<Transaction>> = HashMap::new();

    for account in accounts {
        let t = fetch_transactions(&account.account_id);
        info!("Account ID {} with {} transactions", account.account_id, t.len());
        accounts_with_transactions.insert(account.account_id, t);
    }

    return accounts_with_transactions;
}
