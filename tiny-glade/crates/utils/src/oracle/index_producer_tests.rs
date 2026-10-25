use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Snapshot {
    keys: Vec<u32>,
    mapping: HashMap<u32, u32>,
    map_len: usize,
}
#[derive(Deserialize)]
struct Insert {
    keys: Vec<u32>,
    key: u32,
    index: Option<u32>,
    after: Snapshot,
}
#[derive(Deserialize)]
struct Remove {
    keys: Vec<u32>,
    key: u32,
    cached: u32,
    index: Option<u32>,
    updated_cached: u32,
    after: Snapshot,
}
#[derive(Deserialize)]
struct Fixtures {
    insert: Vec<Insert>,
    remove: Vec<Remove>,
}
fn fixture() -> Fixtures {
    serde_json::from_str(include_str!("../../tests/fixtures/index-producer.json")).unwrap()
}
fn populated(keys: &[u32]) -> IndexCondenser<u32> {
    let mut c = IndexCondenser::default();
    for &key in keys {
        c.insert(key).unwrap();
    }
    c
}
fn check_snapshot(c: &IndexCondenser<u32>, expected: Snapshot) {
    assert_eq!(c.keys(), expected.keys);
    assert_eq!(c.indices, expected.mapping);
    assert_eq!(c.indices.len(), expected.map_len);
}
#[test]
fn original_insert_rejects_duplicate_and_appends_new_key() {
    for row in fixture().insert {
        let mut c = populated(&row.keys);
        let h = c.insert(row.key);
        assert_eq!(h.map(|v| v.last_known_index()), row.index);
        check_snapshot(&c, row.after);
    }
}
#[test]
fn original_complete_removal_returns_index_and_repairs_mapping() {
    for row in fixture().remove {
        let mut c = populated(&row.keys);
        let h = Condex {
            key: row.key,
            cached_index: Cell::new(row.cached),
        };
        assert_eq!(c.remove_index(&h), row.index);
        assert_eq!(h.last_known_index(), row.updated_cached);
        check_snapshot(&c, row.after);
    }
}
