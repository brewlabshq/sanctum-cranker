use std::env;
use thiserror::Error;

#[derive(Default, Debug)]
pub struct CrankerConfig {
    pub port: u16,
    pub rpc_url: String,
    pub payer_private_key: String,
}

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Error: Invalid RPC URL")]
    InvalidRpcUrl,
    #[error("Error: Invalid payer private key")]
    InvalidPayerPrivateKey,
}

impl CrankerConfig {
    pub fn get_config() -> Self {
        let port = match env::var("PORT") {
            Ok(port) => port.parse::<u16>().unwrap(),
            Err(_) => 5555,
        };

        let rpc_url =
            env::var("RPC_URL").unwrap_or_else(|_| ConfigError::InvalidRpcUrl.to_string());

        let payer_private_key = env::var("PAYER_PRIVATE_KEY")
            .unwrap_or_else(|_| ConfigError::InvalidPayerPrivateKey.to_string());

        Self {
            port,
            rpc_url,
            payer_private_key,
        }
    }
}
