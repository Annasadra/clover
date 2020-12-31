#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
use sp_core::H160;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {

    pub trait OwnerManagerApi {
        fn get_owner_address(contract_address: Option<H160>) -> Option<H160>;
    }
}
