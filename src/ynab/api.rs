use reqwest::{StatusCode, Method};
use ynab::data::{budgets_response_to_budgets};
use ynab::helpers::{build_api_client};
use std::io::Read;

const BUDGETS_API: &str = "https://api.youneedabudget.com/v1/budgets";

pub fn get_ynab_budgets() {
    let client = build_api_client();

    let request = client
        .get(BUDGETS_API)
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

    let budgets = budgets_response_to_budgets(body);

    debug!("Budgets: {:#?}", budgets);
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
