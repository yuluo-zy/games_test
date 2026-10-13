// ---------------------------------------------------------------------------
// hotwatch / notify — file-watch surface.
// ---------------------------------------------------------------------------
pub mod watch {
    use std::path::Path;

    /// `hotwatch::GenericHotwatch` (reused). `new_with_custom_delay` / `watch`.
    pub struct GenericHotwatch(pub crate::Gap);

    impl GenericHotwatch {
        pub fn new_with_custom_delay(_delay: core::time::Duration) -> Result<Self, ()> {
            Ok(GenericHotwatch(crate::Gap))
        }
        pub fn watch<P: AsRef<Path>, F: FnMut(std::path::PathBuf) + Send + 'static>(
            &self,
            _path: P,
            _cb: F,
        ) -> Result<(), ()> {
            Ok(())
        }
    }
    /// `notify` watcher backend tag used by hotwatch.
    pub type ReadDirectoryChangesWatcher = ();
}
