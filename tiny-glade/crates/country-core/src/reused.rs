//! Original hotwatch fork adapter; event mapping is game-facing glue.
pub mod watch {
    use std::path::Path;
    pub struct GenericHotwatch(std::sync::Mutex<hotwatch::Hotwatch>);
    impl GenericHotwatch {
        pub fn new_with_custom_delay(delay: core::time::Duration) -> Result<Self, hotwatch::Error> {
            hotwatch::Hotwatch::new_with_custom_delay(delay).map(|w| Self(std::sync::Mutex::new(w)))
        }
        pub fn watch<P: AsRef<Path>, F: FnMut(hotwatch::Event) + Send + 'static>(
            &self,
            path: P,
            cb: F,
        ) -> Result<(), hotwatch::Error> {
            self.0
                .lock()
                .expect("watcher lock poisoned")
                .watch(path.as_ref(), cb)
        }
    }
}
