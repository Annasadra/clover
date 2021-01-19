// Copyright (C) 2021 Clover Finance.
// This file is part of Clover.

use frame_support::traits::{OnUnbalanced, Imbalance, Currency};
use crate::NegativeImbalance;

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
    where
        R: pallet_balances::Config + pallet_authorship::Config,
        <R as frame_system::Config>::AccountId: From<primitives::AccountId>,
        <R as frame_system::Config>::AccountId: Into<primitives::AccountId>,
        <R as frame_system::Config>::Event: From<pallet_balances::RawEvent<
            <R as frame_system::Config>::AccountId,
            <R as pallet_balances::Config>::Balance,
            pallet_balances::DefaultInstance>
        >,
{
    fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
        let numeric_amount = amount.peek();
        let author = <pallet_authorship::Module<R>>::author();
        <pallet_balances::Module<R>>::resolve_creating(&<pallet_authorship::Module<R>>::author(), amount);
        <frame_system::Module<R>>::deposit_event(pallet_balances::RawEvent::Deposit(author, numeric_amount));
    }
}
