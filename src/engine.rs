use super::tx::{Amount, Client, Tx, TxId, TxType};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

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
    tx_amounts: HashMap<TxId, Amount>,
    disputed: HashSet<TxId>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            accounts: HashMap::new(),
            tx_amounts: HashMap::new(),
            disputed: HashSet::new(),
        }
    }

    pub fn apply(&mut self, tx: Tx) -> Result<()> {
        self.tx_amounts.insert(tx.id, tx.amount);

        match tx.ty {
            TxType::Deposit => self.deposit(tx.client, tx.amount, tx.id),
            TxType::Withdrawal => self.withdraw(tx.client, tx.amount, tx.id),
            TxType::Dispute => self.dispute(tx.client, tx.amount, tx.id),
            TxType::Resolve => self.resolve(tx.client, tx.amount, tx.id),
            TxType::Chargeback => self.chargeback(tx.client, tx.amount, tx.id),
        }
    }

    fn account(&mut self, client: Client) -> Result<&mut Account> {
        let account = self.accounts.entry(client).or_insert(Account {
            client,
            available: Amount::from(0.0),
            held: Amount::from(0.0),
            total: Amount::from(0.0),
            locked: false,
        });

        if account.locked {
            return Err(anyhow!("account is locked"));
        }

        Ok(account)
    }

    fn deposit(&mut self, client: Client, amount: Amount, _tx_id: TxId) -> Result<()> {
        let account = self.account(client)?;

        account.available = account.available + amount;
        account.total = account.total + amount;
        Ok(())
    }

    fn withdraw(&mut self, client: Client, amount: Amount, tx_id: TxId) -> Result<()> {
        let account = self.account(client)?;

        if account.available < amount {
            return Err(anyhow!(
                "insufficient funds for withdrawal, tx: {:?}",
                tx_id
            ));
        }

        account.available = account.available - amount;
        account.total = account.total - amount;
        Ok(())
    }

    fn dispute(&mut self, client: Client, _amount: Amount, tx_id: TxId) -> Result<()> {
        let amount = *self
            .tx_amounts
            .get(&tx_id)
            .ok_or(anyhow!("tx not found, tx: {:?}", tx_id))?;
        let account = self.account(client)?;

        if account.available < amount {
            account.locked = true;
            return Err(anyhow!(
                "insufficient funds for dispute, locking account, tx: {:?}",
                tx_id
            ));
        }

        account.available = account.available - amount;
        account.held = account.held + amount;
        Ok(())
    }

    fn resolve(&mut self, client: Client, _amount: Amount, tx_id: TxId) -> Result<()> {
        let amount = *self
            .tx_amounts
            .get(&tx_id)
            .ok_or(anyhow!("tx not found, tx: {:?}", tx_id))?;
        let account = self.account(client)?;

        if account.held < amount {
            return Err(anyhow!(
                "insufficient held funds, this shouldn't happen, tx: {:?}",
                tx_id
            ));
        }

        account.available = account.available + amount;
        account.held = account.held - amount;
        Ok(())
    }

    fn chargeback(&mut self, client: Client, _amount: Amount, tx_id: TxId) -> Result<()> {
        let amount = *self
            .tx_amounts
            .get(&tx_id)
            .ok_or(anyhow!("tx not found, tx: {:?}", tx_id))?;
        let account = self.account(client)?;

        if account.held < amount {
            return Err(anyhow!(
                "insufficient held funds, this shouldn't happen, tx: {:?}",
                tx_id
            ));
        }

        account.held = account.held - amount;
        account.total = account.total - amount;
        account.locked = true;
        Ok(())
    }

    pub fn accounts(&self) -> impl Iterator<Item = Account> + '_ {
        self.accounts.values().copied()
    }
}

// ignore resolve, chargeback in some cases
