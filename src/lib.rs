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

// ref: https://docs.turnkey.com/api#tag/Wallets/operation/GetWalletAccounts

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/GetWallets

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/CreateWallet



//ref: https://docs.turnkey.com/api#tag/Wallets/operation/CreateWalletAccounts

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/ExportWallet

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/ExportWalletAccount

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/ImportWallet

//ref: https://docs.turnkey.com/api#tag/Wallets/operation/InitImportWallet
