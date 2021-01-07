#![cfg_attr(not(feature = "std"), no_std)]

//! A simple pallet with two storage values. The pallet itself does not teach any new concepts.
//! Rather we use this pallet as demonstration case as we demonstrate custom runtime APIs.
//! This pallet supports a runtime API which will allow querying the runtime for the sum of
//! the two storage items.
#[cfg(test)]
mod test;

#[cfg(test)]
mod mock;

use clover_traits::OwnerManagerOps;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
    weights::constants::WEIGHT_PER_MICROS,
};
use frame_system::ensure_signed;
use sp_core::H160;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

/// The module's configuration trait
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
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
        OwnerUpdated(AccountId, H160, H160),
        ValueSet(u32, u32),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        ContractAddressAlreadyExist,
        ContractAddressNotExist,
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

        /// find contract address
        /// if the contract address and source owner address is news,
        /// then insert to map
        /// or mut V(source owner address) by  contract address index
        /// K - contract address
        /// V - transfer owner address
        /// # <weight>
        /// - Base Weight: 206 * WEIGHT_PER_MICROS
        /// - DB Weight: 1 Read, and Write
        /// # </weight>
        #[weight = 206 * WEIGHT_PER_MICROS + T::DbWeight::get().reads_writes(1,1)]
        pub fn manager_owner(origin, contract_address: H160, source_owner_address: H160) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            let ret = <Self as OwnerManagerOps>::manager_owner(contract_address, source_owner_address);
            Self::deposit_event(RawEvent::OwnerUpdated(sender, contract_address, source_owner_address));
            ret
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
                if OwnerMaps::contains_key(contract_address) {
                    Some(OwnerMaps::get(contract_address))
                } else {
                    None
                }
            }
        }
    }
}

impl<T: Trait> OwnerManagerOps for Module<T> {
    fn get_owner_address(contract_address: Option<H160>) -> Option<H160> {
        Self::get_owner_address(contract_address)
    }

    fn get(contract_address: H160) -> H160 {
        let contract_address: Option<H160> = Some(contract_address);
        Self::get_owner_address(contract_address).unwrap()
    }

    fn contain_key(contract_address: H160) -> bool {
        let contract_address: Option<H160> = Some(contract_address);
        match Self::get_owner_address(contract_address) {
            None => false,
            Some(_) => true,
        }
    }

    fn manager_owner(
        contract_address: H160,
        source_owner_address: H160,
    ) -> Result<(), DispatchError> {
        // identify contract address is exist, to insert (K-V) or mutate (value)
        if OwnerMaps::contains_key(&contract_address) {
            // exist so, mutate V
            OwnerMaps::mutate(contract_address, |old_value| {
                *old_value = source_owner_address
            });
        } else {
            // no exist so, insert K-V
            OwnerMaps::insert(contract_address, source_owner_address);
        }
        ensure!(
            OwnerMaps::contains_key(&contract_address),
            Error::<T>::ContractAddressNotExist
        );
        Ok(())
    }
}
