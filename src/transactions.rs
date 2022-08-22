use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub type TxId = u32;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub kind: Kind,
    pub client: u16,
    pub tx: TxId,
    pub amount: Option<f64>,
}

impl Transaction {
    pub fn new(kind: Kind, client: u16, tx: TxId, amount: Option<f64>) -> Transaction {
        Transaction {
            kind,
            client,
            tx,
            amount,
        }
    }
}

pub fn read_transactions(
    transactions_abs_path: &PathBuf,
) -> Vec<Transaction> {
    let mut transactions = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(transactions_abs_path).unwrap();

    for result in reader.deserialize() {
        let record: Transaction = result.unwrap();

        transactions.push(record);
    }
    transactions
}
