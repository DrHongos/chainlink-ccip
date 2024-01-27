// taken from https://docs.chain.link/ccip/supported-networks
use crate::types::*;
use std::collections::HashMap;
use alloy_chains::NamedChain;
use alloy_primitives::Address;

pub const MAX_DATA_LENGTH: u64 = 50000; // bytes
pub const MESSAGE_GAS_LIMIT: u64 = 2_000_000;
pub const MAX_TOKENS_CONCURRENT: u64 = 5;

// Networks data
pub const MAINNET_ROUTER: &str = "0xE561d5E02207fb5eB32cca20a699E0d8919a1476";
pub const MAINNET_SELECTOR: i128 = 5009297550715157269;

pub const SEPOLIA_ROUTER: &str = "0xD0daae2231E9CB96b94C8512223533293C3693Bf";
pub const SEPOLIA_SELECTOR: i128 = 16015286601757825753;

pub const OPTIMISM_ROUTER: &str = "0x261c05167db67B2b619f9d312e0753f3721ad6E8";
pub const OPTIMISM_SELECTOR: i128 = 3734403246176062136;

pub const OPTIMISM_GOERLI_ROUTER: &str = "0xEB52E9Ae4A9Fb37172978642d4C141ef53876f26";
pub const OPTIMISM_GOERLI_SELECTOR: i128 = 2664363617261496610;

pub const AVALANCHE_ROUTER: &str = "0x27F39D0af3303703750D4001fCc1844c6491563c";
pub const AVALANCHE_SELECTOR: i128 = 6433500567565415381;

pub const AVALANCHE_FUJI_ROUTER: &str = "0x554472a2720E5E7D5D3C817529aBA05EEd5F82D8";
pub const AVALANCHE_FUJI_SELECTOR: i128 = 14767482510784806043;

pub const ARBITRUM_GOERLI_ROUTER: &str = "0x88E492127709447A5ABEFdaB8788a15B4567589E";
pub const ARBITRUM_GOERLI_SELECTOR: i128 = 6101244977088475029;

pub const POLYGON_ROUTER: &str = "0x3C3D92629A02a8D95D5CB9650fe49C3544f69B43";
pub const POLYGON_SELECTOR: i128 = 4051577828743386545;

pub const POLYGON_MUMBAI_ROUTER: &str = "0x70499c328e1E2a3c41108bd3730F6670a44595D1";
pub const POLYGON_MUMBAI_SELECTOR: i128 = 12532609583862916517;

pub const BNB_TESNET_ROUTER: &str = "0x9527e2d01a3064ef6b50c1da1c0cc523803bcff2";
pub const BNB_TESNET_SELECTOR: i128 = 13264668187771770619;

pub const BASE_GOERLI_ROUTER: &str = "0xa8c0c11bf64af62cdca6f93d3769b88bdd7cb93d";
pub const BASE_GOERLI_SELECTOR: i128 = 5790810961207155433;

lazy_static! {
    pub static ref MAINNET_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap());
        h.insert(FeeToken::Link, "0x514910771AF9Ca656af840dff83E8264EcF986CA".parse().unwrap());
        h
    };

    pub static ref SEPOLIA_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x097D90c9d3E0B50Ca60e1ae45F6A81010f9FB534".parse().unwrap());
        h.insert(FeeToken::Link, "0x779877A7B0D9E8603169DdbD7836e478b4624789".parse().unwrap());
        h
    };

    pub static ref OPTIMISM_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0x350a791Bfc2C21F9Ed5d10980Dad2e2638ffa7f6".parse().unwrap());
        h
    };

    pub static ref OPTIMISM_GOERLI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0xdc2CC710e42857672E7907CF474a69B63B93089f".parse().unwrap());
        h
    };

    pub static ref AVALANCHE_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7".parse().unwrap());
        h.insert(FeeToken::Link, "0x5947BB275c521040051D82396192181b413227A3".parse().unwrap());
        h
    };

    pub static ref AVALANCHE_FUJI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xd00ae08403B9bbb9124bB305C09058E32C39A48c".parse().unwrap());
        h.insert(FeeToken::Link, "0x0b9d5D9136855f6FEc3c0993feE6E9CE8a297846".parse().unwrap());
        h
    };

    pub static ref POLYGON_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270".parse().unwrap());
        h.insert(FeeToken::Link, "0xb0897686c545045aFc77CF20eC7A532E3120E0F1".parse().unwrap());
        h
    };

    pub static ref POLYGON_MUMBAI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889".parse().unwrap());
        h.insert(FeeToken::Link, "0x326C977E6efc84E512bB9C30f76E30c160eD06FB".parse().unwrap());
        h
    };

    pub static ref BNB_TESNET_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd".parse().unwrap());
        h.insert(FeeToken::Link, "0x84b9B910527Ad5C03A9Ca831909E21e236EA7b06".parse().unwrap());
        h
    };

    pub static ref BASE_GOERLI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0xd886e2286fd1073df82462ea1822119600af80b6".parse().unwrap());
        h
    };

    pub static ref ARBITRUM_GOERLI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x32d5D5978905d9c6c2D4C417F0E06Fe768a4FB5a".parse().unwrap());
        h.insert(FeeToken::Link, "0xd14838A68E8AFBAdE5efb411d5871ea0011AFd28".parse().unwrap());
        h
    };
}

// Tokens
pub const SNXUSD_MAINNET_OPTIMISM: TokenCcip = TokenCcip {
    name: "snxUSD",
    address: "0xb2F30A7C980f052f02563fb518dcc39e6bf38175",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const LINK_SEPOLIA: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x779877A7B0D9E8603169DdbD7836e478b4624789",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const CCIP_BNM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xFd57b4ddBf88a4e07fF4e34C487b99af2Fe82a05",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const CCIP_LNM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x466D489b6d36E7E3b824ef491C225F5830E81cC1",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};


pub const LINK_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xdc2CC710e42857672E7907CF474a69B63B93089f",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const CCIP_BNM_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xaBfE9D11A2f1D61990D1d253EC98B5Da00304F16",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const CCIP_LNM_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0x835833d556299CdEC623e7980e7369145b037591",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const LINK_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x0b9d5D9136855f6FEc3c0993feE6E9CE8a297846",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const CCIP_BNM_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xD21341536c5cF5EB1bcb58f6723cE26e8D8E90e4",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const CCIP_LNM_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0xD21341536c5cF5EB1bcb58f6723cE26e8D8E90e4",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const LINK_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xd14838A68E8AFBAdE5efb411d5871ea0011AFd28",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const CCIP_BNM_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0x0579b4c1C8AcbfF13c6253f1B10d66896Bf399Ef",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const CCIP_LNM_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0x0E14dBe2c8e1121902208be173A3fb91Bb125CDB",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const LINK_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x326C977E6efc84E512bB9C30f76E30c160eD06FB",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const CCIP_BNM_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xf1E3A5842EeEF51F2967b3F05D45DD4f4205FF40",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const CCIP_LNM_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0xc1c76a8c5bFDE1Be034bbcD930c668726E7C1987",
    decimals: 18,
    capacity_usd: 100_000,
    refill_per_second: 167,
};

pub const LINK_BNB_TESTNET: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x84b9B910527Ad5C03A9Ca831909E21e236EA7b06",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};

pub const LINK_BASE_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xd886e2286fd1073df82462ea1822119600af80b6",
    decimals: 18,
    capacity_usd: 100,
    refill_per_second: 1,
};



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