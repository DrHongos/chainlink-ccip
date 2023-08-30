use ethers::types::Chain;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum FeeToken {
    Native,
    WrappedNative,
    Link
}

#[derive(Debug, Clone)]
pub struct TokenCcip {
    pub name: &'static str,
    pub address: &'static str,
    pub decimals: u8,
    pub capacity_usd: u64,
    pub refill_per_second: u64,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub source: Chain,
    pub destination: Chain,
    pub supported_tokens: Option<Vec<TokenCcip>>,
}