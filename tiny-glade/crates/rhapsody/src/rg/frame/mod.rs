//! CPU frame diagnostics. GPU completion and presentation ownership are tracked
//! by FrameTransaction and the production FrameLoop, rather than a shadow sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameLedgerError {
    /// A frame can be finalized only once.
    EndedTwice,
    /// No successful GPU submission was observed.
    NoSubmission,
}
#[derive(Clone, Debug, Default)]
pub struct FrameLedger {
    submissions: u32,
    ended: bool,
}
impl FrameLedger {
    /// `begin_frame`：`+0x108 = 2`、计数清零。
    pub fn begin(&mut self) {
        *self = Self::default();
    }
    /// `submit_pending_cbs` 成功提交一次后写回回执。
    pub fn note_submission(&mut self) {
        self.submissions += 1;
    }
    pub fn submissions(&self) -> u32 {
        self.submissions
    }
    pub fn has_ended(&self) -> bool {
        self.ended
    }
    /// `end_frame`：先查"已结束"，再查"是否有过提交"。
    pub fn end(&mut self) -> Result<(), FrameLedgerError> {
        if std::mem::replace(&mut self.ended, true) {
            return Err(FrameLedgerError::EndedTwice);
        }
        if self.submissions == 0 {
            return Err(FrameLedgerError::NoSubmission);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
