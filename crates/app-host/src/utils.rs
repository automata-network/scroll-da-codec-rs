use base::eth::EthError;
use alloy::sol_types::Error;
use anyhow::Error as AnyError;


pub fn custom_eth_error<T>(reason: &str) -> Result<T, EthError> {
    Err(EthError::Type(Error::custom(reason)))
}

pub fn convert_eth_error(eth_error: EthError) -> AnyError {
    if let Some(data) = eth_error.revert() {
        anyhow::anyhow!(&data)
    } else {
        anyhow::anyhow!("unknown eth error")
    }
}