use anyhow::Result;
use serde::Deserialize;
use std::fs::File;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub l1_geth_endpoint: String,
    pub l2_geth_endpoint: String,
    pub enclave_endpoint: String,
}

impl Config {
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn from_file(file_name: String) -> Result<Self> {
        let file = File::open(file_name)?;
        Config::from_reader(&file)
    }
}
