#![cfg(test)]

use super::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

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
  type BaseCallFilter = ();
  type AccountId = AccountId;
  type Call = Call;
  type Lookup = Indices;
  type Index = Index;
  type BlockNumber = BlockNumber;
  type Hash = Hash;
  type Hashing = BlakeTwo256;
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  type Event = Event;
  type Origin = Origin;
  type BlockHashCount = BlockHashCount;
  // type MaximumBlockWeight = MaximumBlockWeight;
  type DbWeight = RocksDbWeight;
  // type BlockExecutionWeight = BlockExecutionWeight;
  // type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
  // type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
  // type MaximumBlockLength = MaximumBlockLength;
  // type AvailableBlockRatio = AvailableBlockRatio;
  type BlockWeights = BlockWeights;
  type BlockLength = BlockLength;
  type SS58Prefix = SS58Prefix;
  type Version = Version;
  type PalletInfo = ();
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type AccountData = pallet_balances::AccountData<Balance>;
  type SystemWeightInfo = ();
}

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const DAVE: [u8; 32] = [2u8; 32];
pub const CLV: CurrencyId = CurrencyId::CLV;

pub struct ExtBuilder {
  endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
  fn default() -> Self {
    Self {
      endowed_accounts: vec![],
    }
  }
}

impl ExtBuilder {
  pub fn balances(mut self, endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
    self.endowed_accounts = endowed_accounts;
    self
  }

  pub fn build(self) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
      .build_storage::<Runtime>()
      .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
      balances: self
        .endowed_accounts
        .clone()
        .into_iter()
        .filter(|(_, currency_id, _)| *currency_id == CLV)
        // the balance of any account should always be more than existential deposit.
        .map(|(account_id, _, _initial_balance)| (account_id, 500))
        .collect::<Vec<_>>(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    orml_tokens::GenesisConfig::<Runtime> {
      endowed_accounts: self
        .endowed_accounts
        .into_iter()
        .filter(|(_, currency_id, _)| *currency_id != CLV)
        .collect::<Vec<_>>(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_membership::GenesisConfig::<Runtime, pallet_membership::Instance1> {
      members: vec![
        AccountId::from(ALICE),
        AccountId::from(BOB),
        AccountId::from(DAVE),
      ],
      phantom: Default::default(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
  }
}
