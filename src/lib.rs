//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use errors::{BResult, BitsaveErrors, UserNotExist};
use stylus_sdk::{
    alloy_primitives::{U256, U8},
    call::{call, Call},
    msg,
    prelude::*,
};
use user_data::UserData;

mod errors;
mod user_data;

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    // #[entrypoint]
    pub struct Counter {
        uint256 number;
    }

    #[entrypoint]
    pub struct Bitsave {
        uint256 user_count;
        mapping(address => UserData) users_mapping;
    }
}

/// impl for borrow and borrowMut

/// Declare that `Counter` is a contract with the following external methods.
// #[external]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }
}

#[external]
impl Bitsave {
    /// Helpers
    // get user
    // fn get_user(&self) -> User_data {
    //     self.users_mapping.get(msg::sender());
    //     todo!()
    // }

    /// Joining bitsave: Initiates
    /// 1. user mapping (address -> savingsMap)
    /// 2. user savings names
    pub fn join_bitsave(&mut self) -> Result<bool, Vec<u8>> {
        // check user doesn't exist
        let fetched_user = self.users_mapping.get(msg::sender());
        if fetched_user.user_exists.get() {
            return Err(
                format!("User {:?} has joined bitsave already", fetched_user.user_id).into(),
            );
        };

        // check for joining fee todo
        // if ()

        // incr user count
        let new_user_count = self.user_count.get() + U256::from(1);
        self.user_count.set(new_user_count);

        let mut fetched_user = self.users_mapping.setter(msg::sender());
        // update user data
        fetched_user.user_exists.set(true);
        fetched_user.user_id.set(new_user_count);
        fetched_user.user_address.set(msg::sender());

        // return user exists txn
        Ok(fetched_user.user_exists.get())
    }

    /// Create savings:
    pub fn create_saving(
        &mut self,
        name_of_saving: String,
        maturity_time: U256,
        penalty_perc: u8,
        use_safe_mode: bool,
    ) -> Result<(), Vec<u8>> {
        // retrieve some data
        // fetch user's data
        let fetched_user = self.users_mapping.get(msg::sender());
        if !fetched_user.user_exists.get() {
            println!("User not found");
            return Err(BitsaveErrors::UserNotExist(UserNotExist {}).into());
        }

        let amount_of_saving = msg::value();
        let token_id = Address::ZERO; // todo: fix in token address

        // user setter
        let mut user_updater = self.users_mapping.setter(msg::sender());
        let res = user_updater.create_saving_data(
            name_of_saving,
            amount_of_saving,
            token_id,
            maturity_time,
            penalty_perc,
            use_safe_mode,
        );

        if let Err(res_err) = res {
            return Err(res_err.into());
        }

        Ok(())
    }

    /// Increment savings
    pub fn increment_saving(&mut self, name_of_saving: String) -> Result<(), Vec<u8>> {
        // retrieve some data
        // fetch user's data
        let fetched_user = self.users_mapping.get(msg::sender());
        if !fetched_user.user_exists.get() {
            return Err("User doesn't exist".into());
        }

        let amount_to_add = msg::value();
        let token_id = Address::ZERO; // todo: fix in token address

        // user setter
        let mut user_updater = self.users_mapping.setter(msg::sender());
        user_updater.increment_saving_data(name_of_saving, amount_to_add, token_id)?;
        Ok(())
    }

    /// Withdraw savings
    pub fn withdraw_savings(&mut self, name_of_saving: String) -> Result<U256, Vec<u8>> {
        let fetched_user = self.users_mapping.get(msg::sender());
        if !fetched_user.user_exists.get() {
            return Err("User doesn't exist".into());
        }

        // user updater
        let mut user_updater = self.users_mapping.setter(msg::sender());
        let with_amount = user_updater.withdraw_saving_data(name_of_saving)?;

        // transfer funds
        call(Call::new_in(self).value(with_amount), msg::sender(), &[])?;

        Ok(with_amount)
    }
}
