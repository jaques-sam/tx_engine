use error_stack::{Context, IntoReport, Report, Result, ResultExt};
use serde::Deserialize;
use std::fmt;
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

#[derive(Debug, PartialEq)]
pub enum ParseTxError {
    InvalidInput(String),
    Other,
}

impl Context for ParseTxError {}

impl fmt::Display for ParseTxError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Parsing a transaction record failed")
    }
}

pub fn is_disputable(tx: &Transaction) -> bool {
    tx.kind == Kind::Deposit || tx.kind == Kind::Withdrawal
}

fn validate_optional_field(transaction: &Transaction) -> Result<(), ParseTxError> {
    if is_disputable(transaction) {
        if transaction.amount.is_none() {
            return Err(Report::new(ParseTxError::InvalidInput(format!(
                "{:?} transactions must contain an amount",
                transaction.kind
            ))));
        }
    } else {
        if transaction.amount.is_some() {
            return Err(Report::new(ParseTxError::InvalidInput(format!(
                "{:?} transactions cannot contain an amount",
                transaction.kind
            ))));
        }
    }
    Ok(())
}

/// Read 'Transaction' records from a CSV file which includes a header row
pub fn parse_transactions(
    transactions_abs_path: &PathBuf,
) -> Result<Vec<Transaction>, ParseTxError> {
    let mut transactions = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(transactions_abs_path)
        .report()
        .attach_printable(format!("{transactions_abs_path:?} is not a valid file"))
        .change_context(ParseTxError::InvalidInput("CSV parser cannot be built".to_owned()))?;

    for (idx, result) in reader.deserialize().into_iter().enumerate() {
        let line_nbr = idx + 1; // 1 header + starting from 1
        let record: Transaction = result
            .report()
            .attach_printable(format!("has an invalid transaction on line {line_nbr}"))
            .change_context(ParseTxError::InvalidInput("record cannot be parsed".to_owned()))?;

        validate_optional_field(&record)?;
        transactions.push(record);
    }
    Ok(transactions)
}
