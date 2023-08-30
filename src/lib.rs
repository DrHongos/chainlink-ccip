#[macro_use]
extern crate lazy_static;

pub mod constants;
pub mod types;

use constants::*;
use eyre::Result;
use ethers::types::Chain;
use crate::types::Lane;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_router(chain: &Chain) -> Result<&str> {
    match chain {
        Chain::Mainnet => Ok(MAINNET_ROUTER),
        Chain::Optimism => Ok(OPTIMISM_ROUTER),
        Chain::OptimismGoerli => Ok(OPTIMISM_GOERLI_ROUTER),
        Chain::Sepolia => Ok(SEPOLIA_ROUTER),
        Chain::Avalanche => Ok(AVALANCHE_ROUTER),
        Chain::AvalancheFuji => Ok(AVALANCHE_FUJI_ROUTER),
        Chain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_ROUTER),
        Chain::Polygon => Ok(POLYGON_ROUTER),
        Chain::PolygonMumbai => Ok(POLYGON_MUMBAI_ROUTER),
        Chain::BinanceSmartChainTestnet => Ok(BNB_TESNET_ROUTER),
        Chain::BaseGoerli => Ok(BASE_GOERLI_ROUTER),
        _ => Err(eyre::eyre!("Unsupported network"))
    }    
}

pub fn get_selector(chain: &Chain) -> Result<i128> {
    match chain {
        Chain::Mainnet => Ok(MAINNET_SELECTOR),
        Chain::Optimism => Ok(OPTIMISM_SELECTOR),
        Chain::OptimismGoerli => Ok(OPTIMISM_GOERLI_SELECTOR),
        Chain::Sepolia => Ok(SEPOLIA_SELECTOR),
        Chain::Avalanche => Ok(AVALANCHE_SELECTOR),
        Chain::AvalancheFuji => Ok(AVALANCHE_FUJI_SELECTOR),
        Chain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_SELECTOR),
        Chain::Polygon => Ok(POLYGON_SELECTOR),
        Chain::PolygonMumbai => Ok(POLYGON_MUMBAI_SELECTOR),
        Chain::BinanceSmartChainTestnet => Ok(BNB_TESNET_SELECTOR),
        Chain::BaseGoerli => Ok(BASE_GOERLI_SELECTOR),
        _ => Err(eyre::eyre!("Unsupported network"))
    }    
}

pub fn get_chain(chain_name: &str) -> Result<Chain> {
    match chain_name.to_ascii_lowercase().as_ref() {
        "mainnet" => Ok(Chain::Mainnet),
        "optimism" => Ok(Chain::Optimism),
        "optimism-goerli" => Ok(Chain::OptimismGoerli),
        "sepolia" => Ok(Chain::Sepolia),
        "avalanche" => Ok(Chain::Avalanche),
        "avalanche-fuji" => Ok(Chain::AvalancheFuji),
        "arbitrum-goerli" => Ok(Chain::ArbitrumGoerli),
        "polygon" => Ok(Chain::Polygon),
        "polygon-mumbai" => Ok(Chain::PolygonMumbai),
        "bnb-testnet" => Ok(Chain::BinanceSmartChainTestnet),
        "base-goerli" => Ok(Chain::BaseGoerli),
        _ => {
            Err(eyre::eyre!("Chain unknown {chain_name}"))
        }
    }
}

pub fn get_lane(source: Chain, destination: Chain) -> Result<Lane> {
    match (source, destination) {
        (Chain::Mainnet, Chain::Optimism) => Ok(MAINNET_OPTIMISM_LANE.clone()),
        (Chain::Sepolia, Chain::OptimismGoerli) => Ok(SEPOLIA_OPTIMISM_GOERLI_LANE.clone()),
        (Chain::Sepolia, Chain::ArbitrumGoerli) => Ok(SEPOLIA_ARBITRUM_GOERLI_LANE.clone()),
        (Chain::Sepolia, Chain::AvalancheFuji) => Ok(SEPOLIA_AVALANCHE_FUJI_LANE.clone()),
        (Chain::Sepolia, Chain::PolygonMumbai) => Ok(SEPOLIA_POLYGON_MUMBAI_LANE.clone()),
        (Chain::Sepolia, Chain::BinanceSmartChainTestnet) => Ok(SEPOLIA_BNB_TESNET_LANE.clone()),
        (Chain::Sepolia, Chain::BaseGoerli) => Ok(SEPOLIA_BASE_GOERLI_LANE.clone()),
        (Chain::Optimism, Chain::Mainnet) => Ok(OPTIMISM_MAINNET_LANE.clone()),
        (Chain::OptimismGoerli, Chain::ArbitrumGoerli) => Ok(OPTIMISM_GOERLI_ARBITRUM_GOERLI_LANE.clone()),
        (Chain::OptimismGoerli, Chain::AvalancheFuji) => Ok(OPTIMISM_GOERLI_AVALANCHE_FUJI_LANE.clone()),
        (Chain::Avalanche, Chain::Mainnet) => Ok(AVALANCHE_MAINNET_LANE.clone()),
        (Chain::Avalanche, Chain::Polygon) => Ok(AVALANCHE_POLYGON_LANE.clone()),
        (Chain::AvalancheFuji, Chain::Sepolia) => Ok(AVALANCHE_FUJI_SEPOLIA_LANE.clone()),
        (Chain::AvalancheFuji, Chain::OptimismGoerli) => Ok(AVALANCHE_FUJI_OPTIMISM_GOERLI_LANE.clone()),
        (Chain::AvalancheFuji, Chain::PolygonMumbai) => Ok(AVALANCHE_FUJI_POLYGON_MUMBAI_LANE.clone()),
        (Chain::ArbitrumGoerli, Chain::Sepolia) => Ok(ARBITRUM_GOERLI_SEPOLIA_LANE.clone()),
        (Chain::ArbitrumGoerli, Chain::OptimismGoerli) => Ok(ARBITRUM_GOERLI_OPTIMISM_GOERLI_LANE.clone()),
        (Chain::Polygon, Chain::Mainnet) => Ok(POLYGON_MAINNET_LANE.clone()),
        (Chain::Polygon, Chain::Avalanche) => Ok(POLYGON_AVALANCHE_LANE.clone()),
        (Chain::PolygonMumbai, Chain::AvalancheFuji) => Ok(POLYGON_MUMBAI_AVALANCHE_FUJI_LANE.clone()),
        (Chain::BinanceSmartChainTestnet, Chain::Sepolia) => Ok(BNB_TESTNET_SEPOLIA_LANE.clone()),
        (Chain::BaseGoerli, Chain::Sepolia) => Ok(BASE_GOERLI_SEPOLIA_LANE.clone()),
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