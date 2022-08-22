use crate::transactions::{Kind, Transaction};
use serde::Serialize;
use std::{collections::HashMap, io::Write};

pub mod client;
use client::Amount;

pub type ClientId = u16;

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

impl Bank {
    pub fn new() -> Bank {
        Bank::default()
    }

    pub fn handle_transactions(&mut self, transactions: Vec<Transaction>) {
        // todo
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

    pub fn output_accounts_report<W: Write>(&self, writer: &mut W) {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .from_writer(writer);

        for report in self.get_accounts_report() {
            writer.serialize(report).unwrap();
        }
        writer.flush().unwrap();
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
    fn test_client_report_for_basic_transactions() {
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
        bank.handle_transactions(transactions);
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
    }
}
