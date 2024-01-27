#[macro_use]
extern crate lazy_static;

pub mod constants;
pub mod types;

use constants::*;
use eyre::Result;
use types::{TokenCcip, FeeToken};
use crate::types::Lane;
use alloy_primitives::Address;
use alloy_chains::NamedChain;

// GETTERS FOR CONSTANTS
pub fn get_provider_rpc_url(chain: NamedChain) -> Result<String> {
    match chain {
        NamedChain::Mainnet => Ok(String::from("https://mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::Sepolia => Ok(String::from("https://sepolia.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::Polygon => Ok(String::from("https://polygon-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::PolygonMumbai => Ok(String::from("https://polygon-mumbai.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::Optimism => Ok(String::from("https://optimism-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::OptimismGoerli => Ok(String::from("https://optimism-goerli.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::Arbitrum => Ok(String::from("https://arbitrum-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::ArbitrumGoerli => Ok(String::from("https://arbitrum-goerli.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::Avalanche => Ok(String::from("https://avalanche-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::AvalancheFuji => Ok(String::from("https://avalanche-fuji.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        NamedChain::BinanceSmartChainTestnet => Ok(String::from("https://data-seed-prebsc-1-s1.binance.org:8545/")),
        NamedChain::BaseGoerli => Ok(String::from("https://base-goerli.blockpi.network/v1/rpc/public	")),
        _ => Err(eyre::eyre!("Chain has no RPC URL")) 
    }
}

pub fn get_tokens_tests(chain: &NamedChain) -> Result<Vec<TokenCcip>> {
    match chain {
        NamedChain::Mainnet => Ok(vec![SNXUSD_MAINNET_OPTIMISM]),
        NamedChain::Optimism => Ok(vec![SNXUSD_MAINNET_OPTIMISM]),
        NamedChain::OptimismGoerli => Ok(OPTIMISM_GOERLI_TOKENS.clone()),
        NamedChain::Sepolia => Ok(SEPOLIA_TOKENS.clone()),
        NamedChain::Avalanche => Ok(Vec::new()),
        NamedChain::AvalancheFuji => Ok(AVALANCHE_FUJI_TOKENS.clone()),
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_TOKENS.clone()),
        NamedChain::Polygon => Ok(Vec::new()),
        NamedChain::PolygonMumbai => Ok(POLYGON_MUMBAI_TOKENS.clone()),
        NamedChain::BinanceSmartChainTestnet => Ok(vec![LINK_BNB_TESTNET]),
        NamedChain::BaseGoerli => Ok(vec![LINK_BASE_GOERLI]),
        _ => Err(eyre::eyre!("Unsupported network"))
    }
}

pub fn get_fee_tokens(chain: &NamedChain, selector: FeeToken) -> Result<Address>  {
    match chain {
        NamedChain::Mainnet => Ok(MAINNET_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::Optimism => Ok(OPTIMISM_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::OptimismGoerli => Ok(OPTIMISM_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::Sepolia => Ok(SEPOLIA_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::Avalanche => Ok(AVALANCHE_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::AvalancheFuji => Ok(AVALANCHE_FUJI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::Polygon => Ok(POLYGON_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::PolygonMumbai => Ok(POLYGON_MUMBAI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::BinanceSmartChainTestnet => Ok(BNB_TESNET_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        NamedChain::BaseGoerli => Ok(BASE_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        _ => Err(eyre::eyre!("Unsupported network"))
    }
}

pub fn get_router(chain: &NamedChain) -> Result<&str> {
    match chain {
        NamedChain::Mainnet => Ok(MAINNET_ROUTER),
        NamedChain::Optimism => Ok(OPTIMISM_ROUTER),
        NamedChain::OptimismGoerli => Ok(OPTIMISM_GOERLI_ROUTER),
        NamedChain::Sepolia => Ok(SEPOLIA_ROUTER),
        NamedChain::Avalanche => Ok(AVALANCHE_ROUTER),
        NamedChain::AvalancheFuji => Ok(AVALANCHE_FUJI_ROUTER),
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_ROUTER),
        NamedChain::Polygon => Ok(POLYGON_ROUTER),
        NamedChain::PolygonMumbai => Ok(POLYGON_MUMBAI_ROUTER),
        NamedChain::BinanceSmartChainTestnet => Ok(BNB_TESNET_ROUTER),
        NamedChain::BaseGoerli => Ok(BASE_GOERLI_ROUTER),
        _ => Err(eyre::eyre!("Unsupported network"))
    }    
}

pub fn get_selector(chain: &NamedChain) -> Result<i128> {
    match chain {
        NamedChain::Mainnet => Ok(MAINNET_SELECTOR),
        NamedChain::Optimism => Ok(OPTIMISM_SELECTOR),
        NamedChain::OptimismGoerli => Ok(OPTIMISM_GOERLI_SELECTOR),
        NamedChain::Sepolia => Ok(SEPOLIA_SELECTOR),
        NamedChain::Avalanche => Ok(AVALANCHE_SELECTOR),
        NamedChain::AvalancheFuji => Ok(AVALANCHE_FUJI_SELECTOR),
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_SELECTOR),
        NamedChain::Polygon => Ok(POLYGON_SELECTOR),
        NamedChain::PolygonMumbai => Ok(POLYGON_MUMBAI_SELECTOR),
        NamedChain::BinanceSmartChainTestnet => Ok(BNB_TESNET_SELECTOR),
        NamedChain::BaseGoerli => Ok(BASE_GOERLI_SELECTOR),
        _ => Err(eyre::eyre!("Unsupported network"))
    }    
}

pub fn get_chain(chain_name: &str) -> Result<NamedChain> {
    match chain_name.to_ascii_lowercase().as_ref() {
        "mainnet" => Ok(NamedChain::Mainnet),
        "optimism" => Ok(NamedChain::Optimism),
        "optimism-goerli" => Ok(NamedChain::OptimismGoerli),
        "sepolia" => Ok(NamedChain::Sepolia),
        "avalanche" => Ok(NamedChain::Avalanche),
        "avalanche-fuji" => Ok(NamedChain::AvalancheFuji),
        "arbitrum-goerli" => Ok(NamedChain::ArbitrumGoerli),
        "polygon" => Ok(NamedChain::Polygon),
        "polygon-mumbai" => Ok(NamedChain::PolygonMumbai),
        "bnb-testnet" => Ok(NamedChain::BinanceSmartChainTestnet),
        "base-goerli" => Ok(NamedChain::BaseGoerli),
        _ => {
            Err(eyre::eyre!("Chain unknown {chain_name}"))
        }
    }
}

pub fn get_lane(source: NamedChain, destination: NamedChain) -> Result<Lane> {
    match (source, destination) {
        (NamedChain::Mainnet, NamedChain::Optimism) => Ok(MAINNET_OPTIMISM_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::OptimismGoerli) => Ok(SEPOLIA_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::ArbitrumGoerli) => Ok(SEPOLIA_ARBITRUM_GOERLI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::AvalancheFuji) => Ok(SEPOLIA_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::PolygonMumbai) => Ok(SEPOLIA_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::BinanceSmartChainTestnet) => Ok(SEPOLIA_BNB_TESNET_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::BaseGoerli) => Ok(SEPOLIA_BASE_GOERLI_LANE.clone()),
        (NamedChain::Optimism, NamedChain::Mainnet) => Ok(OPTIMISM_MAINNET_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::ArbitrumGoerli) => Ok(OPTIMISM_GOERLI_ARBITRUM_GOERLI_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::AvalancheFuji) => Ok(OPTIMISM_GOERLI_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::Avalanche, NamedChain::Mainnet) => Ok(AVALANCHE_MAINNET_LANE.clone()),
        (NamedChain::Avalanche, NamedChain::Polygon) => Ok(AVALANCHE_POLYGON_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::Sepolia) => Ok(AVALANCHE_FUJI_SEPOLIA_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::OptimismGoerli) => Ok(AVALANCHE_FUJI_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::PolygonMumbai) => Ok(AVALANCHE_FUJI_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::ArbitrumGoerli, NamedChain::Sepolia) => Ok(ARBITRUM_GOERLI_SEPOLIA_LANE.clone()),
        (NamedChain::ArbitrumGoerli, NamedChain::OptimismGoerli) => Ok(ARBITRUM_GOERLI_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::Polygon, NamedChain::Mainnet) => Ok(POLYGON_MAINNET_LANE.clone()),
        (NamedChain::Polygon, NamedChain::Avalanche) => Ok(POLYGON_AVALANCHE_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::AvalancheFuji) => Ok(POLYGON_MUMBAI_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::BinanceSmartChainTestnet, NamedChain::Sepolia) => Ok(BNB_TESTNET_SEPOLIA_LANE.clone()),
        (NamedChain::BaseGoerli, NamedChain::Sepolia) => Ok(BASE_GOERLI_SEPOLIA_LANE.clone()),
        (_, _) => Err(eyre::eyre!("Lane not supported yet"))
    }
}

/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */