use alloy_chains::NamedChain;
use alloy_primitives::{Address, U256, B256, Bytes};

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
    pub decimals: usize,
    pub capacity_usd: u64,
    pub refill_per_second: f64,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub source: NamedChain,
    pub destination: NamedChain,
    pub supported_tokens: Option<Vec<TokenCcip>>,
}

// Client types (https://docs.chain.link/ccip/api-reference/client#types-and-constants)
#[derive(Debug, Clone)]
pub struct EVMTokenAmount {
    pub token: Address,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub struct Any2EVMMessage {
    pub message_id: B256,
    pub source_chain_selector: u128,
    pub sender: Address,
    pub data: Bytes,
    pub dest_token_amounts: Vec<EVMTokenAmount>
}

#[derive(Debug, Clone)]
pub struct EVM2AnyMessage {
    pub receiver: Address,
    pub data: Bytes,
    pub token_amounts: Vec<EVMTokenAmount>,
    pub fee_token: Address,
    pub extra_args: Bytes
}

#[derive(Debug, Clone)]
pub struct EVMExtraArgsV1 {
    pub gas_limit: U256,
    pub strict: bool
} 