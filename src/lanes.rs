use crate::types::*;
use alloy_chains::NamedChain;
use crate::tokens::*;

// Lanes
lazy_static!{

    // Mainnet
    pub static ref MAINNET_AVALANCHE_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Mainnet,
            destination: NamedChain::Avalanche,
            supported_tokens: Some(vec![USDC_MAINNET_AVALANCHE]),
        } 
    };

    pub static ref MAINNET_BNB_LANE: Lane = {
        Lane {
            capacity: 0,
            refill_rate: 0.,
            source: NamedChain::Mainnet,
            destination: NamedChain::BinanceSmartChain,
            supported_tokens: None,
        } 
    };

    pub static ref MAINNET_ARBITRUM_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Mainnet,
            destination: NamedChain::Arbitrum,
            supported_tokens: Some(vec![
                NUON_MAINNET_ARBITRUM, SUUSD_MAINNET_ARBITRUM, SUETH_MAINNET_ARBITRUM,
                SUBTC_MAINNET_ARBITRUM, DFX_MAINNET_ARBITRUM, HYPE_MAINNET_ARBITRUM, USDC_MAINNET_ARBITRUM
            ]),
        } 
    };

    pub static ref MAINNET_BASE_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Mainnet,
            destination: NamedChain::Base,
            supported_tokens: Some(vec![
                NUON_MAINNET_ARBITRUM, SUUSD_MAINNET_ARBITRUM, SUETH_MAINNET_ARBITRUM,
                SUBTC_MAINNET_ARBITRUM, LINK_MAINNET_BASE, USDC_MAINNET_ARBITRUM
            ]),
        } 
    };
    
    pub static ref MAINNET_OPTIMISM_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Mainnet,
            destination: NamedChain::Optimism,
            supported_tokens: Some(vec![USDC_MAINNET_OPTIMISM]),
        } 
    };
    
    pub static ref MAINNET_POLYGON_LANE: Lane = {
        Lane {
            capacity: 600_000,
            refill_rate: 167.,
            source: NamedChain::Mainnet,
            destination: NamedChain::Polygon,
            supported_tokens: Some(vec![DFX_MAINNET_POLYGON]),
        } 
    };
    // Optimism
    pub static ref OPTIMISM_MAINNET_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Optimism,
            destination: NamedChain::Mainnet,
            supported_tokens: Some(vec![USDC_OPTIMISM]),
        } 
    };
    
    pub static ref OPTIMISM_BASE_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Optimism,
            destination: NamedChain::Base,
            supported_tokens: Some(vec![USDC_OPTIMISM]),
        } 
    };
    
    pub static ref OPTIMISM_POLYGON_LANE: Lane = {
        Lane {
            capacity: 0,
            refill_rate: 0.,
            source: NamedChain::Optimism,
            destination: NamedChain::Polygon,
            supported_tokens: None,
        } 
    };
    
    // Arbitrum
    pub static ref ARBITRUM_MAINNET_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Optimism,
            destination: NamedChain::Polygon,
            supported_tokens: Some(vec![
                NUON_ARBITRUM_MAINNET, 
                SUUSD_MAINNET_ARBITRUM, 
                SUETH_MAINNET_ARBITRUM,
                SUBTC_MAINNET_ARBITRUM, 
                DFX_ARBITRUM_MAINNET, 
                HYPE_MAINNET_ARBITRUM, 
                USDC_ARBITRUM_MAINNET
            ]),
        } 
    };

    pub static ref ARBITRUM_BASE_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Arbitrum,
            destination: NamedChain::Base,
            supported_tokens: Some(vec![
                NUON_ARBITRUM_MAINNET,
                SUUSD_MAINNET_ARBITRUM, 
                SUETH_MAINNET_ARBITRUM,
                SUBTC_MAINNET_ARBITRUM, 
                USDC_ARBITRUM_MAINNET
            ]),
        } 
    };

    pub static ref POLYGON_MAINNET_LANE: Lane = Lane {
        capacity: 600_000,
        refill_rate: 167.,
        source: NamedChain::Polygon,
        destination: NamedChain::Mainnet,
        supported_tokens: Some(vec![DFX_ARBITRUM_MAINNET]), 
    };

    pub static ref POLYGON_AVALANCHE_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::Polygon,
        destination: NamedChain::Avalanche,
        supported_tokens: Some(vec![THE_POLYGON_AVALANCHE]), 
    };

    pub static ref POLYGON_BNB_LANE: Lane = Lane {
        capacity: 100_000,
        refill_rate: 167.,
        source: NamedChain::Polygon,
        destination: NamedChain::BinanceSmartChain,
        supported_tokens: Some(vec![THE_POLYGON_AVALANCHE]), 
    };

    pub static ref POLYGON_OPTIMISM_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::Polygon,
        destination: NamedChain::Optimism,
        supported_tokens: None, 
    };

    pub static ref AVALANCHE_MAINNET_LANE: Lane = Lane {
        capacity: 1_000_000,
        refill_rate: 277.,
        source: NamedChain::Avalanche,
        destination: NamedChain::Mainnet,
        supported_tokens: Some(vec![THE_POLYGON_AVALANCHE]), 
    };

    pub static ref AVALANCHE_BNB_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::Avalanche,
        destination: NamedChain::BinanceSmartChain,
        supported_tokens: None, 
    };

    pub static ref AVALANCHE_POLYGON_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::Avalanche,
        destination: NamedChain::Polygon,
        supported_tokens: None, 
    };

    pub static ref BNB_AVALANCHE_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::BinanceSmartChain,
        destination: NamedChain::Avalanche,
        supported_tokens: None, 
    };

    pub static ref BNB_BASE_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::BinanceSmartChain,
        destination: NamedChain::Base,
        supported_tokens: None, 
    };

    pub static ref BNB_MAINNET_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::BinanceSmartChain,
        destination: NamedChain::Mainnet,
        supported_tokens: None, 
    };

    pub static ref BNB_POLYGON_LANE: Lane = Lane {
        capacity: 100_000,
        refill_rate: 167.,
        source: NamedChain::BinanceSmartChain,
        destination: NamedChain::Polygon,
        supported_tokens: Some(vec![THE_POLYGON_AVALANCHE]), 
    };

    pub static ref BASE_BNB_LANE: Lane = Lane {
        capacity: 0,
        refill_rate: 0.,
        source: NamedChain::Base,
        destination: NamedChain::BinanceSmartChain,
        supported_tokens: None,
    };
    
    pub static ref BASE_OPTIMISM_LANE: Lane = Lane {
        capacity: 1_000_000,
        refill_rate: 277.,
        source: NamedChain::Base,
        destination: NamedChain::Optimism,
        supported_tokens: Some(vec![USDC_BASE_OPTIMISM]), 
    };
 
    pub static ref BASE_ARBITRUM_LANE: Lane = Lane {
        capacity: 1_000_000,
        refill_rate: 277.,
        source: NamedChain::Base,
        destination: NamedChain::Arbitrum,
        supported_tokens: Some(vec![
            NUON_MAINNET_ARBITRUM,
            SUUSD_MAINNET_ARBITRUM,
            SUETH_MAINNET_ARBITRUM,
            SUBTC_MAINNET_ARBITRUM,
            USDC_BASE_OPTIMISM
        ]), 
    };

    pub static ref BASE_MAINNET_LANE: Lane = Lane {
        capacity: 1_000_000,
        refill_rate: 277.,
        source: NamedChain::Base,
        destination: NamedChain::Mainnet,
        supported_tokens: Some(vec![
            NUON_MAINNET_ARBITRUM,
            SUUSD_MAINNET_ARBITRUM,
            SUETH_MAINNET_ARBITRUM,
            SUBTC_MAINNET_ARBITRUM,
            USDC_OPTIMISM,
            LINK_BASE_MAINNET
        ]), 
    };

// tesnets

    pub static ref SEPOLIA_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::Sepolia,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(vec![CCIP_BNM_SEPOLIA, CCIP_LNM_SEPOLIA])
        }
    };

    pub static ref SEPOLIA_OPTIMISM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Sepolia,
            destination: NamedChain::OptimismSepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
                USDC_SEPOLIA
            ])
        }
    };

    pub static ref SEPOLIA_ARBITRUM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,    // its more!
            refill_rate: 277.,      // this too
            source: NamedChain::Sepolia,
            destination: NamedChain::ArbitrumSepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
                USDC_SEPOLIA,
                GHO_SEPOLIA
            ])
        }
    };

    pub static ref SEPOLIA_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Sepolia,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
                USDC_SEPOLIA,
            ])
        }
    };

    pub static ref SEPOLIA_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::Sepolia,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
                USDC_SEPOLIA,
            ])
        }
    };

    pub static ref SEPOLIA_BNB_TESNET_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::Sepolia,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
            ])
        }
    };

    pub static ref SEPOLIA_BASE_GOERLI_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::Sepolia,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![
                CCIP_BNM_SEPOLIA, 
                CCIP_LNM_SEPOLIA, 
            ])
        }
    };
// optimism goerli
    pub static ref OPTIMISM_GOERLI_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_GOERLI,
                CCIP_LNM_OPTIMISM_GOERLI
            ]),
        } 
    };

    pub static ref OPTIMISM_GOERLI_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_GOERLI,
                CCIP_LNM_OPTIMISM_GOERLI,
                USDC_OPTIMISM_GOERLI
            ]),
        } 
    };

    pub static ref OPTIMISM_GOERLI_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_GOERLI,
                CCIP_LNM_OPTIMISM_GOERLI,
                USDC_OPTIMISM_GOERLI
            ]),
        } 
    };

    pub static ref OPTIMISM_GOERLI_BASE_GOERLI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismGoerli,
            destination: NamedChain::BaseGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_GOERLI,
                CCIP_LNM_OPTIMISM_GOERLI,
                USDC_OPTIMISM_GOERLI
            ]),
        } 
    };
// optimism -sepolia
    pub static ref OPTIMISM_SEPOLIA_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismSepolia,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_SEPOLIA,
                CCIP_LNM_OPTIMISM_SEPOLIA,
                USDC_OPTIMISM_SEPOLIA
            ]),
        } 
    };

    pub static ref OPTIMISM_SEPOLIA_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismSepolia,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_SEPOLIA,
                CCIP_LNM_OPTIMISM_SEPOLIA,
                USDC_OPTIMISM_SEPOLIA
            ]),
        } 
    };

    pub static ref OPTIMISM_SEPOLIA_ARBITRUM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismSepolia,
            destination: NamedChain::ArbitrumSepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_SEPOLIA,
                CCIP_LNM_OPTIMISM_SEPOLIA,
                USDC_OPTIMISM_SEPOLIA
                ]),
            } 
    };
    
    pub static ref OPTIMISM_SEPOLIA_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::OptimismSepolia,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_OPTIMISM_SEPOLIA,
                CCIP_LNM_OPTIMISM_SEPOLIA,
                USDC_OPTIMISM_SEPOLIA
                ]),
            } 
    };
                       
    pub static ref POLYGON_MUMBAI_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_POLYGON_MUMBAI,
                CCIP_LNM_POLYGON_MUMBAI,
                USDC_POLYGON_MUMBAI
            ]),
        } 
    };
    pub static ref POLYGON_MUMBAI_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_POLYGON_MUMBAI,
                CCIP_LNM_POLYGON_MUMBAI,
                USDC_POLYGON_MUMBAI
            ]),
        } 
    };
    pub static ref POLYGON_MUMBAI_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_POLYGON_MUMBAI,
                CCIP_LNM_POLYGON_MUMBAI,
                USDC_POLYGON_MUMBAI
            ]),
        } 
    };
    pub static ref POLYGON_MUMBAI_OPTIMISM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::OptimismSepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_POLYGON_MUMBAI,
                CCIP_LNM_POLYGON_MUMBAI,
                USDC_POLYGON_MUMBAI
            ]),
        } 
    };
    pub static ref POLYGON_MUMBAI_BNB_TESTNET_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::PolygonMumbai,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![
                CCIP_BNM_POLYGON_MUMBAI,
                CCIP_LNM_POLYGON_MUMBAI,
            ]),
        } 
    };

    pub static ref AVALANCHE_FUJI_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
                USDC_AVALANCHE_FUJI                
            ]),
        } 
    };
    
    pub static ref AVALANCHE_FUJI_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(vec![

                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
                USDC_AVALANCHE_FUJI                
            ]),
        } 
    };
    
    pub static ref AVALANCHE_FUJI_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(vec![
                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
                USDC_AVALANCHE_FUJI                
            ]),
        } 
    };

    pub static ref AVALANCHE_FUJI_BNB_TESNET_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![
                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
            ]),
        } 
    };
    
    pub static ref AVALANCHE_FUJI_OPTIMISM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::OptimismSepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
                USDC_AVALANCHE_FUJI                
            ]),
        } 
    };
    
    pub static ref AVALANCHE_FUJI_BASE_GOERLI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::AvalancheFuji,
            destination: NamedChain::BaseGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_AVALANCHE_FUJI,
                CCIP_LNM_AVALANCHE_FUJI,
                USDC_AVALANCHE_FUJI                
            ]),
        } 
    };

    pub static ref BNB_TESTNET_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BinanceSmartChainTestnet,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_BNB_TESTNET,
                CCIP_LNM_BNB_TESNET,
            ]),
        } 
    };
    
    pub static ref BNB_TESTNET_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BinanceSmartChainTestnet,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_BNB_TESTNET,
                CCIP_LNM_BNB_TESNET,
            ]),
        } 
    };
    pub static ref BNB_TESTNET_POLYGON_MUMBAI_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BinanceSmartChainTestnet,
            destination: NamedChain::PolygonMumbai,
            supported_tokens: Some(vec![
                CCIP_BNM_BNB_TESTNET,
                CCIP_LNM_BNB_TESNET,
            ]),
        } 
    };
    pub static ref BNB_TESTNET_BASE_GOERLI_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BinanceSmartChainTestnet,
            destination: NamedChain::BaseGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_BNB_TESTNET,
                CCIP_LNM_BNB_TESNET,
            ]),
        } 
    };

    pub static ref BASE_GOERLI_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BaseGoerli,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_BASE_GOERLI,
                CCIP_LNM_BASE_GOERLI,
            ]),
        } 
    };
    pub static ref BASE_GOERLI_OPTIMISM_GOERLI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::BaseGoerli,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_BASE_GOERLI,
                CCIP_LNM_BASE_GOERLI,
                USDC_BASE_GOERLI
            ]),
        } 
    };
    pub static ref BASE_GOERLI_BNB_TESNET_LANE: Lane = {
        Lane {
            capacity: 100_000,
            refill_rate: 167.,
            source: NamedChain::BaseGoerli,
            destination: NamedChain::BinanceSmartChainTestnet,
            supported_tokens: Some(vec![
                CCIP_BNM_BASE_GOERLI,
                CCIP_LNM_BASE_GOERLI
            ]),
        } 
    };
    pub static ref BASE_GOERLI_AVALANCHE_FUJI_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::BaseGoerli,
            destination: NamedChain::AvalancheFuji,
            supported_tokens: Some(vec![
                CCIP_BNM_BASE_GOERLI,
                CCIP_LNM_BASE_GOERLI,
                USDC_BASE_GOERLI
            ]),
        } 
    };

    pub static ref ARBITRUM_SEPOLIA_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,        // its more!
            refill_rate: 277.,          // its more!
            source: NamedChain::ArbitrumSepolia,
            destination: NamedChain::Sepolia,
            supported_tokens: Some(vec![
                CCIP_BNM_ARBITRUM_SEPOLIA,
                CCIP_LNM_ARBITRUM_SEPOLIA,
                USDC_ARBITRUM_SEPOLIA,
                GHO_ARBITRUM_SEPOLIA,
            ]),
        } 
    };

    pub static ref ARBITRUM_SEPOLIA_OPTIMISM_SEPOLIA_LANE: Lane = {
        Lane {
            capacity: 1_000_000,
            refill_rate: 277.,
            source: NamedChain::ArbitrumSepolia,
            destination: NamedChain::OptimismGoerli,
            supported_tokens: Some(vec![
                CCIP_BNM_ARBITRUM_SEPOLIA,
                CCIP_LNM_ARBITRUM_SEPOLIA,
                USDC_ARBITRUM_SEPOLIA,
            ]),
        } 
    };


}