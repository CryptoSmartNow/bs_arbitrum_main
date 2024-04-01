use alloy_primitives::U256;
use stylus_sdk::stylus_proc::sol_storage;

sol_storage! {
    pub struct userData {
        address userAddress;
        uint256 userId;
        mapping(uint256 => uint256) savingsMap;
    }
}

impl userData {
    pub fn get_user_id(&self) -> U256 {
        self.userId().get()
    }
}
