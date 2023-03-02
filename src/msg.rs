use cosmwasm_schema::{cw_serde, };
use cosmwasm_std::{Addr, Uint128, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub price: Coin,
    pub balance: Uint128,
    pub Xcoin: Addr,
    pub balanceOfX: Uint128,
    // Buradaki mesajiniz ile instantiate edilecek kontrat.
}

#[cw_serde]
pub enum ExecuteMsg {
    //get all juno balance  
    WithdrawAll {},
    //the price of the token
    SetPrice { denom: String, price: Uint128 },
    //sold amount
     Buy { denom: String, price: Uint128 },
     //Token balance of contract
    Balance {amount: Uint128 },
    // Burada blockchain e yazmak icin kullanacaginiz mesajlar olacak.
}

#[cw_serde]
pub enum QueryMsg {
   GetBalance { amount: Uint128  },
    GetPrice{ price: Uint128},
    GetOwner {address: Addr },

    // Burada blockchainden data almak ciin kullanacaginiz fonksiyonlar olacak.
}
