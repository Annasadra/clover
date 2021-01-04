#![cfg_attr(not(feature = "std"), no_std)]

//! A simple pallet with two storage values. The pallet itself does not teach any new concepts.
//! Rather we use this pallet as demonstration case as we demonstrate custom runtime APIs.
//! This pallet supports a runtime API which will allow querying the runtime for the sum of
//! the two storage items.
#[cfg(test)]
mod test;

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, ensure};
// use frame_system::{ensure_signed, RawOrigin, Origin};
use frame_system::ensure_signed;
use sp_core::H160;
use sp_std::prelude::*;
// use sp_runtime::traits::BadOrigin;
// use sp_core::crypto::AccountId32;

// pub trait EnsureAddressOrigin<OuterOrigin> {
//     /// Success return type
//     type Success;
//
//     /// Perform the origin check.
//     fn ensure_address_origin(
//         address: &H160,
//         origin: OuterOrigin,
//     ) -> Result<Self::Success, BadOrigin> {
//         Self::try_address_origin(address, origin).map_err(|_| BadOrigin)
//     }
//
//     /// Try with origin
//     fn try_address_origin(
//         address: &H160,
//         origin: OuterOrigin,
//     ) -> Result<Self::Success, OuterOrigin>;
// }
//
// /// Ensure that the EVM address is the same as the Substrate address. This only works if the account
// /// ID is 'H160'.
// pub struct EnsureAddressSame;
//
// impl <OuterOrigin> EnsureAddressOrigin<OuterOrigin> for EnsureAddressSame
//     where OuterOrigin: Into<Result<RawOrigin<H160>, OuterOrigin>> + From<RawOrigin<H160>>,
// {
//     type Success = H160;
//     fn try_address_origin (
//         address: &H160,
//         origin: OuterOrigin,
//     ) -> Result<H160, OuterOrigin> {
//         origin.into().and_then(|o| match o {
//             RawOrigin::Signed(who)  if &who == address => Ok(who),
//             r => Err(OuterOrigin::from(r)),
//         })
//     }
// }
//
// /// Ensure that the address is truncated hash of the origin. Only works id the account id is
// /// 'AccountId32'.
// pub struct EnsureAddress;
//
// impl<OuterOrigin> EnsureAddressOrigin<OuterOrigin> for EnsureAddress where
//     OuterOrigin: Into<Result<RawOrigin<AccountId32>, OuterOrigin>> + From<RawOrigin<AccountId32>>,
// {
//     type Success = AccountId32;
//
//     fn try_address_origin(
//         address: &H160,
//         origin: OuterOrigin,
//     ) -> Result<AccountId32, OuterOrigin> {
//         origin.into().and_then(|o| match o {
//             RawOrigin::Signed(who)
//                 if AsRef::<[u8; 32]>::as_ref(&who)[0..20] == address[0..20] => Ok(who),
//             r => Err(OuterOrigin::from(r))
//         })
//     }
// }

/// The module's configuration trait
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    // Allow the origin to call
    // type CallOwner: EnsureAddressOrigin<Self::Origin>;
}

// The pallet's runtime storage items.
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    trait Store for Module<T: Trait> as OwnerManager {
        OwnerMaps get(fn owner_maps): map hasher(blake2_128_concat) H160 => H160;
    }
}

// Pallets use events to inform users when important changes are made.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        ManagerOwner(AccountId, H160, H160),
        ValueSet(u32, u32),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        ContractAddressAlreadyExist,
        ContractAddressNotExist,
        Normal,
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // Errors mut be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        // find contract address
        // if the contract address and source owner address is news,
        // then insert to map
        // or mut V(source owner address) by  contract address index
        // K - contract address
        // V - transfer owner address
        #[weight = 0]
        pub fn manager_owner(origin, contract_address: H160, source_owner_address: H160) -> dispatch::DispatchResult {
            // T::CallOwner::ensure_address_origin(&contract_address, origin)?;
            // debug::info!("Into owner manager");
            let sender = ensure_signed(origin)?;

            // identify contract address is exist, to insert (K-V) or mutate (value)
            if OwnerMaps::contains_key(&contract_address) { // exist so, mutate V
                // debug::info!("contain contract address");
                ensure!(OwnerMaps::contains_key(&contract_address), Error::<T>::ContractAddressNotExist);
                OwnerMaps::mutate(contract_address, |old_value| *old_value = source_owner_address);

            } else { // no exist so, insert K-V
                // debug::info!("no contain contract address");
                ensure!(!OwnerMaps::contains_key(&contract_address), Error::<T>::ContractAddressAlreadyExist);
                OwnerMaps::insert(contract_address, source_owner_address);
            }
            ensure!(OwnerMaps::contains_key(&contract_address), Error::<T>::ContractAddressNotExist);

            Self::deposit_event(RawEvent::ManagerOwner(sender, contract_address, source_owner_address));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {

    /// get owner_address
    /// if owner address not exist return None
    /// else return Some(H160), this wrap owner address
    pub fn get_owner_address(contract_address: Option<H160>) -> Option<H160> {
        match contract_address {
            None => None,
            Some(contract_address) => {
                let ret: Vec<(H160, H160)> = OwnerMaps::iter()
                    .filter(|(val, _)| contract_address == *val)
                    .collect();
                if ret.len() == 1 {
                    return Some(ret[0].1);
                } else {
                    return None;
                }
            }
        }
    }

    /// get the Value
    pub fn get(contract_address: H160) -> H160 {
        let contract_address : Option<H160> = Some(contract_address);
        Self::get_owner_address(contract_address).unwrap()
    }

    /// contains key
    /// if contract address is not exist return false
    /// else return true
    pub fn contains_key(contract_address: H160) -> bool {
        let contract_address: Option<H160> = Some(contract_address);
        match Self::get_owner_address(contract_address) {
            None => false,
            Some(_) => true,
        }
    }
}
