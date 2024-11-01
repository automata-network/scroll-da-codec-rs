use alloy::{
    primitives::{Address, FixedBytes, TxHash}, rpc::types::{Block, BlockNumberOrTag, Filter, Log, Transaction}
};

use base::eth::{Eth, EthError};

pub struct L1Client {
    eth: Eth, 
}


impl L1Client {
    pub fn dial(url: &str) -> Result<Self, EthError> {
        let eth = Eth::dial(url, None)?;
        Ok(Self { eth })
    }

    pub async fn get_block_by_number(&self, block_number: BlockNumberOrTag) -> Result<Option<Block>, EthError> {
        let block = self
        .eth
        .provider()
        .get_block_by_number(block_number, false)
        .await?;

        Ok(block)
    }

    pub async fn get_logs<T: Into<BlockNumberOrTag>>(&self, contract: Address, event_signatures: Vec<FixedBytes<32>>, from: T, to: T) -> Result<Vec<Log>, EthError> {
        let filter = Filter::new()
        .address(contract)
        .event_signature(event_signatures)
        .from_block(from)
        .to_block(to);

        let logs = self.eth.provider().get_logs(&filter).await?;
        
        Ok(logs)
    }

    pub async fn get_transaction_by_hash(&self, tx_hash: TxHash) -> Result<Option<Transaction>, EthError> {
        let transaction = self
        .eth
        .provider()
        .get_transaction_by_hash(tx_hash)
        .await?;
        Ok(transaction)
    }

    pub async fn send_attestation_report(&self) -> Result<(), EthError> {
        todo!()
    }

    pub async fn finalize_bundle_with_tee_proof(
        &self,
        batch_header: Bytes,
        post_state_root: B256,
        withdraw_root: B256,
        tee_proof: Bytes,
    ) -> Result<(), EthError> {
        todo!()
    }

    pub async fn get_last_tee_finalized_batch_index(&self) -> Result<u64, EthError> {
        todo!()
    }
}