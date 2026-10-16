use super::*;
use crate::{
    recovered_building::{
        history_transactions::{HistoryEntry, ObservedHistoryLedger},
        roof_customization::FlatRoofRimRequest,
        wall_state::WallId as ObservedWallId,
    },
    resources::{
        history::EditType,
        roofs::{Roof, commands::*},
        time::GameTime,
        walls::{WallId, public_walls::PublicWalls},
    },
};

fn empty_history() -> History {
    History::from_observed(ObservedHistoryLedger {
        entries: Vec::new(),
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    })
}

#[test]
fn unresolved_auxiliary_owner_does_not_advance_cursor() {
    let mut entry = HistoryEntry::<EditType>::from_edits(Vec::new());
    entry.observed_auxiliary_counts = [1, 0, 0];
    let mut history: History = History::from_observed(ObservedHistoryLedger {
        entries: vec![entry],
        cursor: 0,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    });
    assert!(
        history
            .cursor_step_expanding(HistoryCursorStep::Undo, false, ReplayBucket::expand)
            .is_err()
    );
    assert_eq!(history.ledger().cursor, 0);
}


