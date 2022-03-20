const AMOUNT_SHIFT_ACCURACY: f32 = 10000.0;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops;

#[derive(Deserialize, Clone, Copy)]
pub struct Tx {
    #[serde(rename = "type")]
    pub ty: TxType,

    #[serde(rename = "client")]
    pub client: Client,

    #[serde(rename = "tx")]
    pub id: TxId,

    #[serde(rename = "amount", deserialize_with = "Amount::deserialize", default)]
    pub amount: Amount,
}

#[derive(Deserialize, Clone, Copy)]
pub enum TxType {
    #[serde(rename = "deposit")]
    Deposit,

    #[serde(rename = "withdrawal")]
    Withdrawal,

    #[serde(rename = "dispute")]
    Dispute,

    #[serde(rename = "resolve")]
    Resolve,

    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Client(u16);

impl From<u16> for Client {
    fn from(id: u16) -> Self {
        Client(id)
    }
}

impl Into<u16> for Client {
    fn into(self) -> u16 {
        self.0
    }
}

#[derive(Deserialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TxId(u32);

impl From<u32> for TxId {
    fn from(id: u32) -> Self {
        TxId(id)
    }
}

impl Into<u32> for TxId {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
pub struct Amount(i32);

impl Amount {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let amount: Option<f32> = Deserialize::deserialize(deserializer)?;
        Ok(Amount::from(amount.unwrap_or(0.0)))
    }

    pub fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.clone().into())
    }
}

impl From<f32> for Amount {
    fn from(amount: f32) -> Self {
        Amount((amount * AMOUNT_SHIFT_ACCURACY) as i32)
    }
}

impl Into<f32> for Amount {
    fn into(self) -> f32 {
        self.0 as f32 / AMOUNT_SHIFT_ACCURACY
    }
}

impl ops::Add for Amount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Amount(self.0 + other.0)
    }
}

impl ops::Sub for Amount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Amount(self.0 - other.0)
    }
}
