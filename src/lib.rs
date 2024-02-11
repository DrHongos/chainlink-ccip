#[macro_use]
extern crate lazy_static;

pub mod constants;
pub mod types;
pub mod tokens;
pub mod fee_tokens;

use constants::*;
use fee_tokens::*;
use eyre::Result;
use types::FeeToken;
use crate::types::Lane;
use alloy_primitives::Address;
use alloy_chains::NamedChain;

// GETTERS FOR CONSTANTS
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
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_SEPOLIA_ROUTER),
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
        NamedChain::ArbitrumGoerli => Ok(ARBITRUM_SEPOLIA_SELECTOR),
        NamedChain::Polygon => Ok(POLYGON_SELECTOR),
        NamedChain::PolygonMumbai => Ok(POLYGON_MUMBAI_SELECTOR),
        NamedChain::BinanceSmartChainTestnet => Ok(BNB_TESNET_SELECTOR),
        NamedChain::BaseGoerli => Ok(BASE_GOERLI_SELECTOR),
        _ => Err(eyre::eyre!("Unsupported network"))
    }    
}

// shouldn't be needed anymore (alloy-chains)
pub fn get_chain(chain_name: &str) -> Result<NamedChain> {
    match chain_name.to_ascii_lowercase().as_ref() {
        "mainnet" => Ok(NamedChain::Mainnet),
        "optimism" => Ok(NamedChain::Optimism),
        "optimism-goerli" => Ok(NamedChain::OptimismGoerli),
        "sepolia" => Ok(NamedChain::Sepolia),
        "avalanche" => Ok(NamedChain::Avalanche),
        "avalanche-fuji" => Ok(NamedChain::AvalancheFuji),
        "arbitrum-sepolia" => Ok(NamedChain::ArbitrumSepolia),
        "polygon" => Ok(NamedChain::Polygon),
        "polygon-mumbai" => Ok(NamedChain::PolygonMumbai),
        "bnb-testnet" => Ok(NamedChain::BinanceSmartChainTestnet),
        "base-goerli" => Ok(NamedChain::BaseGoerli),
        _ => {
            Err(eyre::eyre!("Chain unknown {chain_name}"))
        }
    }
}

// add all lanes!
pub fn get_lane(source: NamedChain, destination: NamedChain) -> Result<Lane> {
    match (source, destination) {
        (NamedChain::Mainnet, NamedChain::Optimism) => Ok(MAINNET_OPTIMISM_LANE.clone()),
        (NamedChain::Mainnet, NamedChain::Polygon) => Ok(MAINNET_POLYGON_LANE.clone()),
        (NamedChain::Mainnet, NamedChain::Base) => Ok(MAINNET_BASE_LANE.clone()),
        (NamedChain::Mainnet, NamedChain::Arbitrum) => Ok(MAINNET_ARBITRUM_LANE.clone()),
        (NamedChain::Mainnet, NamedChain::BinanceSmartChain) => Ok(MAINNET_BNB_LANE.clone()),
        (NamedChain::Mainnet, NamedChain::Avalanche) => Ok(MAINNET_AVALANCHE_LANE.clone()),
        (NamedChain::Optimism, NamedChain::Mainnet) => Ok(OPTIMISM_MAINNET_LANE.clone()),
        (NamedChain::Optimism, NamedChain::Polygon) => Ok(OPTIMISM_POLYGON_LANE.clone()),
        (NamedChain::Optimism, NamedChain::Base) => Ok(OPTIMISM_BASE_LANE.clone()),
        (NamedChain::Arbitrum, NamedChain::Mainnet) => Ok(ARBITRUM_MAINNET_LANE.clone()),       
        (NamedChain::Arbitrum, NamedChain::Base) => Ok(ARBITRUM_BASE_LANE.clone()),       
        (NamedChain::Polygon, NamedChain::Mainnet) => Ok(POLYGON_MAINNET_LANE.clone()),
        (NamedChain::Polygon, NamedChain::Avalanche) => Ok(POLYGON_AVALANCHE_LANE.clone()),
        (NamedChain::Polygon, NamedChain::Optimism) => Ok(POLYGON_OPTIMISM_LANE.clone()),
        (NamedChain::Polygon, NamedChain::BinanceSmartChain) => Ok(POLYGON_BNB_LANE.clone()),
        (NamedChain::Avalanche, NamedChain::Mainnet) => Ok(AVALANCHE_MAINNET_LANE.clone()),
        (NamedChain::Avalanche, NamedChain::BinanceSmartChain) => Ok(AVALANCHE_BNB_LANE.clone()),
        (NamedChain::Avalanche, NamedChain::Polygon) => Ok(AVALANCHE_POLYGON_LANE.clone()),
        (NamedChain::BinanceSmartChain, NamedChain::Mainnet) => Ok(BNB_MAINNET_LANE.clone()),
        (NamedChain::BinanceSmartChain, NamedChain::Avalanche) => Ok(BNB_AVALANCHE_LANE.clone()),
        (NamedChain::BinanceSmartChain, NamedChain::Polygon) => Ok(BNB_POLYGON_LANE.clone()),
        (NamedChain::BinanceSmartChain, NamedChain::Base) => Ok(BNB_BASE_LANE.clone()),
        (NamedChain::Base, NamedChain::Mainnet) => Ok(BASE_MAINNET_LANE.clone()),
        (NamedChain::Base, NamedChain::Optimism) => Ok(BASE_OPTIMISM_LANE.clone()),
        (NamedChain::Base, NamedChain::BinanceSmartChain) => Ok(BASE_BNB_LANE.clone()),
        (NamedChain::Base, NamedChain::Arbitrum) => Ok(BASE_ARBITRUM_LANE.clone()),
        // tesnets
        (NamedChain::Sepolia, NamedChain::OptimismGoerli) => Ok(SEPOLIA_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::ArbitrumSepolia) => Ok(SEPOLIA_ARBITRUM_SEPOLIA_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::AvalancheFuji) => Ok(SEPOLIA_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::PolygonMumbai) => Ok(SEPOLIA_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::BinanceSmartChainTestnet) => Ok(SEPOLIA_BNB_TESNET_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::BaseGoerli) => Ok(SEPOLIA_BASE_GOERLI_LANE.clone()),
        (NamedChain::Sepolia, NamedChain::OptimismSepolia) => Ok(SEPOLIA_OPTIMISM_SEPOLIA_LANE.clone()),
        (NamedChain::OptimismSepolia, NamedChain::Sepolia) => Ok(OPTIMISM_SEPOLIA_SEPOLIA_LANE.clone()),
        (NamedChain::OptimismSepolia, NamedChain::ArbitrumSepolia) => Ok(OPTIMISM_SEPOLIA_ARBITRUM_SEPOLIA_LANE.clone()),
        (NamedChain::OptimismSepolia, NamedChain::AvalancheFuji) => Ok(OPTIMISM_SEPOLIA_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::OptimismSepolia, NamedChain::PolygonMumbai) => Ok(OPTIMISM_SEPOLIA_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::Sepolia) => Ok(OPTIMISM_GOERLI_SEPOLIA_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::AvalancheFuji) => Ok(OPTIMISM_GOERLI_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::PolygonMumbai) => Ok(OPTIMISM_GOERLI_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::OptimismGoerli, NamedChain::BaseGoerli) => Ok(OPTIMISM_GOERLI_BASE_GOERLI_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::Sepolia) => Ok(AVALANCHE_FUJI_SEPOLIA_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::OptimismGoerli) => Ok(AVALANCHE_FUJI_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::PolygonMumbai) => Ok(AVALANCHE_FUJI_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::BinanceSmartChainTestnet) => Ok(AVALANCHE_FUJI_BNB_TESNET_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::OptimismSepolia) => Ok(AVALANCHE_FUJI_OPTIMISM_SEPOLIA_LANE.clone()),
        (NamedChain::AvalancheFuji, NamedChain::BaseGoerli) => Ok(AVALANCHE_FUJI_BASE_GOERLI_LANE.clone()),
        (NamedChain::ArbitrumSepolia, NamedChain::Sepolia) => Ok(ARBITRUM_SEPOLIA_SEPOLIA_LANE.clone()),
        (NamedChain::ArbitrumSepolia, NamedChain::OptimismSepolia) => Ok(ARBITRUM_SEPOLIA_OPTIMISM_SEPOLIA_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::AvalancheFuji) => Ok(POLYGON_MUMBAI_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::Sepolia) => Ok(POLYGON_MUMBAI_SEPOLIA_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::OptimismGoerli) => Ok(POLYGON_MUMBAI_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::OptimismSepolia) => Ok(POLYGON_MUMBAI_OPTIMISM_SEPOLIA_LANE.clone()),
        (NamedChain::PolygonMumbai, NamedChain::BinanceSmartChainTestnet) => Ok(POLYGON_MUMBAI_BNB_TESTNET_LANE.clone()),
        (NamedChain::BinanceSmartChainTestnet, NamedChain::Sepolia) => Ok(BNB_TESTNET_SEPOLIA_LANE.clone()),
        (NamedChain::BinanceSmartChainTestnet, NamedChain::AvalancheFuji) => Ok(BNB_TESTNET_AVALANCHE_FUJI_LANE.clone()),
        (NamedChain::BinanceSmartChainTestnet, NamedChain::PolygonMumbai) => Ok(BNB_TESTNET_POLYGON_MUMBAI_LANE.clone()),
        (NamedChain::BinanceSmartChainTestnet, NamedChain::BaseGoerli) => Ok(BNB_TESTNET_BASE_GOERLI_LANE.clone()),
        (NamedChain::BaseGoerli, NamedChain::Sepolia) => Ok(BASE_GOERLI_SEPOLIA_LANE.clone()),
        (NamedChain::BaseGoerli, NamedChain::OptimismGoerli) => Ok(BASE_GOERLI_OPTIMISM_GOERLI_LANE.clone()),
        (NamedChain::BaseGoerli, NamedChain::BinanceSmartChainTestnet) => Ok(BASE_GOERLI_BNB_TESNET_LANE.clone()),
        (NamedChain::BaseGoerli, NamedChain::AvalancheFuji) => Ok(BASE_GOERLI_AVALANCHE_FUJI_LANE.clone()),
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