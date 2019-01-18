extern crate serde_derive;

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
