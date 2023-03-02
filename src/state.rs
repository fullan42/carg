use cosmwasm_std::{Addr, Uint128, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    // X COİN adresi
    pub Xcoin: Addr,
    pub price: Coin,
    pub balanceOfX: Uint128,
    pub balance: Uint128,
}

// Burada State i, blockchain e kaydediyoruz. 
// Bu sekilde buradaki datalar blockchain de kalici olarak kaliyor.
pub const STATE: Item<State> = Item::new("state");