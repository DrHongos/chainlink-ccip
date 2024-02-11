use crate::types::TokenCcip;

// mainnet - avalanche
pub const USDC_MAINNET_AVALANCHE: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// mainnet - arbitrum
pub const NUON_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "NUON",
    address: "0xCA160D11087E03fd398d40f561cd4768825f4958",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 55.55,
};

pub const SUUSD_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "suUSD",
    address: "0x8BF591Eae535f93a242D5A954d3Cde648b48A5A8",
    decimals: 18,
    rate_limit_capacity: 200_000,
    rate_limit_refill_rate: 2.314,
};

pub const SUETH_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "suETH",
    address: "0x1c22531AA9747d76fFF8F0A43b37954ca67d28e0",
    decimals: 18,
    rate_limit_capacity: 150,
    rate_limit_refill_rate: 0.0017,
};

pub const SUBTC_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "suBTC",
    address: "0xe85411C030fB32A9D8b14Bbbc6CB19417391F711",
    decimals: 18,
    rate_limit_capacity: 8,    
    rate_limit_refill_rate: 0.0000926,
};

pub const DFX_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "DFX",
    address: "0x888888435FDe8e7d4c54cAb67f206e4199454c60",
    decimals: 18,
    rate_limit_capacity: 5_000_000,
    rate_limit_refill_rate: 57.,
};

pub const HYPE_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "HYPE",
    address: "0x85225Ed797fd4128Ac45A992C46eA4681a7A15dA",
    decimals: 18,
    rate_limit_capacity: 1_000_000,
    rate_limit_refill_rate: 555.,   
};

pub const USDC_MAINNET_ARBITRUM: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// mainnet - base (uses arbitrums, except HYPE)
pub const LINK_MAINNET_BASE: TokenCcip = TokenCcip {
    name: "LINK",
    address: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
    decimals: 18,
    rate_limit_capacity: 50_000,
    rate_limit_refill_rate: 13.88,
};

// mainnet - optimism
pub const USDC_MAINNET_OPTIMISM: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// mainnet - polygon
pub const DFX_MAINNET_POLYGON: TokenCcip = TokenCcip {
    name: "DFX",
    address: "0x888888435FDe8e7d4c54cAb67f206e4199454c60",
    decimals: 18,
    rate_limit_capacity: 5_000_000,
    rate_limit_refill_rate: 57.,   
};

// optimism - usdc is repeated for mainnet & base
pub const USDC_OPTIMISM: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// arbitrum - mainnet || base
pub const USDC_ARBITRUM_MAINNET: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

pub const DFX_ARBITRUM_MAINNET: TokenCcip = TokenCcip {
    name: "DFX",
    address: "0x27f485b62C4A7E635F561A87560Adf5090239E93",
    decimals: 18,
    rate_limit_capacity: 5_000_000,
    rate_limit_refill_rate: 57.,   
};

pub const NUON_ARBITRUM_MAINNET: TokenCcip = TokenCcip {
    name: "NUON",
    address: "0xfb9Fed8cB962548A11fE7F6F282949061395c7F5",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 55.55,   
};

// polygon-avalanche
pub const THE_POLYGON_AVALANCHE: TokenCcip = TokenCcip {
    name: "THE",
    address: "0x27DfD2D7b85e0010542da35C6EBcD59E45fc949D",
    decimals: 18,
    rate_limit_capacity: 750_000,
    rate_limit_refill_rate: 417.,   
};

// avalanche - mainnet
pub const USDC_AVALANCHE_MAINNET: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// bnb - polygon
pub const THE_BNB_POLYGON: TokenCcip = TokenCcip {
    name: "THE",
    address: "0xF4C8E32EaDEC4BFe97E0F595AdD0f4450a863a11",
    decimals: 18,
    rate_limit_capacity: 750_000,
    rate_limit_refill_rate: 417.,   
};

// base - optimism 
pub const USDC_BASE_OPTIMISM: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,   // N/A
};

// base - mainnet
pub const LINK_BASE_MAINNET: TokenCcip = TokenCcip {
    name: "LINK",
    address: "0x88Fb150BDc53A65fe94Dea0c9BA0a6dAf8C6e196",
    decimals: 18,
    rate_limit_capacity: 50_000,
    rate_limit_refill_rate: 13.88,
};

// Tokens (v1.)
pub const SNXUSD_MAINNET_OPTIMISM: TokenCcip = TokenCcip {
    name: "snxUSD",
    address: "0xb2F30A7C980f052f02563fb518dcc39e6bf38175",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const LINK_SEPOLIA: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x779877A7B0D9E8603169DdbD7836e478b4624789",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const USDC_SEPOLIA: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,  // N/A
};

pub const CCIP_BNM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xFd57b4ddBf88a4e07fF4e34C487b99af2Fe82a05",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x466D489b6d36E7E3b824ef491C225F5830E81cC1",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const GHO_SEPOLIA: TokenCcip = TokenCcip {
    name: "GHO",
    address: "0xc4bF5CbDaBE595361438F8c6a187bDc330539c60",
    decimals: 18,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,  // N/A
};

pub const LINK_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xdc2CC710e42857672E7907CF474a69B63B93089f",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const CCIP_BNM_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xaBfE9D11A2f1D61990D1d253EC98B5Da00304F16",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0x835833d556299CdEC623e7980e7369145b037591",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const USDC_OPTIMISM_GOERLI: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xe05606174bac4A6364B31bd0eCA4bf4dD368f8C6",
    decimals: 6,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};

pub const USDC_OPTIMISM_SEPOLIA: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x5fd84259d66Cd46123540766Be93DFE6D43130D7",
    decimals: 6,
    rate_limit_capacity: 0,        // N/A
    rate_limit_refill_rate: 0.,  // N/A
};

pub const CCIP_BNM_OPTIMISM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0x8aF4204e30565DF93352fE8E1De78925F6664dA7",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_OPTIMISM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x044a6B4b561af69D2319A2f4be5Ec327a6975D0a",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const LINK_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x0b9d5D9136855f6FEc3c0993feE6E9CE8a297846",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const USDC_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x5425890298aed601595a70AB815c96711a31Bc65",
    decimals: 6,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};

pub const CCIP_BNM_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xD21341536c5cF5EB1bcb58f6723cE26e8D8E90e4",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_AVALANCHE_FUJI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0xD21341536c5cF5EB1bcb58f6723cE26e8D8E90e4",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const LINK_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xd14838A68E8AFBAdE5efb411d5871ea0011AFd28",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const CCIP_BNM_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0x0579b4c1C8AcbfF13c6253f1B10d66896Bf399Ef",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_ARBITRUM_GOERLI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0x0E14dBe2c8e1121902208be173A3fb91Bb125CDB",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const LINK_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x326C977E6efc84E512bB9C30f76E30c160eD06FB",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const USDC_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x9999f7Fea5938fD3b1E26A12c3f2fb024e194f97",
    decimals: 6,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};

pub const CCIP_BNM_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xf1E3A5842EeEF51F2967b3F05D45DD4f4205FF40",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_POLYGON_MUMBAI: TokenCcip = TokenCcip {
    name: "clCCIP-LnM",
    address: "0xc1c76a8c5bFDE1Be034bbcD930c668726E7C1987",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const LINK_BNB_TESTNET: TokenCcip = TokenCcip {
    name: "Link",
    address: "0x84b9B910527Ad5C03A9Ca831909E21e236EA7b06",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const LINK_BASE_GOERLI: TokenCcip = TokenCcip {
    name: "Link",
    address: "0xd886e2286fd1073df82462ea1822119600af80b6",
    decimals: 18,
    rate_limit_capacity: 100,
    rate_limit_refill_rate: 1.,
};

pub const CCIP_BNM_BNB_TESTNET: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xbFA2ACd33ED6EEc0ed3Cc06bF1ac38d22b36B9e9",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_BNB_TESNET: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x79a4Fc27f69323660f5Bfc12dEe21c3cC14f5901",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_BNM_BASE_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xbf9036529123DE264bFA0FC7362fE25B650D4B16",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_BASE_GOERLI: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x73ed16c1a61b098fd6924CCE5cC6a9A30348D944",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const USDC_BASE_GOERLI: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0xF175520C52418dfE19C8098071a252da48Cd1C19",
    decimals: 6,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};

pub const CCIP_BNM_ARBITRUM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-BnM",
    address: "0xA8C0c11bf64AF62CDCA6f93D3769B88BdD7cb93D",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const CCIP_LNM_ARBITRUM_SEPOLIA: TokenCcip = TokenCcip {
    name: "CCIP-LnM",
    address: "0x139E99f0ab4084E14e6bb7DacA289a91a2d92927",
    decimals: 18,
    rate_limit_capacity: 100_000,
    rate_limit_refill_rate: 167.,
};

pub const USDC_ARBITRUM_SEPOLIA: TokenCcip = TokenCcip {
    name: "USDC",
    address: "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d",
    decimals: 6,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};

pub const GHO_ARBITRUM_SEPOLIA: TokenCcip = TokenCcip {
    name: "GHO",
    address: "0xb13Cfa6f8B2Eed2C37fB00fF0c1A59807C585810",
    decimals: 18,
    rate_limit_capacity: 0,
    rate_limit_refill_rate: 0.,
};