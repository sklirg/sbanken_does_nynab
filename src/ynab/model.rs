extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde::de::Deserializer;
use self::serde::Deserialize;

use crate::sbanken::model::Transaction as SbankenTransaction;
use crate::ynab::helpers::to_milliunits;

pub struct YnabConfig {
    pub access_token: String,
    pub budget_id: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub account_id: String,
    pub date: String,
    pub payee_name: Option<String>,
    pub amount: i32,
    pub memo: Option<String>,
    pub import_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionsRequest {
    pub transactions: Vec<Transaction>,
}

fn null_to_empty_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

pub fn sbanken_account_to_ynab(account_id: &str) -> String {
    return match account_id {
        "73AB74C904F72EB4150F79779C988F8F" => "4ece9b96-7836-419d-b436-3e1a1634a1db".to_string(), // Alt
        "55589FE3B128944387C6ADB0A4E495B7" => "09e453f6-99d0-4c6d-abce-09cd1d3ee0dd".to_string(), // Sparing
        "9AA21EBBB8B1361B6462E15649695DDD" => "a0ca772a-fa40-480c-a7d2-678d5f67079c".to_string(), // Regningsbetaling
        "C91CDA494608C5D2A923A3F8F5002335" => "52e9f94f-91ab-4a96-8cce-df66d88d7190".to_string(), // Husleie
        "F839DB7C901B3E453B0FABC818CA1031" => "46d7b736-2570-4b38-9020-a5d86f0ff76e".to_string(), // Kredittkort
        _ => panic!("Missing account for account id {}", account_id),
    };
}

pub fn sbanken_to_ynab_transaction(
    transaction: &SbankenTransaction,
    account_id: &str,
) -> Transaction {
    debug!("Processing transaction {}", transaction.transaction_id);
    // trace!("Processing transaction {:#?}", transaction);

    if transaction.transaction_id == "0" {
        warn!("Skipping transaction {} with id 0", transaction.text);
    }

    let converted_amount = to_milliunits(&transaction.amount);

    let ynab_account = sbanken_account_to_ynab(account_id);

    let import_id = format!(
        "sb:{}-{}-{:?}",
        transaction.accounting_date[..10].to_string(),
        transaction.amount,
        transaction.text.chars()
    )[..36]
        .to_string();

    return Transaction {
        account_id: ynab_account,
        date: transaction.accounting_date.to_string(),
        payee_name: Some(transaction.text.to_string()),
        amount: converted_amount,
        memo: Some("".to_string()),
        import_id: import_id,
    };
}
