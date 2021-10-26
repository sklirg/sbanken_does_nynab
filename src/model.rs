struct Transaction {
    ID: String,
    AccountingDate: String, // Date
    InterestDate: String,   // Date
    Amount: i32,
    Text: String,
    TransactionType: String,
    TransactionTypeCode: u16,
    TransactionTypeText: String,
    IsReservation: bool,
    ReservationType: String,
}
