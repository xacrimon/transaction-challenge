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
        todo!()
    }

    pub fn accounts(&self) -> impl Iterator<Item = Account> + '_ {
        self.accounts.values().copied()
    }
}