use heed::types::*;
use heed::{Database, RoTxn, RwTxn};
use plain_types::{sdk_types::Txid, AuthorizedTransaction};

pub struct MemPool {
    pub transactions: Database<OwnedType<[u8; 32]>, SerdeBincode<AuthorizedTransaction>>,
}

impl MemPool {
    pub const NUM_DBS: u32 = 1;

    pub fn new(env: &heed::Env) -> Result<Self, Error> {
        let transactions = env.create_database(Some("transactinos"))?;
        Ok(Self { transactions })
    }

    pub fn put(&self, txn: &mut RwTxn, transaction: &AuthorizedTransaction) -> Result<(), Error> {
        self.transactions
            .put(txn, &transaction.transaction.txid().into(), &transaction)?;
        Ok(())
    }

    pub fn delete(&self, txn: &mut RwTxn, txid: &Txid) -> Result<(), Error> {
        self.transactions.delete(txn, txid.into())?;
        Ok(())
    }

    pub fn take(&self, txn: &RoTxn, number: usize) -> Result<Vec<AuthorizedTransaction>, Error> {
        let mut transactions = vec![];
        for item in self.transactions.iter(txn)?.take(number) {
            let (_, transaction) = item?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("heed error")]
    Heed(#[from] heed::Error),
}
