extern crate serde_derive;

#[derive(Debug)]
pub struct SbankenConfig {
    pub username: String,
    pub password: String,
    pub customer_id: String,
}

#[derive(Deserialize)]
pub struct BearerTokenResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,

    #[serde(rename = "expires_in")]
    pub expires_in: u16,

    #[serde(rename = "token_type")]
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct SBankenAccountResponse {
    #[serde(rename = "availableItems")]
    pub available_items: u16,

    pub items: Vec<Account>,

    // "errorType":null,
    // "isError":false,
    // "errorMessage":null,
    // "traceId":null
}

#[derive(Deserialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    pub account_id: String,
    
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    #[serde(rename = "ownerCustomerId")]
    pub owner_customer_id: String,

    pub name: String,

    #[serde(rename = "accountType")]
    pub account_type: String,

    pub available: f32,

    pub balance: f32,

    #[serde(rename = "creditLimit")]
    pub credit_limit: f32,
}

#[derive(Deserialize)]
pub struct SBankenTransactionsResponse {
    #[serde(rename = "availableItems")]
    pub available_items: u16,

    pub items: Vec<Transaction>,

    // "errorType":null,
    // "isError":false,
    // "errorMessage":null,
    // "traceId":null
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Transaction {
    #[serde(default)]
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "accountingDate")]
    pub accounting_date: String, // iso8601 date

    #[serde(rename = "interestDate")]
    pub interest_date: String, // iso8601 date

    #[serde(rename = "otherAccountNumberSpecified")]
    pub other_account_number_specified: bool,

    pub amount: f32,

    pub text: String,

    #[serde(rename = "transactionType")]
    pub transaction_type: String,

    #[serde(rename = "transactionTypeCode")]
    pub transaction_type_code: i16, // Enum

    #[serde(rename = "transactionTypeText")]
    pub transaction_type_text: String,

    #[serde(rename = "isReservation")]
    pub is_reservation: bool,

    pub source: String,

    #[serde(rename = "cardDetailsSpecified")]
    pub card_details_specified: bool,
}
