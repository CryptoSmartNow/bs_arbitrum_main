
pub const ABI: &str = 
"[
            function getBitsaveUserCount() external view returns (uint256)
            function joinBitsave() external payable returns (address)
            function fund() external payable returns (uint256)
            function createSaving(string calldata name_of_saving, uint256 maturity_time, uint8 penalty_perc, bool use_safe_mode) external

            function incrementSaving(string calldata name_of_saving) external

            function withdrawSavings(string calldata name_of_saving) external returns (uint256)
        ]";

