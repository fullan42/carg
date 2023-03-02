#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{ Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Coin, Addr,  CosmosMsg,  BankMsg, to_binary, Binary,  };
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, self};
use crate::state::{State, STATE};

use self::query::get_balance;

// Burada versiyonlarimizi yaratiyor ki ileride migrate edersek kontrati bu bilgileri kullanabilelim
const CONTRACT_NAME: &str = "Case Study";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Once state i yaratiyoruz
    let state = State {
        owner: info.sender.clone(),
        Xcoin: msg.Xcoin,
        price: msg.price,
        balanceOfX: msg.balanceOfX,
        balance: msg.balance,
    };

    // Kontrat versiyonumuzu kaydediyoruz.
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Yarattigimiz state i blockchain e kalici olarak kaydediyoruz.
    STATE.save(deps.storage, &state)?;

    // Fonksiyonumuz donus yapiyor burada, return degerleri olarak methodun ne oldugunu ve instantiate eden kisinin kim oldugunu donduruyoruz.
    // Bu degerler ozellikle front end kisminda onemli olacak.
    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPrice { denom, price } => try_set_price(
            deps,
            info.sender,
            Coin {
                denom,
                amount: price,
            },
        ),
        ExecuteMsg::Balance { amount } => try_balance(deps, info.sender, amount),
        ExecuteMsg::Buy { denom,  price } => try_buy(deps, info, denom, price),
        ExecuteMsg::WithdrawAll {} => try_withdraw_all(deps, info.sender),
    }
}
//execute methodları
pub fn try_set_price(deps: DepsMut, sender: Addr, price: Coin) -> Result<Response, ContractError> {
    if STATE.load(deps.storage)?.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.price = price;
        Ok(state)
    })?;

    Ok(Response::default())
}
pub fn try_balance(deps: DepsMut, sender: Addr, amount: Uint128) -> Result<Response, ContractError> {
    if STATE.load(deps.storage)?.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.balanceOfX = amount;
        Ok(state)
    })?;

    Ok(Response::default())
}
pub fn try_buy(deps: DepsMut, info: MessageInfo, denom: String, price: Uint128) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if state.price.denom != denom {
        return Err(ContractError::Unauthorized {});
    }
    if state.price.amount > price {
        return Err(ContractError::Unauthorized {});
    }
    if state.balanceOfX < price {
        return Err(ContractError::Unauthorized {});
    }
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: state.Xcoin.to_string(),
        amount: vec![Coin {
            denom: state.price.denom.to_string(),
            amount: price,
        }],
    });
    let response = Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "buy")
        .add_attribute("from", info.sender.to_string());
    Ok(response)
}
pub fn try_withdraw_all(deps: DepsMut, sender: Addr) -> Result<Response, ContractError> {
    // Load the state from storage.
    let mut state = STATE.load(deps.storage)?;

    // Get the balance of the contract's native tokens.
    let balance = state.balanceOfX;

    // Update the state to reflect the withdrawn tokens.
    state.balanceOfX = Uint128::zero();
    STATE.save(deps.storage, &state)?;

    // Transfer the native tokens to the sender's account.
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: sender.to_string(),
        amount: vec![Coin {
            denom: state.price.denom.to_string(),
            amount: balance,
        }],
    });

    // Create a response with the transfer message and an empty `SubMsg` array.
    let response = Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "withdraw_all")
        .add_attribute("from", state.Xcoin.to_string());

    Ok(response)
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { price } => {
            let result = query::get_price(deps)?;
            Ok(to_binary(&result)?)
        },
        QueryMsg::GetBalance { amount } => {
            let result = query::get_balance(deps)?;
            Ok(to_binary(&result)?)
        },
        QueryMsg::GetOwner { address: _ } => {
            let result = query::get_owner(deps)?;
            Ok(to_binary(&result)?)
        },
    }
}



pub mod query {
    use super::*;

    pub fn get_price(deps: Deps) -> StdResult<Uint128> {
        let state = STATE.load(deps.storage)?;
        Ok(state.price.amount)
    }

    pub fn get_balance(deps: Deps) -> StdResult<Uint128> {
        let state = STATE.load(deps.storage)?;
        Ok(state.balanceOfX)
    }

    pub fn get_owner(deps: Deps) -> StdResult<Addr> {
        let state = STATE.load(deps.storage)?;
        Ok(state.owner)
    }
}

#[cfg(test)]
mod tests {



    use super::*;
    #[test]
    fn test_instantiate_msg() {
        // Create a sample InstantiateMsg instance
        let owner = Addr::unchecked("owner");
        let price = Coin::new(100, "ABC");
        let balance = Uint128::new(1000);
        let xcoin = Addr::unchecked("xcoin");
        let balance_of_x = Uint128::new(500);
        let msg = InstantiateMsg {
            owner: owner.clone(),
            price: price.clone(),
            balance: balance.clone(),
            Xcoin: xcoin.clone(),
            balanceOfX: balance_of_x.clone(),
        };

        // Test that the values are properly set
        assert_eq!(msg.owner, owner);
        assert_eq!(msg.price, price);
        assert_eq!(msg.balance, balance);
        assert_eq!(msg.Xcoin, xcoin);
        assert_eq!(msg.balanceOfX, balance_of_x);
    }
}
    