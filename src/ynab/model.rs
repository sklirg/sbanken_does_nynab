extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde::Deserialize;
use self::serde::de::Deserializer;

pub struct YnabConfig {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
}

#[derive(Deserialize)]
pub struct BudgetBaseResponse {
    pub data: BudgetResponse,
}

#[derive(Deserialize)]
pub struct BudgetResponse {
    pub budgets: Vec<Budget>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,

    #[serde(rename = "type")]
    pub account_type: String,

    pub on_budget: bool,
    pub closed: bool,

    #[serde(deserialize_with = "null_to_empty_string")]
    pub note: String,
    pub balance: i32,
    pub cleared_balance: i32,
    pub uncleared_balance: i32,
    pub transfer_payee_id: String,
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct AccountResponse {
    pub accounts: Vec<Account>,
}

#[derive(Deserialize)]
pub struct AccountBaseResponse {
    pub data: AccountResponse,
}

fn null_to_empty_string<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("".to_string())
        })
}
