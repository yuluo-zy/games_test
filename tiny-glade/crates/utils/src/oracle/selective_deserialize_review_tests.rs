use super::*;
#[test]
fn borrowed_key_selection_preserves_nested_partial_success() {
    let input=br#"{"unknown":{"deep":[1,2]},"History":{"snap\u0073hots":[7,9],"snapshots":"bad"},"broken": }"#;
    let mut deserializer = serde_json::Deserializer::from_slice(input);
    let value = SaveHistorySnapshots::<Vec<u64>>::deserialize(&mut deserializer).unwrap();
    assert_eq!(value.value.value, vec![7, 9]);
    let key = serde::de::value::BorrowedStrDeserializer::<serde::de::value::Error>::new("History");
    assert!(SelectedKey::<HistoryField>::deserialize(key).unwrap().0);
}
