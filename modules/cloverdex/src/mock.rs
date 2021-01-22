#![cfg(test)]
use super::*;
use frame_support::{
  impl_outer_event, impl_outer_origin, parameter_types,
  weights::{
    Weight,
    constants::{WEIGHT_PER_SECOND, BlockExecutionWeight, ExtrinsicBaseWeight},
    DispatchClass,
  },
};
use frame_system::{limits};

use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
use sp_std::cell::RefCell;
use std::collections::HashMap;
pub use pallet_balances::Call as BalancesCall;
use clover_traits::{IncentiveOps, IncentivePoolAccountInfo, };
use orml_traits::parameter_type_with_key;
// use sp_std::marker;
// use orml_traits::{
//   MultiCurrency, OnDust,
// };

pub use primitives::{
  AccountId, AccountIndex, Amount, Balance,
  CurrencyId,
  EraIndex, Hash, Index, Moment,
  Rate, Share,
  Signature,
};

use orml_currencies::{BasicCurrencyAdapter};

pub type BlockNumber = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

mod cloverdex {
  pub use super::super::*;
}

impl_outer_event! {
  pub enum TestEvent for TestRuntime {
    frame_system<T>,
    cloverdex<T>,
    orml_tokens<T>,
    orml_currencies<T>,
    pallet_balances<T>,
  }
}
impl_outer_origin! {
  pub enum Origin for TestRuntime {}
}

pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;
// pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_perthousand(25);


parameter_types! {
  pub BlockLength: limits::BlockLength =
    limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

  pub const BlockHashCount: BlockNumber = 2400;
  /// We allow for 2 seconds of compute with a 6 second average block time.
  pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have an extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
    .build_or_panic();

  // pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
  // pub const Version: RuntimeVersion = VERSION;
  pub const SS58Prefix: u8 = 229; // Ss58AddressFormat::CloverAccount
}
impl frame_system::Config for TestRuntime {
  type Origin = Origin;
  type Index = u64;
  type BlockNumber = BlockNumber;
  type Call = ();
  type Hash = H256;
  type Hashing = ::sp_runtime::traits::BlakeTwo256;
  type AccountId = AccountId;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = TestEvent;
  type Version = ();
  type PalletInfo = ();
  type AccountData = pallet_balances::AccountData<Balance>;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type DbWeight = ();
  type BaseCallFilter = ();
  type SystemWeightInfo = ();
  type BlockHashCount = BlockHashCount;
  type BlockWeights = BlockWeights;
  type BlockLength = BlockLength;
  type SS58Prefix = SS58Prefix;
}

pub type System = frame_system::Module<TestRuntime>;

parameter_types! {
  pub const ExistentialDeposit: u128 = 500;
  pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type Event = TestEvent;
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type MaxLocks = MaxLocks;
  type WeightInfo = ();
}

pub type Balances = pallet_balances::Module<TestRuntime>;

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&BTC => 1,
			&DOT => 2,
			_ => 0,
		}
	};
}



parameter_types! {
	pub DustAccount: AccountId = ModuleId(*b"orml/dst").into_account();
}

impl orml_tokens::Config for TestRuntime {
  type Event = TestEvent;
  type Balance = Balance;
  type Amount = Amount;
  type CurrencyId = CurrencyId;
  type WeightInfo = ();
  type ExistentialDeposits = ExistentialDeposits;
  type OnDust = ();
}

pub type Tokens = orml_tokens::Module<TestRuntime>;

parameter_types! {
  pub const GetNativeCurrencyId: CurrencyId = CurrencyId::CLV;
}

impl orml_currencies::Config for TestRuntime {
  type Event = TestEvent;
  type MultiCurrency = Tokens;
  type NativeCurrency = BasicCurrencyAdapter<TestRuntime, Balances, Amount, BlockNumber>;
  type GetNativeCurrencyId = GetNativeCurrencyId;
  type WeightInfo = ();
}

pub type Currencies = orml_currencies::Module<TestRuntime>;

parameter_types! {
  pub GetExchangeFee: Rate = Rate::saturating_from_rational(1, 100);
  pub const CloverdexModuleId: ModuleId = ModuleId(*b"clv/dexm");
}

impl Config for TestRuntime {
  type Event = TestEvent;
  type Currency = Currencies;
  type Share = Share;
  type GetExchangeFee = GetExchangeFee;
  type ModuleId = CloverdexModuleId;
  type OnAddLiquidity = ();
  type OnRemoveLiquidity = ();
  type IncentiveOps = IncentiveOpsHandler;
}

pub type CloverdexModule = Module<TestRuntime>;

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const CLV: CurrencyId = CurrencyId::CLV;
pub const CUSDT: CurrencyId = CurrencyId::CUSDT;
pub const DOT: CurrencyId = CurrencyId::DOT;
pub const CETH: CurrencyId = CurrencyId::CETH;

thread_local! {
  pub static SHARES_STAKED: RefCell<HashMap<(AccountId, PairKey), Balance>> = RefCell::new(HashMap::new());
}

pub struct IncentiveOpsHandler;

impl IncentiveOps<AccountId, CurrencyId, Share, Balance> for IncentiveOpsHandler {
  fn add_share(who: &AccountId, left: &CurrencyId, right: &CurrencyId, amount: &Share) -> Result<Share, DispatchError> {
    let t = SHARES_STAKED.with(|v| {
      let total;
      let mut old_map = v.borrow().clone();
      let key = CloverdexModule::get_pair_key(left, right);
      if let Some(before) = old_map.get_mut(&(who.clone(), key)) {
        *before += amount;
        total = before.clone();
      } else {
        old_map.insert((who.clone(), key), amount.clone());
        total = amount.clone();
      };
      *v.borrow_mut() = old_map;
      total
    });
    Ok(t)
  }

  fn remove_share(who: &AccountId, left: &CurrencyId, right: &CurrencyId, amount: &Share) -> Result<Share, DispatchError> {
    let total = SHARES_STAKED.with(|v| {
      let total;
      let mut old_map = v.borrow().clone();
      let key = CloverdexModule::get_pair_key(left, right);
      if let Some(before) = old_map.get_mut(&(who.clone(), key)) {
        *before -= amount;
        total = before.clone();
      } else {
        old_map.insert((who.clone(), key), amount.clone());
        total = amount.clone();
      };
      *v.borrow_mut() = old_map;
      total
    });
    Ok(total)
  }

  fn get_account_shares(who: &AccountId, left: &CurrencyId, right: &CurrencyId) -> Share {
    SHARES_STAKED.with(|v| {
      let key = CloverdexModule::get_pair_key(left, right);
      v.borrow().get(&(who.clone(), key)).unwrap_or(&0).clone()
    })
  }

  // todo implement it
  fn get_accumlated_rewards(_who: &AccountId, _left: &CurrencyId, _right: &CurrencyId) -> Balance {
    0
  }

  fn get_account_info(_who: &AccountId, _left: &CurrencyId, _right: &CurrencyId) -> IncentivePoolAccountInfo<Share, Balance> {
    IncentivePoolAccountInfo { shares: 0, accumlated_rewards: 0 }
  }

  fn claim_rewards(_who: &AccountId, _left: &CurrencyId, _right: &CurrencyId) -> Result<Balance, DispatchError> {
    Ok(Zero::zero())
  }

  fn get_all_incentive_pools() -> Vec<(CurrencyId, CurrencyId, Share, Balance)> {
    vec![]
  }
}

pub struct ExtBuilder {
  endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
  fn default() -> Self {
    let alice = AccountId::from(ALICE);
    let bob = AccountId::from(BOB);

    Self {
      endowed_accounts: vec![
        (alice.clone(), CLV, 1_000_000_000_000_000_000u128),
        (bob.clone(), CLV, 1_000_000_000_000_000_000u128),
        (alice.clone(), CUSDT, 1_000_000_000_000_000_000u128),
        (bob.clone(), CUSDT, 1_000_000_000_000_000_000u128),
        (alice.clone(), DOT, 1_000_000_000_000_000_000u128),
        (bob.clone(), DOT, 1_000_000_000_000_000_000u128),
        (alice.clone(), CETH, 1_000_000_000_000_000_000u128),
        (bob.clone(), CETH, 1_000_000_000_000_000_000u128),
      ],
    }
  }
}

impl ExtBuilder {
  pub fn build(self) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
      .build_storage::<TestRuntime>()
      .unwrap();

    pallet_balances::GenesisConfig::<TestRuntime> {
      balances: self
        .endowed_accounts
        .clone()
        .into_iter()
        .filter(|(_, currency_id, _)| *currency_id == CLV)
        .map(|(account_id, _, initial_balance)| (account_id, initial_balance))
        .collect::<Vec<_>>(),
    }
    .assimilate_storage(&mut t)
      .unwrap();


    orml_tokens::GenesisConfig::<TestRuntime> {
      endowed_accounts: self
        .endowed_accounts
        .into_iter()
        .filter(|(_, currency_id, _)| *currency_id != CLV)
        .collect::<Vec<_>>(),
    }
    .assimilate_storage(&mut t).unwrap();

    // cloverdex::GenesisConfig {
    //   initial_pairs: vec![
    //     (CLV, CETH, Some(0), Some(0)),
    //     (CUSDT, CETH, Some(0), Some(0)),
    //     (CUSDT, DOT, Some(0), Some(0)),
    //     (DOT, CETH, Some(0), Some(0)),
    //   ],
    // }.assimilate_storage::<TestRuntime>(&mut t).unwrap();

    t.into()
  }
}
