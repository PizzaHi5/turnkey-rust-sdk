use turnkey::client::Turnkey;

use turnkey::errors::*;
use dotenv::dotenv;
use std::{env, str::FromStr};
use ethers::prelude::*;

//import ethers

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/GetWallet
pub async fn get_wallet_v1(turnkey:&Turnkey, wallet_id: String) -> TurnkeyResult<()> {
    dotenv().ok();
    let turnkey_client = Turnkey::new();
    
    
    // {
    //     "organizationId": "string",
    //     "walletId": "string"
    //   }
    Ok(())
}