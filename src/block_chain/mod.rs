mod transaction;
pub use transaction::Transaction;

mod block;
pub use block::Block;

mod blockchain;
pub use blockchain::Blockchain;

pub fn calculate_hash(
    pre_hash: &String,
    transactions: &Vec<Transaction>,
    timestamp: i64,
) -> String {
    let mut bytes = Vec::new();

    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transactions
            .iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}
