pub use crate::recovered_systems::clutter::AutoClutterEdit;
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
