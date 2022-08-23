use crate::transactions::{Kind, Transaction};
use error_stack::{Context, IntoReport, Report, Result, ResultExt};
use serde::Serialize;
use std::{collections::HashMap, fmt, io::Write};

pub mod client;
use client::Amount;

pub type ClientId = u16;

#[derive(Debug)]
pub enum BankError {
    InvalidInput,
    Other,
}

impl Context for BankError {}

impl fmt::Display for BankError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Wrong bank operation")
    }
}

#[derive(Default)]
pub struct Bank {
    clients: HashMap<ClientId, client::Account>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct AccountReport {
    client: ClientId,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
}

impl AccountReport {
    pub fn new(
        client: ClientId,
        available: Amount,
        held: Amount,
        total: Amount,
        locked: bool,
    ) -> AccountReport {
        AccountReport {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}

fn group_transactions(transactions: Vec<Transaction>) -> HashMap<ClientId, Vec<Transaction>> {
    let mut groups: HashMap<ClientId, Vec<Transaction>> = HashMap::new();

    for tx in transactions {
        let client_group = groups.entry(tx.client).or_insert(vec![]);
        if !crate::transactions::is_disputable(&tx) {
            if let None = client_group.iter().find(|x| x.tx == tx.tx) {
                continue;
            }
        }
        client_group.push(tx);
    }
    groups
}

fn get_amount_for_tx(tx: &Transaction, txs: &Vec<Transaction>) -> Option<f64> {
    if let Some(disputable_tx) = txs.into_iter().find(|tx_from_client| {
        (tx.tx == tx_from_client.tx) && crate::transactions::is_disputable(&tx_from_client)
    }) {
        if tx.kind == Kind::Withdrawal {
            if let Some(amount) = disputable_tx.amount {
                return Some(amount * -1.0);
            }
        }
        return disputable_tx.amount;
    }
    None
}

fn handle_tx(
    tx: &Transaction,
    account: &mut client::Account,
    txs: &Vec<Transaction>,
) -> Result<(), client::AccountError> {
    match tx.kind {
        Kind::Withdrawal => {
            let amount = tx.amount.expect("Should be checked when parsing");
            return account.withdrawal(amount).map_err(|err| Report::new(err));
        }
        Kind::Deposit => account.deposit(tx.amount.expect("Should be checked when parsing")),
        _ => {
            if let Some(amount) = get_amount_for_tx(&tx, &txs) {
                match tx.kind {
                    Kind::Dispute => account.dispute(amount),
                    Kind::Resolve => account.resolve(amount),
                    Kind::Chargeback => account.chargeback(amount),
                    _ => {}
                };
            }
        }
    }
    Ok(())
}

impl Bank {
    pub fn new() -> Bank {
        Bank::default()
    }

    pub fn handle_transactions(&mut self, transactions: Vec<Transaction>) -> Result<(), BankError> {
        for (client_id, txs) in &mut group_transactions(transactions) {
            let mut account = self
                .clients
                .entry(*client_id)
                .or_insert(client::Account::new());

            if account.is_locked() {
                continue;
            }

            //--> replace this block when `Vec::drain_filter` becomes stable
            let mut i = 0;
            while i < txs.len() {
                if handle_tx(&txs[i], &mut account, &txs).is_err() {
                    txs.remove(i);
                } else {
                    i += 1;
                }
            }
            //<--
        }
        Ok(())
    }

    pub fn get_accounts_report(&self) -> Vec<AccountReport> {
        let mut reports = Vec::new();
        for client in &self.clients {
            let report = AccountReport {
                client: *client.0,
                available: client.1.get_available_funds(),
                held: client.1.get_held_funds(),
                total: client.1.get_total_funds(),
                locked: client.1.is_locked(),
            };
            reports.push(report);
        }
        reports.sort_by(|a, b| a.client.cmp(&b.client));
        reports
    }

    pub fn output_accounts_report<W: Write>(&self, writer: &mut W) -> Result<(), BankError> {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .from_writer(writer);

        for report in self.get_accounts_report() {
            writer
                .serialize(&report)
                .report()
                .change_context(BankError::Other)
                .attach_printable(format!("Failed to serialize account report {report:?}"))?;
        }
        writer.flush().report().change_context(BankError::Other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_bank_starts_with_zero_clients() {
        let bank = Bank::new();

        assert!(bank.clients.is_empty());
    }

    #[test]
    fn test_client_report_for_basic_transactions() -> Result<(), BankError> {
        let mut bank = Bank::new();

        let transactions = vec![
            Transaction {
                kind: Kind::Deposit,
                client: 1,
                tx: 1,
                amount: Some(1.0),
            },
            Transaction {
                kind: Kind::Deposit,
                client: 2,
                tx: 2,
                amount: Some(2.0),
            },
            Transaction {
                kind: Kind::Deposit,
                client: 1,
                tx: 3,
                amount: Some(2.0),
            },
            Transaction {
                kind: Kind::Withdrawal,
                client: 1,
                tx: 4,
                amount: Some(1.5),
            },
            Transaction {
                kind: Kind::Withdrawal,
                client: 2,
                tx: 5,
                amount: Some(3.0),
            },
        ];
        bank.handle_transactions(transactions)?;
        let expected = vec![
            AccountReport {
                client: 1,
                available: 1.5,
                held: 0.0,
                total: 1.5,
                locked: false,
            },
            AccountReport {
                client: 2,
                available: 2.0,
                held: 0.0,
                total: 2.0,
                locked: false,
            },
        ];

        assert_eq!(bank.get_accounts_report(), expected);
        Ok(())
    }
}
