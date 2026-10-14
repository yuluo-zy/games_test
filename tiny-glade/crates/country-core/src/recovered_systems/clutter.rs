//! country-core AutoClutterEdit data contract: externally tagged Delete{id:u64}.
//! History mutation, picking and replay protection consumers remain unconnected.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AutoClutterEdit {
    Delete { id: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_externally_tagged_delete_wire_contract() {
        let wire = serde_json::to_string(&AutoClutterEdit::Delete { id: 5 }).unwrap();
        assert_eq!(wire, r#"{"Delete":{"id":5}}"#);
        assert_eq!(
            serde_json::from_str::<AutoClutterEdit>(r#"{"Delete":{"id":5}}"#).unwrap(),
            AutoClutterEdit::Delete { id: 5 }
        );
    }
}
