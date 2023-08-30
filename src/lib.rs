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