use super::*;
use time;

pub fn now() -> i64 {
    // time::now().to_timespec().sec
    time::OffsetDateTime::now_utc().unix_timestamp()
}

#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}
impl Block {
    ///
    /// Block constructor
    /// Creates block from previous block hash and transaction data
    ///

    pub fn new(pre_hash: String, transaction: Vec<Transaction>) -> Self {
        let time = now();
        Block {
            timestamp: time,
            hash: calculate_hash(&pre_hash, &transaction, time),
            pre_hash,
            transaction,
        }
    }
}
