// verified [10/02/2024] with
// https://docs.chain.link/ccip/supported-networks/v1_2_0/mainnet#ethereum-mainnet-base-mainnet

use crate::types::*;
use alloy_chains::NamedChain;
use crate::tokens::*;

pub const MAX_DATA_LENGTH: u64 = 50000; // bytes
pub const MESSAGE_GAS_LIMIT: u64 = 2_000_000;
pub const MAX_TOKENS_CONCURRENT: u64 = 5;

// Networks data v1.2
pub const MAINNET_ROUTER: &str = "0x80226fc0Ee2b096224EeAc085Bb9a8cba1146f7D";
pub const MAINNET_SELECTOR: i128 = 5009297550715157269;

pub const OPTIMISM_ROUTER: &str = "0x3206695CaE29952f4b0c22a169725a865bc8Ce0f";
pub const OPTIMISM_SELECTOR: i128 = 3734403246176062136;

pub const AVALANCHE_ROUTER: &str = "0x27F39D0af3303703750D4001fCc1844c6491563c";
pub const AVALANCHE_SELECTOR: i128 = 6433500567565415381;

pub const ARBITRUM_ROUTER: &str = "0x141fa059441E0ca23ce184B6A78bafD2A517DdE8";
pub const ARBITRUM_SELECTOR: i128 = 4949039107694359620;

pub const POLYGON_ROUTER: &str = "0x849c5ED5a80F5B408Dd4969b78c2C8fdf0565Bfe";
pub const POLYGON_SELECTOR: i128 = 4051577828743386545;

pub const AVALANCHE_ROUTER: &str = "0xF4c7E640EdA248ef95972845a62bdC74237805dB";
pub const AVALANCHE_SELECTOR: i128 = 6433500567565415381;

pub const BNB_ROUTER: &str = "0x34B03Cb9086d7D758AC55af71584F81A598759FE";
pub const BNB_SELECTOR: i128 = 11344663589394136015;

pub const BASE_ROUTER: &str = "0x881e3A65B4d4a04dD529061dd0071cf975F58bCD";
pub const BASE_SELECTOR: i128 = 15971525489660198786;

// Testnets v1.2
pub const SEPOLIA_ROUTER: &str = "0x0BF3dE8c5D3e8A2B34D2BEeB17ABfCeBaf363A59";
pub const SEPOLIA_SELECTOR: i128 = 16015286601757825753;

pub const OPTIMISM_GOERLI_ROUTER: &str = "0xcc5a0B910D9E9504A7561934bed294c51285a78D";
pub const OPTIMISM_GOERLI_SELECTOR: i128 = 2664363617261496610;

pub const OPTIMISM_SEPOLIA_ROUTER: &str = "0x114a20a10b43d4115e5aeef7345a1a71d2a60c57";
pub const OPTIMISM_SEPOLIA_SELECTOR: i128 = 5224473277236331295;

pub const POLYGON_MUMBAI_ROUTER: &str = "0x1035CabC275068e0F4b745A29CEDf38E13aF41b1";
pub const POLYGON_MUMBAI_SELECTOR: i128 = 12532609583862916517;

pub const BNB_TESNET_ROUTER: &str = "0xE1053aE1857476f36A3C62580FF9b016E8EE8F6f";
pub const BNB_TESNET_SELECTOR: i128 = 13264668187771770619;

pub const BASE_GOERLI_ROUTER: &str = "0x80AF2F44ed0469018922c9F483dc5A909862fdc2";
pub const BASE_GOERLI_SELECTOR: i128 = 5790810961207155433;

pub const AVALANCHE_FUJI_ROUTER: &str = "0xF694E193200268f9a4868e4Aa017A0118C9a8177";
pub const AVALANCHE_FUJI_SELECTOR: i128 = 14767482510784806043;

pub const ARBITRUM_SEPOLIA_ROUTER: &str = "0x2a9C5afB0d0e4BAb2BCdaE109EC4b0c4Be15a165";
pub const ARBITRUM_SEPOLIA_SELECTOR: i128 = 3478487238524512106;

// Lanes

lazy_static!{

    pub static ref SEPOLIA_TOKENS: Vec<TokenCcip> = vec![
        LINK_SEPOLIA,
        CCIP_BNM_SEPOLIA,
        CCIP_LNM_SEPOLIA
    ];

    pub static ref OPTIMISM_GOERLI_TOKENS: Vec<TokenCcip> = vec![
        LINK_OPTIMISM_GOERLI,
        CCIP_BNM_OPTIMISM_GOERLI,
        CCIP_LNM_OPTIMISM_GOERLI
    ];

    pub static ref AVALANCHE_FUJI_TOKENS: Vec<TokenCcip> = vec![
        LINK_AVALANCHE_FUJI,
        CCIP_BNM_AVALANCHE_FUJI,
        CCIP_LNM_AVALANCHE_FUJI
    ];

    pub static ref ARBITRUM_GOERLI_TOKENS: Vec<TokenCcip> = vec![
        LINK_ARBITRUM_GOERLI,
        CCIP_BNM_ARBITRUM_GOERLI,
        CCIP_LNM_ARBITRUM_GOERLI
    ];

    pub static ref POLYGON_MUMBAI_TOKENS: Vec<TokenCcip> = vec![
        LINK_POLYGON_MUMBAI,
        CCIP_BNM_POLYGON_MUMBAI,
        CCIP_LNM_POLYGON_MUMBAI
    ];

    pub static ref MAINNET_OPTIMISM_LANE: Lane = {
        Lane {
            source: NamedChain::Mainnet,
            destination: NamedChain::Optimism,
            supported_tokens: Some(vec![SNXUSD_MAINNET_OPTIMISM]),
        } 
    };

    pub static ref SEPOLIA_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(SEPOLIA_TOKENS.clone())
        }
    };

    pub static ref SEPOLIA_ARBITRUM_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::ArbitrumGoerli,
            supported_tokens: Some(SEPOLIA_TOKENS.clone())
        }
    };

    pub static ref SEPOLIA_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(SEPOLIA_TOKENS.clone())
        }
    };

    pub static ref SEPOLIA_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(SEPOLIA_TOKENS.clone())
        }
    };

    pub static ref SEPOLIA_BNB_TESNET_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![LINK_SEPOLIA])
        }
    };

    pub static ref SEPOLIA_BASE_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::Sepolia,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![LINK_SEPOLIA])
        }
    };

    pub static ref OPTIMISM_MAINNET_LANE: Lane = {
        Lane {
            source: NamedChain::Optimism,
            destination: NamedChain::Mainnet,
            supported_tokens: Some(vec![SNXUSD_MAINNET_OPTIMISM]),
        } 
    };

    pub static ref OPTIMISM_GOERLI_ARBITRUM_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::ArbitrumGoerli,
            supported_tokens: Some(OPTIMISM_GOERLI_TOKENS.clone()),
        } 
    };

    pub static ref OPTIMISM_GOERLI_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(OPTIMISM_GOERLI_TOKENS.clone()),
        } 
    };

    pub static ref AVALANCHE_FUJI_SEPOLIA_LANE: Lane = {
        Lane {
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(AVALANCHE_FUJI_TOKENS.clone()),
        } 
    };

    pub static ref AVALANCHE_FUJI_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(AVALANCHE_FUJI_TOKENS.clone()),
        } 
    };

    pub static ref AVALANCHE_FUJI_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(AVALANCHE_FUJI_TOKENS.clone()),
        } 
    };

    pub static ref ARBITRUM_GOERLI_SEPOLIA_LANE: Lane = {
        Lane {
            source: NamedChain::ArbitrumGoerli,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(ARBITRUM_GOERLI_TOKENS.clone()),
        } 
    };

    pub static ref ARBITRUM_GOERLI_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            source: NamedChain::ArbitrumGoerli,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(ARBITRUM_GOERLI_TOKENS.clone()),
        } 
    };

    pub static ref POLYGON_MUMBAI_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(POLYGON_MUMBAI_TOKENS.clone()),
        } 
    };

    pub static ref BNB_TESTNET_SEPOLIA_LANE: Lane = {
        Lane {
            source: NamedChain::BinanceSmartChainTestnet,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![LINK_BNB_TESTNET]),
        } 
    };

    pub static ref BASE_GOERLI_SEPOLIA_LANE: Lane = {
        Lane {
            source: NamedChain::BaseGoerli,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![LINK_BASE_GOERLI]),
        } 
    };

}

pub const MAINNET_POLYGON_LANE: Lane = Lane {
    source: NamedChain::Mainnet,
    destination: NamedChain::Polygon,
    supported_tokens: None, 
};

pub const MAINNET_AVALANCHE_LANE: Lane = Lane {
    source: NamedChain::Mainnet,
    destination: NamedChain::Avalanche,
    supported_tokens: None, 
};

pub const AVALANCHE_MAINNET_LANE: Lane = Lane {
    source: NamedChain::Avalanche,
    destination: NamedChain::Mainnet,
    supported_tokens: None, 
};

pub const AVALANCHE_POLYGON_LANE: Lane = Lane {
    source: NamedChain::Mainnet,
    destination: NamedChain::Polygon,
    supported_tokens: None, 
};

pub const POLYGON_MAINNET_LANE: Lane = Lane {
    source: NamedChain::Polygon,
    destination: NamedChain::Mainnet,
    supported_tokens: None, 
};

pub const POLYGON_AVALANCHE_LANE: Lane = Lane {
    source: NamedChain::Polygon,
    destination: NamedChain::Avalanche,
    supported_tokens: None, 
};