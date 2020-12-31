use super::*;

use frame_support::{assert_ok, impl_outer_origin, parameter_types, weights::Weight};
use sp_core::{H160, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

use sp_serializer as ser;

impl_outer_origin! {
    pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type ('test') which 'impl's each of the
// configuration trait of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

impl Trait for Test {
    type Event = ();
}

type OwnerManager = Module<Test>;

// This function basically just  builds a genesis storage key/value storage according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn default_owner_manager_none() {
    new_test_ext().execute_with(|| {
        assert_eq!(OwnerManager::get_owner_address(None), None);
    });
}

#[test]
fn insert_one_key_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(OwnerManager::manager_owner(
            Origin::signed(1),
            ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap(),
            ser::from_str::<H160>("\"0x4294a885223cde8b89c5e9e4cc5ac747d97cb17d\"").unwrap()
        ));
        assert_eq!(
            OwnerManager::get_owner_address(Some(
                ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap()
            )),
            Some(ser::from_str::<H160>("\"0x4294a885223cde8b89c5e9e4cc5ac747d97cb17d\"").unwrap())
        );
    })
}

#[test]
fn mutate_owner_address() {
    new_test_ext().execute_with(|| {
        assert_ok!(OwnerManager::manager_owner(
            Origin::signed(1),
            ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap(),
            ser::from_str::<H160>("\"0xff7850ee4c035baf133b075f24803062595bdd5c\"").unwrap()
        ));
        assert_eq!(
            OwnerManager::get_owner_address(Some(
                ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap()
            )),
            Some(ser::from_str::<H160>("\"0xff7850ee4c035baf133b075f24803062595bdd5c\"").unwrap())
        );
    })
}

#[test]
fn get_none_address() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            OwnerManager::get_owner_address(Some(
                ser::from_str::<H160>("\"0x70308ff12c223874276094bfd0b853821a09e0de\"").unwrap()
            )),
            None
        );
    })
}
