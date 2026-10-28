use super::*;
#[test]
fn ledger_requires_a_submission_and_rejects_double_end() {
    let mut ledger = FrameLedger::default();
    ledger.begin();
    assert_eq!(ledger.end(), Err(FrameLedgerError::NoSubmission));
    assert_eq!(ledger.end(), Err(FrameLedgerError::EndedTwice));
    ledger.begin();
    ledger.note_submission();
    assert_eq!(ledger.end(), Ok(()));
    assert_eq!(ledger.end(), Err(FrameLedgerError::EndedTwice));
}
