use std::time::Duration;

use alloy::primitives::U64;
use base::eth::{Eth, EthError, PrimitivesConvert};
use scroll_executor::BlockTrace;

#[derive(Clone)]
pub struct ScrollExecutionNode {
    eth: Eth,
    call_timeout: Option<Duration>,
}

impl ScrollExecutionNode {
    pub fn dial(url: &str, call_timeout: Option<Duration>) -> Result<Self, EthError> {
        let mut eth = Eth::dial(url, None)?;
        eth.with_call_timeout(call_timeout.clone());
        Ok(Self { eth, call_timeout })
    }

    pub async fn trace_block(&self, blk: u64) -> Result<BlockTrace, EthError> {
        let blk: U64 = blk.to();
        let block_trace = base::thread::wait_timeout(
            self.call_timeout,
            self.eth
                .client()
                .request("scroll_getBlockTraceByNumberOrHash", (blk,)),
        )
        .await??;

        Ok(block_trace)
    }
}
