//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use alloy_primitives::U256;
use dotenv::dotenv;
use ethers::{
    abi::{decode, AbiEncode, ParamType},
    contract::ContractError,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes},
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string;
use std::sync::Arc;
use stylus_sdk::call::Error;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // calls and set up environment variables
    dotenv::dotenv().ok();

    let priv_key_path =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let program_address = std::env::var(STYLUS_PROGRAM_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_PROGRAM_ADDRESS))?;

    abigen!(
        Bitsave,
        r#"[
            function getBitsaveUserCount() external view returns (uint256)
            function getUserDetails(uint8[] memory username) external view returns (string memory, uint256, address)
            function getBitsaveBalance() external view returns (uint256)
            function getAccumulatedPool() external view returns (uint256)
            function getTokensBalance() external view returns (uint256)

            function joinBitsave(uint8[] calldata user_name) external payable returns (address)
            function fund() external payable returns (uint256)
            function createSaving(string calldata name_of_saving, uint256 maturity_time, uint8 penalty_perc, bool use_safe_mode) external
            function incrementSaving(string calldata name_of_saving) external
            function withdrawSavings(string calldata name_of_saving) external returns (uint256)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let bitsave = Bitsave::new(address, client);

    // TXs
    let fund_bitsave_res = bitsave.fund().value(2).send().await?.await?;
    println!("{:?}", fund_bitsave_res);

    let bitsave_balance = bitsave.get_bitsave_balance().call().await;
    println!("Balance: {:?}", bitsave_balance);

    // let join_res = bitsave
    //     .join_bitsave("me".as_bytes().to_vec())
    //     .send()
    //     .await?
    //     .await?;
    // println!("Join bitsave return value = {:?}", join_res);

    println!("{:?}", "xpan".as_bytes().to_vec());
    let user_data = bitsave
        .get_user_details("xpan".as_bytes().to_vec())
        .call()
        .await;
    println!("{:?}", user_data);
    if let Ok((username, user_id, user_address)) = user_data {
        println!(
            "User details\nName: {:?}\nId: {user_id}\nAddress: {user_address}",
            String::from_utf8(username.into()).unwrap()
        );
    } else {
        println!("Err decoding {:?}", user_data);
    }

    // if let Err(ContractErr   or::Revert(Bytes(join_val))) = join_res {
    //     println!("{:?}", String::from_utf8(join_val.encode()));
    // };

    let count_res = bitsave.get_bitsave_user_count().call().await;
    println!("Bitsave user count = {:?}", count_res);

    // let create_res = bitsave
    //     .create_saving("schoolFee".to_string(), 1714242866.into(), 2, false)
    //     .call()
    //     .await;
    // println!("Create saving bitsave return value = {:?}", create_res);
    //
    // if let Err(err_vec) = create_res {
    //     println!("{:#?}", err_vec);
    // }
    // if let Err(ContractError::Revert(Bytes(err_vec))) = create_res {
    //     let err = err_vec.to_vec();
    //     println!("{:?}", String::from_utf8(err));
    // }

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
