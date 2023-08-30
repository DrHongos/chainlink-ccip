#[macro_use]
extern crate lazy_static;

pub mod constants;
pub mod types;

use constants::*;
use eyre::Result;
use ethers::{
    types::Chain,
    prelude::*, utils::{format_units, format_ether},
};
use types::{TokenCcip, FeeToken};
use crate::types::Lane;
use std::sync::Arc;

abigen!(ERC20, "./abis/ERC20.json");
abigen!(BNM_ERC677, "./abis/BurnMintERC677Helper.json");

//
pub async fn get_status_on_chain(pk: String, chain: Chain) -> Result<()> {
    let signer: LocalWallet = pk.parse()?;
    let account = signer.address();
    println!("Chain: {}", chain);    
    println!("Account: {}", format!("{:?}", account));
    let provider_rpc_url = get_provider_rpc_url(chain).expect("No RPC URL found for {chain}");
    let provider = Arc::new(Provider::<Http>::try_from(provider_rpc_url)?);
    //let mut client = SignerMiddleware::new(provider, signer);

    // gas tokens balances
    let native_balance = provider.get_balance(account, None).await?;
    println!("Balance of native: {}", format_ether(native_balance));

    // find balances
        // other fee tokens (link and wrapped native)
    //let wrapped_native_token_address = get_fee_tokens(&chain, FeeToken::WrappedNative)?;
    //let link_address = get_fee_tokens(&chain, FeeToken::Link)?;
    
    // test tokens balances
    let tokens_chain = get_tokens_tests(&chain)?;
    for t in  tokens_chain {
        let b = get_balance_of(account, provider.clone(), t.address.parse()?).await?;
        println!("Balance of {}: {}", 
            t.name,
            format_units(b, t.decimals).unwrap()
        );
    }

    Ok(())
}

pub async fn get_balance_of(account: Address, provider: Arc<Provider<Http>>, token: Address) -> Result<U256> {
    let contract = ERC20::new(token, provider);
    let balance = contract.balance_of(account).await?;    
    
    Ok(balance)
} 


// GETTERS FOR CONSTANTS
pub fn get_provider_rpc_url(chain: Chain) -> Result<String> {
    match chain {
        Chain::Mainnet => Ok(String::from("https://mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::Sepolia => Ok(String::from("https://sepolia.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::Polygon => Ok(String::from("https://polygon-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::PolygonMumbai => Ok(String::from("https://polygon-mumbai.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::Optimism => Ok(String::from("https://optimism-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::OptimismGoerli => Ok(String::from("https://optimism-goerli.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::Arbitrum => Ok(String::from("https://arbitrum-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::ArbitrumGoerli => Ok(String::from("https://arbitrum-goerli.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::Avalanche => Ok(String::from("https://avalanche-mainnet.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::AvalancheFuji => Ok(String::from("https://avalanche-fuji.infura.io/v3/88371c5dbe284f97bb2789cf7f9ca6f1")),
        Chain::BinanceSmartChainTestnet => Ok(String::from("https://data-seed-prebsc-1-s1.binance.org:8545/")),
        Chain::BaseGoerli => Ok(String::from("https://base-goerli.blockpi.network/v1/rpc/public	")),
        _ => Err(eyre::eyre!("Chain has no RPC URL")) 
    }
}

pub fn get_tokens_tests(chain: &Chain) -> Result<Vec<TokenCcip>> {
    match chain {
        Chain::Mainnet => Ok(vec![SNXUSD_MAINNET_OPTIMISM]),
        Chain::Optimism => Ok(vec![SNXUSD_MAINNET_OPTIMISM]),
        Chain::OptimismGoerli => Ok(OPTIMISM_GOERLI_TOKENS.clone()),
        Chain::Sepolia => Ok(SEPOLIA_TOKENS.clone()),
        Chain::Avalanche => Ok(Vec::new()),
        Chain::AvalancheFuji => Ok(AVALANCHE_FUJI_TOKENS.clone()),
        Chain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_TOKENS.clone()),
        Chain::Polygon => Ok(Vec::new()),
        Chain::PolygonMumbai => Ok(POLYGON_MUMBAI_TOKENS.clone()),
        Chain::BinanceSmartChainTestnet => Ok(vec![LINK_BNB_TESTNET]),
        Chain::BaseGoerli => Ok(vec![LINK_BASE_GOERLI]),
        _ => Err(eyre::eyre!("Unsupported network"))
    }
}

pub fn get_fee_tokens(chain: &Chain, selector: FeeToken) -> Result<Address>  {
    match chain {
        Chain::Mainnet => Ok(MAINNET_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::Optimism => Ok(OPTIMISM_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::OptimismGoerli => Ok(OPTIMISM_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::Sepolia => Ok(SEPOLIA_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::Avalanche => Ok(AVALANCHE_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::AvalancheFuji => Ok(AVALANCHE_FUJI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::ArbitrumGoerli => Ok(ARBITRUM_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::Polygon => Ok(POLYGON_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::PolygonMumbai => Ok(POLYGON_MUMBAI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::BinanceSmartChainTestnet => Ok(BNB_TESNET_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        Chain::BaseGoerli => Ok(BASE_GOERLI_FEE_TOKENS.get(&selector).expect("Not existent").clone()),
        _ => Err(eyre::eyre!("Unsupported network"))
    }
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