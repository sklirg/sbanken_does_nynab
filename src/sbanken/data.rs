use crate::sbanken::model::{Account, SBankenAccountResponse, SBankenTransactionsResponse, Transaction};
use regex::Regex;

pub fn accounts_response_to_account(resp: String) -> Vec<Account> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming account response {}", resp);
    let data: SBankenAccountResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring account response: {}", error);
        }
    };

    return data.items;
}

pub fn transactions_response_to_transactions(resp: String) -> Vec<Transaction> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming transaction response {}", resp);
    let data: SBankenTransactionsResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring transactions response: {}", error);
        }
    };

    let mut transactions = Vec::new();

    let re = Regex::new(r"^\d{1,2}\.\d{1,2}\s(?P<r>.*)").unwrap();
    for mut transaction in data.items {
        transaction.text = re.replace_all(&transaction.text, "$r").to_string();
        transactions.push(transaction);
    }

    return transactions;
}

#[test]
fn strips_date_from_tx() {
    let txs = r#"{
        "availableItems": 1,
        "items": [
            {
                "accountingDate": "2021-01-12T00:00:00",
                "interestDate": "2021-01-12T00:00:00",
                "otherAccountNumberSpecified": false,
                "amount": 85,
                "text": "16.12 FOO",
                "transactionType": "StraksOvf",
                "transactionTypeCode": 561,
                "transactionTypeText": "StraksOvf",
                "isReservation": false,
                "reservationType": null,
                "source": "Archive",
                "cardDetailsSpecified": false,
                "transactionDetailSpecified": false
            }
        ]
    }"#;
    let converted = transactions_response_to_transactions(txs.to_string());
    // Assert that '16.12 FOO' has become 'FOO'
    assert_eq!("FOO", converted[0].text);
}
