use asset::prefab::{PrefabName, PrefabNameRef};
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::Deref,
};

#[test]
fn borrowed_name_views_share_bytes_and_map_hash_without_normalizing() {
    fn hash(value: &impl Hash) -> u64 {
        let mut state = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut state);
        state.finish()
    }
    for text in [
        "",
        "body",
        "BODY",
        "path/name",
        "path\\name",
        " 木头 ",
        "embedded\0nul",
    ] {
        let name = PrefabName::from(text);
        let borrowed: &PrefabNameRef = name.borrow();
        assert_eq!(borrowed.as_str(), text);
        assert!(std::ptr::eq(borrowed.as_str(), name.as_str()));
        assert!(std::ptr::eq(borrowed, name.as_name_ref()));
        assert!(std::ptr::eq(borrowed, name.deref()));
        assert!(std::ptr::eq(
            borrowed,
            AsRef::<PrefabNameRef>::as_ref(&name)
        ));
        assert!(std::ptr::eq(
            borrowed,
            AsRef::<PrefabNameRef>::as_ref(borrowed)
        ));
        assert_eq!(AsRef::<str>::as_ref(&name), text);
        assert_eq!(hash(&name), hash(&borrowed));
        let map = HashMap::from([(name.clone(), 7)]);
        assert_eq!(map.get(PrefabNameRef::from_str_ref(text)), Some(&7));
        assert_eq!(
            map.get(PrefabNameRef::from_str_ref(&format!("{text}_other"))),
            None
        );
    }
}
