//! country-core AutoClutterEdit data contract: externally tagged Delete{id:u64}.
//! Live deletion/history consumers reuse this type through resources::history.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AutoClutterEdit {
    Delete { id: u64 },
}

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AutoClutterReplayCmd {
    pub edit: Arc<AutoClutterEdit>,
    pub timed: bool,
    pub duration: f32,
    pub elapsed: f32,
    pub undo: bool,
}
impl AutoClutterReplayCmd {
    /// B tag-13 expansion at 0x14143fc1e: command tag 9, duration bits
    /// 0x3dcccccd, elapsed zero, timed=!instant, and the shared edit Arc.
    pub fn expanded_shared(edit: Arc<AutoClutterEdit>, undo: bool, instant: bool) -> Self {
        Self {
            edit,
            timed: !instant,
            duration: 0.1,
            elapsed: 0.0,
            undo,
        }
    }
    pub fn make_instant(&mut self) {
        self.timed = false;
    }
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
