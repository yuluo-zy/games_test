use super::*;
#[test]
fn family_keys_are_canonical_and_invalid_lists_are_rejected() {
    assert_eq!(
        QueueFamilies::new(&[5, 1, 3]).unwrap(),
        QueueFamilies::new(&[1, 3, 5]).unwrap()
    );
    assert!(QueueFamilies::new(&[1]).is_err());
    assert!(QueueFamilies::new(&[1, 1]).is_err());
    assert!(QueueFamilies::new(&[0; MAX_QUEUES + 1]).is_err());
    let sharing = ResourceSharing::Concurrent(QueueFamilies::new(&[1, 3]).unwrap());
    assert!(sharing.allows_family(3));
    assert!(!sharing.allows_family(2));
}
