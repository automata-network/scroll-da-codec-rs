use std::convert::Infallible;

pub use eth_types::Transaction;
use eth_types::{
    l2_types::{AccountTrace, BytecodeTrace, StorageTrace, TransactionTrace},
    EthBlock, H256,
};

use scroll_revm::primitives::EVMError;
pub use scroll_revm::primitives::{
    AccessListItem, Address, BlockEnv, Bytes, Env, ScrollFields, SpecId, TransactTo, TxEnv, B256,
    U256,
};

pub use mpt_zktrie::{builder::init_hash_scheme, AccountData, ZkTrie, ZktrieState};

pub use eth_types;
pub use scroll_revm as revm;
pub use zktrie::ZkMemoryDb;

// pub use eth_types::l2_types::BlockTrace;

#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
pub struct BlockTrace {
    /// Version string
    //pub version: String,
    /// chain id
    #[serde(rename = "chainID", default)]
    pub chain_id: u64,
    /// coinbase's status AFTER execution
    pub coinbase: AccountTrace,
    /// block
    pub header: EthBlock,
    /// txs
    pub transactions: Vec<TransactionTrace>,
    /// execution results
    // #[serde(rename = "executionResults", default)]
    // pub execution_results: Vec<ExecutionResult>,
    /// Accessed bytecodes with hashes
    #[serde(default)]
    pub codes: Vec<BytecodeTrace>,
    /// storage trace BEFORE execution
    #[serde(rename = "storageTrace")]
    pub storage_trace: StorageTrace,
    /// per-tx storage used by ccc
    // #[serde(rename = "txStorageTraces", default)]
    // pub tx_storage_trace: Vec<StorageTrace>,
    /// l1 tx queue
    #[serde(rename = "startL1QueueIndex", default)]
    pub start_l1_queue_index: u64,
    /// Withdraw root
    pub withdraw_trie_root: H256,
}

base::stack_error! {
    #[derive(Debug)]
    name: ExecutionError,
    stack_name: ExecutionErrorStack,
    error: {
        GenOldStateTrieFail { block_number: u64 },
        WithdrawalAccNotFound { block_number: u64, acc: Address },
        WithdrawalAccStorageNotFound { block_number: u64, acc: Address, root: B256 },
    },
    wrap: {
        EVM(EVMError<Infallible>),
        Str(String),
    },
    stack: {
        CommitTx(block_number: u64, tx_hash: H256),
        UpdateAccount(block_number: u64, acc: Address),
    }
}
