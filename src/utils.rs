use crate::{math, Erc20Token, Pool, PoolLiquidity, StorageChange, WHITELIST_TOKENS};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::ops::{Add, Mul};
use std::str;
use std::str::FromStr;
use substreams::{hex, log, Hex};

// const _DAI_USD_KEY: &str = "8ad599c3a0ff1de082011efddc58f1908eb6e6d8";
// const _USDC_ADDRESS: &str = "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
// const _USDC_WETH_03_POOL: &str = "8ad599c3a0ff1de082011efddc58f1908eb6e6d8";

pub const UNISWAP_V3_SMART_CONTRACT_BLOCK: u64 = 12369621;
pub const UNISWAP_V3_FACTORY: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");
pub const ZERO_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");

pub const _STABLE_COINS: [&str; 6] = [
    "6b175474e89094c44da98b954eedeac495271d0f",
    "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
    "dac17f958d2ee523a2206206994597c13d831ec7",
    "0000000000085d4780b73119b644ae5ecd22b376",
    "956f47f50a910163d8bf957cf5846d573e7f87ca",
    "4dd28568d05f09b02220b09c2cb307bfd837cb95",
];

// hard-coded tokens which have various behaviours but for which a UniswapV3 valid pool
// exists, some are tokens which were migrated to a new address, etc.
pub fn get_static_uniswap_tokens(token_address: &str) -> Option<Erc20Token> {
    return match token_address {
        "e0b7927c4af23765cb51314a0e0521a9645f0e2a" => Some(Erc20Token {
            // add DGD
            address: "e0b7927c4af23765cb51314a0e0521a9645f0e2a".to_string(),
            name: "DGD".to_string(),
            symbol: "DGD".to_string(),
            decimals: 9,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9" => Some(Erc20Token {
            // add AAVE
            address: "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9".to_string(),
            name: "Aave Token".to_string(),
            symbol: "AAVE".to_string(),
            decimals: 18,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        "eb9951021698b42e4399f9cbb6267aa35f82d59d" => Some(Erc20Token {
            // add LIF
            address: "eb9951021698b42e4399f9cbb6267aa35f82d59d".to_string(),
            name: "LIF".to_string(),
            symbol: "LIF".to_string(),
            decimals: 18,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        "bdeb4b83251fb146687fa19d1c660f99411eefe3" => Some(Erc20Token {
            // add SVD
            address: "bdeb4b83251fb146687fa19d1c660f99411eefe3".to_string(),
            name: "savedroid".to_string(),
            symbol: "SVD".to_string(),
            decimals: 18,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        "bb9bc244d798123fde783fcc1c72d3bb8c189413" => Some(Erc20Token {
            // add TheDAO
            address: "bb9bc244d798123fde783fcc1c72d3bb8c189413".to_string(),
            name: "TheDAO".to_string(),
            symbol: "TheDAO".to_string(),
            decimals: 16,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        "38c6a68304cdefb9bec48bbfaaba5c5b47818bb2" => Some(Erc20Token {
            // add HPB
            address: "38c6a68304cdefb9bec48bbfaaba5c5b47818bb2".to_string(),
            name: "HPBCoin".to_string(),
            symbol: "HPB".to_string(),
            decimals: 18,
            total_supply: "".to_string(), // subgraph doesn't check the total supply
            whitelist_pools: vec![],
        }),
        _ => None,
    };
}

pub fn convert_token_to_decimal(amount: &BigInt, decimals: u64) -> BigDecimal {
    let big_float_amount = BigDecimal::from_str(amount.to_string().as_str())
        .unwrap()
        .with_prec(100);

    return math::divide_by_decimals(big_float_amount, decimals);
}

pub fn should_handle_swap(pool: &Pool) -> bool {
    if pool.ignore_pool {
        return false;
    }
    return &pool.address != "9663f2ca0454accad3e094448ea6f77443880454";
}

pub fn should_handle_mint_and_burn(pool: &Pool) -> bool {
    if pool.ignore_pool {
        return false;
    }
    return true;
}

pub fn extract_pool_liquidity(
    log_ordinal: u64,
    pool_address: &Vec<u8>,
    storage_changes: &Vec<StorageChange>,
) -> Option<PoolLiquidity> {
    for sc in storage_changes {
        if pool_address.eq(&sc.address) {
            if sc.key[sc.key.len() - 1] == 4 {
                return Some(PoolLiquidity {
                    pool_address: Hex(&pool_address).to_string(),
                    liquidity: math::decimal_from_hex_be_bytes(&sc.new_value).to_string(),
                    log_ordinal,
                });
            }
        }
    }
    None
}

pub fn log_token(token: &Erc20Token, index: u64) {
    log::info!(
        "token {} addr: {}, name: {}, symbol: {}, decimals: {}",
        index,
        token.address,
        token.decimals,
        token.symbol,
        token.name
    );
}

pub fn calculate_amount_usd(
    amount0: &BigDecimal,
    amount1: &BigDecimal,
    token0_derived_eth_price: &BigDecimal,
    token1_derived_eth_price: &BigDecimal,
    bundle_eth_price: &BigDecimal,
) -> BigDecimal {
    return amount0
        .mul(token0_derived_eth_price.mul(bundle_eth_price.clone()))
        .add(amount1.mul(token1_derived_eth_price.mul(bundle_eth_price)));
}

pub fn decode_bytes_to_big_decimal(bytes: Vec<u8>) -> BigDecimal {
    let bytes_as_str = str::from_utf8(bytes.as_slice()).unwrap();
    return BigDecimal::from_str(bytes_as_str).unwrap().with_prec(100);
}

pub fn get_tracked_amount_usd(
    token0_id: &String,
    token1_id: &String,
    token0_derived_eth_price: &BigDecimal,
    token1_derived_eth_price: &BigDecimal,
    amount0_abs: &BigDecimal,
    amount1_abs: &BigDecimal,
    eth_price_in_usd: &BigDecimal,
) -> BigDecimal {
    let price0_usd = token0_derived_eth_price.mul(eth_price_in_usd.clone());
    let price1_usd = token1_derived_eth_price.mul(eth_price_in_usd);

    // both are whitelist tokens, return sum of both amounts
    if WHITELIST_TOKENS.contains(&token0_id.as_str())
        && WHITELIST_TOKENS.contains(&token0_id.as_str())
    {
        return amount0_abs.mul(price0_usd).add(amount1_abs.mul(price1_usd));
    }

    // take double value of the whitelisted token amount
    if WHITELIST_TOKENS.contains(&token0_id.as_str())
        && !WHITELIST_TOKENS.contains(&token1_id.as_str())
    {
        return amount0_abs.mul(price0_usd).mul(BigDecimal::from(2 as i32));
    }

    // take double value of the whitelisted token amount
    if !WHITELIST_TOKENS.contains(&token0_id.as_str())
        && WHITELIST_TOKENS.contains(&token1_id.as_str())
    {
        return amount1_abs.mul(price1_usd).mul(BigDecimal::from(2 as i32));
    }

    // neither token is on white list, tracked amount is 0
    return BigDecimal::from(0 as i32);
}
