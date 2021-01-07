use sp_runtime::DispatchError;
use sp_core::H160;

// pub trait OwnerManagerOps<AccountId> {
//     fn get_owner_address(who: &AccountId, contract_address: Option<H160>) -> Option<H160>;
//     fn get(who: &AccountId, contract_address: H160) -> H160;
//     fn contain_key(who: &AccountId, contract_address: H160) -> bool;
//     fn manager_owner(who: &AccountId, contact_address: H160, source_owner_address: H160) -> Result<(),DispatchError>;
// }
pub trait OwnerManagerOps{
    fn get_owner_address(contract_address: Option<H160>) -> Option<H160>;
    fn get(contract_address: H160) -> H160;
    fn contain_key(contract_address: H160) -> bool;
    fn manager_owner(contact_address: H160, source_owner_address: H160) -> Result<(),DispatchError>;
}