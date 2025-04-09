use key_utils::Secp256k1PublicKey;
use key_utils::Secp256k1SecretKey;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::Path;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PlebLotteryMiningServerConfig {
    pub listening_port: u16,
    pub pub_key: Secp256k1PublicKey,
    pub priv_key: Secp256k1SecretKey,
    pub cert_validity: u64,
    pub inactivity_limit: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PlebLotteryTemplateDistributionClientConfig {
    pub server_addr: SocketAddr,
    pub auth_pk: Option<Secp256k1PublicKey>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PleblotteryConfig {
    pub mining_server_config: PlebLotteryMiningServerConfig,
    pub template_distribution_config: PlebLotteryTemplateDistributionClientConfig,
}

/// The intention here is to expose the configs without exposing the private key
/// and other sensitive information.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PlebLotteryMiningServerConfigView {
    pub listening_port: u16,
    pub pub_key: Secp256k1PublicKey,
    pub cert_validity: u64,
    pub inactivity_limit: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PleblotteryConfigView {
    pub mining_server_config: PlebLotteryMiningServerConfigView,
    pub template_distribution_config: PlebLotteryTemplateDistributionClientConfig,
}

impl From<PlebLotteryMiningServerConfig> for PlebLotteryMiningServerConfigView {
    fn from(config: PlebLotteryMiningServerConfig) -> Self {
        PlebLotteryMiningServerConfigView {
            listening_port: config.listening_port,
            pub_key: config.pub_key,
            cert_validity: config.cert_validity,
            inactivity_limit: config.inactivity_limit,
        }
    }
}

impl From<PleblotteryConfig> for PleblotteryConfigView {
    fn from(config: PleblotteryConfig) -> Self {
        PleblotteryConfigView {
            mining_server_config: PlebLotteryMiningServerConfigView::from(config.mining_server_config),
            template_distribution_config: config.template_distribution_config,
        }
    }
}

impl PleblotteryConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }
}
