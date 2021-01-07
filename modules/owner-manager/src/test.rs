use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use sp_core::H160;
use sp_serializer as ser;

#[test]
fn default_owner_manager_none() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            <OwnerManager as OwnerManagerOps>::get_owner_address(None),
            None
        );
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
            <OwnerManager as OwnerManagerOps>::get_owner_address(Some(
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
            <OwnerManager as OwnerManagerOps>::get_owner_address(Some(
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
            <OwnerManager as OwnerManagerOps>::get_owner_address(Some(
                ser::from_str::<H160>("\"0x70308ff12c223874276094bfd0b853821a09e0de\"").unwrap()
            )),
            None
        );
    })
}

#[test]
fn test_get_func() {
    new_test_ext().execute_with(|| {
        assert_ok!(OwnerManager::manager_owner(
            Origin::signed(1),
            ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap(),
            ser::from_str::<H160>("\"0xff7850ee4c035baf133b075f24803062595bdd5c\"").unwrap()
        ));

        assert_eq!(
            ser::from_str::<H160>("\"0xff7850ee4c035baf133b075f24803062595bdd5c\"").unwrap(),
            <OwnerManager as OwnerManagerOps>::get(
                ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap()
            )
        );
    })
}

#[test]
fn test_contains_key_success() {
    new_test_ext().execute_with(|| {
        assert_ok!(OwnerManager::manager_owner(
            Origin::signed(1),
            ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap(),
            ser::from_str::<H160>("\"0xff7850ee4c035baf133b075f24803062595bdd5c\"").unwrap()
        ));
        assert!(<OwnerManager as OwnerManagerOps>::contain_key(
            ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b83\"").unwrap()
        ));
    })
}

#[test]
fn test_contains_key_failed_when_key_not_exist() {
    new_test_ext().execute_with(|| {
        assert_eq!(
            false,
            <OwnerManager as OwnerManagerOps>::contain_key(
                ser::from_str::<H160>("\"0x7a9e0b6100780dc229b524e1d237cce53eaf2b33\"").unwrap()
            )
        );
    })
}
