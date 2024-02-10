use crate::types::*;
use std::collections::HashMap;
use alloy_primitives::Address;

lazy_static! {

    // Networks
    pub static ref MAINNET_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap());
        h.insert(FeeToken::Link, "0x514910771AF9Ca656af840dff83E8264EcF986CA".parse().unwrap());
        h
    };
    
    pub static ref OPTIMISM_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0x350a791Bfc2C21F9Ed5d10980Dad2e2638ffa7f6".parse().unwrap());
        h
    };
    
    pub static ref AVALANCHE_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7".parse().unwrap());
        h.insert(FeeToken::Link, "0x5947BB275c521040051D82396192181b413227A3".parse().unwrap());
        h
    };
    
    pub static ref POLYGON_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270".parse().unwrap());
        h.insert(FeeToken::Link, "0xb0897686c545045aFc77CF20eC7A532E3120E0F1".parse().unwrap());
        h
    };
    
    pub static ref ARBITRUM_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".parse().unwrap());
        h.insert(FeeToken::Link, "0xf97f4df75117a78c1A5a0DBb814Af92458539FB4".parse().unwrap());
        h
    };

    pub static ref BNB_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c".parse().unwrap());
        h.insert(FeeToken::Link, "0x404460C6A5EdE2D891e8297795264fDe62ADBB75".parse().unwrap());
        h
    };

    pub static ref BASE_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0x88Fb150BDc53A65fe94Dea0c9BA0a6dAf8C6e196".parse().unwrap());
        h
    };
    
    // Testnets
    pub static ref SEPOLIA_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x097D90c9d3E0B50Ca60e1ae45F6A81010f9FB534".parse().unwrap());
        h.insert(FeeToken::Link, "0x779877A7B0D9E8603169DdbD7836e478b4624789".parse().unwrap());
        h
    };

    pub static ref OPTIMISM_GOERLI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0xdc2CC710e42857672E7907CF474a69B63B93089f".parse().unwrap());
        h
    };

    pub static ref OPTIMISM_SEPOLIA_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0x4200000000000000000000000000000000000006".parse().unwrap());
        h.insert(FeeToken::Link, "0xE4aB69C077896252FAFBD49EFD26B5D171A32410".parse().unwrap());
        h
    };

    pub static ref AVALANCHE_FUJI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xd00ae08403B9bbb9124bB305C09058E32C39A48c".parse().unwrap());
        h.insert(FeeToken::Link, "0x0b9d5D9136855f6FEc3c0993feE6E9CE8a297846".parse().unwrap());
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
        h.insert(FeeToken::Link, "0xD886E2286Fd1073df82462ea1822119600Af80b6".parse().unwrap());
        h
    };

    pub static ref ARBITRUM_GOERLI_FEE_TOKENS: HashMap<FeeToken, Address> = {
        let mut h = HashMap::new();
        h.insert(FeeToken::Native, Address::ZERO);
        h.insert(FeeToken::WrappedNative, "0xE591bf0A0CF924A0674d7792db046B23CEbF5f34".parse().unwrap());
        h.insert(FeeToken::Link, "0xb1D4538B4571d411F07960EF2838Ce337FE1E80E".parse().unwrap());
        h
    };
}