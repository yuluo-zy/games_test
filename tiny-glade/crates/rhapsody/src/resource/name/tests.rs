use super::*;
#[test]
fn equal_strings_share_one_identity_and_different_strings_do_not() {
    let a = ResourceName::new("main_color");
    let b = ResourceName::from(String::from("main_color"));
    let c = ResourceName::from(Arc::<str>::from("main_color_2"));
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert!(std::ptr::eq(a.as_str(), b.as_str()), "interned once");
    assert_eq!(a, "main_color");
    assert_eq!(format!("{a}"), "main_color");
    assert_eq!(format!("{a:?}"), "\"main_color\"");
    assert_eq!(a.len(), 10, "derefs to str");
}
#[test]
fn hashing_and_ordering_follow_the_string() {
    use std::collections::{BTreeSet, HashSet};
    let names = ["b", "a", "c", "a"].map(ResourceName::new);
    assert_eq!(names.iter().copied().collect::<HashSet<_>>().len(), 3);
    let sorted: Vec<_> = names
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(sorted, ["a", "b", "c"]);
}
