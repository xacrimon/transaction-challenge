const AMOUNT_SHIFT_ACCURACY: f32 = 1000.0;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Tx {
    pub ty: TxType,
    pub client: Client,
    pub id: TxId,

    #[serde(deserialize_with = "Amount::deserialize")]
    pub amount: Amount,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Amount(u32);

impl Amount {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let amount: f32 = Deserialize::deserialize(deserializer)?;
        Ok(Amount::from(amount))
    }
}

impl From<f32> for Amount {
    fn from(amount: f32) -> Self {
        Amount((amount * AMOUNT_SHIFT_ACCURACY) as u32)
    }
}

impl Into<f32> for Amount {
    fn into(self) -> f32 {
        self.0 as f32 / AMOUNT_SHIFT_ACCURACY
    }
}
