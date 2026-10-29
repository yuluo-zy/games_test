use super::*;
use crate::error::ErrorKind;

#[test]
fn generation_is_monotonic_and_distinct() {
    let first = DeviceGeneration::next();
    let second = DeviceGeneration::next();
    assert!(first.is_valid());
    assert_ne!(first, second);
    assert!(second.value() > first.value());
}

#[test]
fn invalid_generation_is_never_valid() {
    assert!(!DeviceGeneration::INVALID.is_valid());
    assert_eq!(DeviceGeneration::INVALID.value(), 0);
    assert_eq!(std::mem::size_of::<DeviceGeneration>(), 8);
}

#[test]
fn generationed_gate_accepts_only_the_matching_generation() {
    let generation = DeviceGeneration::next();
    let item = Generationed::new(42u32, generation);

    assert_eq!(*item.check(generation).unwrap(), 42);
    assert_eq!(item.into_value(generation).unwrap(), 42);

    let stale = Generationed::new(7u32, generation);
    let other = DeviceGeneration::next();
    let error = stale.check(other).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::InvalidUsage);
    assert_eq!(
        stale.into_value(other).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
}

#[test]
fn uninitialized_generations_do_not_authorize_access() {
    let item = Generationed::new(42, DeviceGeneration::INVALID);
    assert_eq!(
        item.check(DeviceGeneration::INVALID).unwrap_err().kind(),
        ErrorKind::InvalidUsage
    );
    assert_eq!(
        item.into_value(DeviceGeneration::INVALID)
            .unwrap_err()
            .kind(),
        ErrorKind::InvalidUsage
    );
}
