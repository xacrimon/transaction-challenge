use super::tx::{Amount, Tx, TxId, TxType, Client};
use std::collections::HashMap;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Account {
    client: Client,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
}

pub struct Engine {
    accounts: HashMap<Client, Account>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            accounts: HashMap::new(),
        }
    }

    pub fn apply(&mut self, tx: Tx) {
        match tx.ty {
            TxType::Deposit => self.deposit(tx.client, tx.amount, tx.id),
            TxType::Withdrawal => self.withdraw(tx.client, tx.amount, tx.id),
            TxType::Dispute => self.dispute(tx.client, tx.amount, tx.id),
            TxType::Resolve => self.resolve(tx.client, tx.amount, tx.id),
            TxType::Chargeback => self.chargeback(tx.client, tx.amount, tx.id),
        }
    }

    fn deposit(&mut self, client: Client, amount: Amount, tx_id: TxId) {
        todo!()
    }

    fn withdraw(&mut self, client: Client, amount: Amount, tx_id: TxId) {
        todo!()
    }

    fn dispute(&mut self, client: Client, amount: Amount, tx_id: TxId) {
        todo!()
    }

    fn resolve(&mut self, client: Client, amount: Amount, tx_id: TxId) {
        todo!()
    }

    fn chargeback(&mut self, client: Client, amount: Amount, tx_id: TxId) {
        todo!()
    }

    pub fn accounts(&self) -> impl Iterator<Item = Account> + '_ {
        self.accounts.values().copied()
    }
}