#![cfg(test)]
use super::*;
use frame_support::{
  impl_outer_event, impl_outer_origin, parameter_types,
  traits::{OnFinalize, OnInitialize},
  weights::{
    Weight,
    constants::{WEIGHT_PER_SECOND, BlockExecutionWeight, ExtrinsicBaseWeight},
    DispatchClass,
  },
};
use frame_system::{limits};

use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
pub use pallet_balances::Call as BalancesCall;

pub use primitives::{
  AccountId, AccountIndex, Amount, Balance,
  CurrencyId,
  EraIndex, Hash, Index, Moment,
  Rate, Share,
  Signature,
  currency::*,
};

use orml_traits::parameter_type_with_key;
// use sp_std::marker;
use orml_traits::{
  MultiCurrency,
  OnDust,
};

use orml_currencies::{BasicCurrencyAdapter};

pub type BlockNumber = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

mod reward_pool{
  pub use super::super::*;
}

impl_outer_event! {
  pub enum TestEvent for TestRuntime {
    frame_system<T>,
    reward_pool<T>,
    orml_tokens<T>,
    orml_currencies<T>,
    pallet_balances<T>,
  }
}

impl_outer_origin! {
  pub enum Origin for TestRuntime {}
}

pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;
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
  type BlockHashCount = BlockHashCount;
  type BlockWeights = BlockWeights;
  type BlockLength = BlockLength;
  type SS58Prefix = SS58Prefix;
  type Version = ();
  type PalletInfo = ();
  type AccountData = pallet_balances::AccountData<Balance>;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type DbWeight = ();
  type BaseCallFilter = ();
  type SystemWeightInfo = ();
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

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&BTC => 1,
			&DOT => 2,
			_ => 0,
		}
	};
}

pub type Balances = pallet_balances::Module<TestRuntime>;

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
  // type OnDust = TransferDust<TestRuntime, DustAccount>;
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
  pub const RewardPoolModuleId: ModuleId = ModuleId(*b"clv/repm");
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Ord, PartialOrd)]
pub enum PoolId {
  Swap(u64),
}

pub struct Handler;
impl RewardHandler<AccountId, BlockNumber, Balance, Share, PoolId> for Handler {
  // simple reward calculation, 1 block 1 reward
  fn caculate_reward(pool_id: &PoolId, total_share: &Share, last_update_block: BlockNumber,
                     now: BlockNumber) -> Balance {
    println!("calculate reward for pool: {:?}", pool_id);
    if total_share.is_zero() {
      println!("no reward because no share in pool, pool: {:?}", pool_id);
      0
    } else {
      DOLLARS.checked_mul((now - last_update_block).into()).unwrap()
    }
  }
}

impl Config for TestRuntime {
  type Event = TestEvent;
  type Currency = Currencies;
  type ModuleId = RewardPoolModuleId;
  type GetNativeCurrencyId = GetNativeCurrencyId;
  type PoolId = PoolId;
  type Handler = Handler;
  type ExistentialReward = ExistentialDeposit;
}

pub type RewardPoolModule = Module<TestRuntime>;

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const DAVE: [u8; 32] = [2u8; 32];
pub const CLV: CurrencyId = CurrencyId::CLV;
pub const CUSDT: CurrencyId = CurrencyId::CUSDT;
pub const DOT: CurrencyId = CurrencyId::DOT;
pub const CETH: CurrencyId = CurrencyId::CETH;

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

    t.into()
  }
}

pub fn run_to_block(n: u64) {
  while System::block_number() < n {
    RewardPoolModule::on_finalize(System::block_number());
    Currencies::on_finalize(System::block_number());
    Tokens::on_finalize(System::block_number());
    Balances::on_finalize(System::block_number());
    System::on_finalize(System::block_number());
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    Balances::on_initialize(System::block_number());
    Tokens::on_initialize(System::block_number());
    Currencies::on_initialize(System::block_number());
    RewardPoolModule::on_initialize(System::block_number());
  }
}
