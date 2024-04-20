use alloy_primitives::{Address, U256, U8};
use stylus_sdk::{block, contract::address, stylus_proc::sol_storage};

use crate::errors::{BitsaveErrors, GeneralError};

sol_storage! {
    pub struct UserData {
        bool user_exists;
        address user_address;
        uint256 user_id;
        string user_name;
        uint8 savings_count;
        mapping(string => SavingData) savings_map;
        string[] savings_names;
    }

    pub struct SavingData {
        bool is_valid;
        uint256 amount;
        uint256 maturity_time;
        uint256 start_time;
        address token_id;
        bool is_safe_mode;
        uint256 interest_accumulated;
        uint8 penalty_perc;
    }
}

type BResult<T, E = BitsaveErrors> = core::result::Result<T, E>;

impl UserData {
    pub fn get_user_id(&self) -> U256 {
        self.user_id.get()
    }

    fn calculate_new_interest(&self, amount: U256) -> U256 {
        amount * U256::from(1) / U256::from(100)
    }

    pub fn create_user(&mut self, address: Address, user_id: U256, user_name: String) -> bool {
        self.user_address.set(address);
        self.user_exists.set(true);
        self.user_id.set(user_id);
        self.user_name.set_str(user_name);
        self.user_exists.get()
    }

    fn calculate_balance_from_penalty(amount: U256, penalty_perc: U8) -> U256 {
        let perc_value = amount * U256::from(penalty_perc) / U256::from(100);
        amount - perc_value
    }

    pub fn create_saving_data(
        &mut self,
        name_of_saving: String,
        amount_of_saving: U256,
        token_id: Address,
        maturity_time: U256,
        penalty_perc: u8,
        use_safe_mode: bool,
    ) -> BResult<()> {
        let fetched_saving = self.savings_map.get(name_of_saving.clone());

        // error if saving exists
        if fetched_saving.is_valid.get() {
            return Err(BitsaveErrors::GeneralError(GeneralError {}));
        };

        // initiate saving object
        // let new_saving = SavingData {
        //     is_valid: true.into(),
        //     token_id,
        //     amount: amount_of_saving.into(),
        //     start_time: block::timestamp(),
        //     maturity_time,
        //     interest_accumulated: U256::from(0),
        //     is_safe_mode,
        // };

        let mut new_saving = self.savings_map.setter(name_of_saving);
        // update saving data
        new_saving.is_safe_mode.set(use_safe_mode);
        new_saving.is_valid.set(true);
        new_saving.token_id.set(token_id);
        new_saving.maturity_time.set(maturity_time);
        new_saving.start_time.set(U256::from(block::timestamp()));
        new_saving.interest_accumulated.set(U256::from(0));
        new_saving.amount.set(amount_of_saving);
        new_saving.penalty_perc.set(U8::from(penalty_perc));

        Ok(())
    }

    pub fn increment_saving_data(
        &mut self,
        name_of_saving: String,
        new_amount: U256,
        token_id: Address,
    ) -> Result<(), Vec<u8>> {
        let saving_data = self.savings_map.get(name_of_saving.clone());
        if !saving_data.is_valid.get() {
            return Err(format!("Saving `{}` doesn't exist", name_of_saving).into());
        };

        if !saving_data.token_id.eq(&token_id) {
            // token not same with one being saved
            return Err("Different token being saved, create new saving".into());
        }

        let old_interest = saving_data.interest_accumulated.get();
        let old_amount = saving_data.amount.get();

        // saving is valid, increment the saving data
        let new_interest = self.calculate_new_interest(new_amount);

        let mut saving_updater = self.savings_map.setter(name_of_saving);

        // increment amount and interest
        saving_updater
            .interest_accumulated
            .set(old_interest + new_interest);
        saving_updater.amount.set(old_amount + new_amount);

        // saving updated
        Ok(())
    }

    pub fn withdraw_saving_data(&mut self, name_of_saving: String) -> Result<U256, Vec<u8>> {
        let saving_data = self.savings_map.get(name_of_saving.clone());
        if !saving_data.is_valid.get() {
            return Err(format!("Saving `{}` doesn't exist", name_of_saving).into());
        }

        let mut withdraw_amount: U256 = U256::from(0);

        // check if maturity is complete
        let saving_amount = saving_data.amount.get();
        if saving_data.maturity_time.get() < U256::from(block::timestamp()) {
            // saving isn't complete, remove percentage
            withdraw_amount =
                Self::calculate_balance_from_penalty(saving_amount, saving_data.penalty_perc.get());
        } else {
            // saving complete, send interest
            withdraw_amount = saving_amount;
            // todo: send interest
        }

        // clear saving data
        // is_valid, amount, interest_accumulated, penalty_perc
        let mut saving_updater = self.savings_map.setter(name_of_saving);

        saving_updater.is_valid.set(false);
        saving_updater.amount.set(U256::from(0));
        saving_updater.interest_accumulated.set(U256::from(0));
        saving_updater.penalty_perc.set(U8::from(0));

        Ok(withdraw_amount)
    }
}
