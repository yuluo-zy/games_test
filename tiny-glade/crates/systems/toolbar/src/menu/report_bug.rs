//! B `toolbar::menu::report_bug` CPU surface (T13 capture excluded).
//!
//! Recovered here: `try_remove_file` (0x141e1e5f0, O). The report form, text
//! editing and GPU screenshot attachment remain U / GPU-postponed per the TB
//! contract §4.3 and AGENTS.md T13 exclusion.

/// O 0x141e1e5f0 `toolbar::menu::report_bug::try_remove_file`.
///
/// Checks `is_file` and removes the path; a removal failure is logged as an
/// `io::Error` instead of propagating. U: the native log level/target string
/// is not recovered.
pub fn try_remove_file(path: &std::path::Path) {
    if path.is_file() {
        if let Err(error) = std::fs::remove_file(path) {
            log::error!(
                "failed to remove temporary feedback file {}: {error}",
                path.display()
            );
        }
    }
}

/// Native `TextEditMemory`: a futex-backed editor owner shared through an Arc.
/// U: the 0x198-byte `cosmic_text::Editor` copy (`replace_editor`) is not
/// modelled because `cosmic-text` is not a toolbar dependency in the recovered
/// crate graph; the shared editor state below is the safe source-level owner.
pub struct TextEditMemory {
    pub state: std::sync::Mutex<TextEditMemoryState>,
}

/// State byte 4 is the native "initialized" sentinel observed in
/// `impl$11::text_edit` (0x140fa0bd0): `state == 4` selects set_metrics/set_text
/// instead of replace_editor.
pub struct TextEditMemoryState {
    pub status: u8,
    pub text: String,
}

impl Default for TextEditMemory {
    fn default() -> Self {
        Self {
            state: std::sync::Mutex::new(TextEditMemoryState {
                status: 4,
                text: String::new(),
            }),
        }
    }
}

/// B `TextEditContent` (ANALYSIS C10): `+0x268` focused, `+0x270` initialized,
/// `+600` `Arc<TextEditMemory>`. Input gating methods are recovered here.
pub struct TextEditContent {
    pub focused: bool,
    pub initialized: bool,
    pub memory: std::sync::Arc<TextEditMemory>,
    /// Native gating byte read by `input_allowed` (U: exact offset inside the
    /// editor memory; the predicate is O).
    pub input_allowed: bool,
}

impl crate::mei::widgets::DefaultNodeContent for TextEditContent {
    fn from_font_table(_: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self {
            focused: false,
            initialized: false,
            memory: std::sync::Arc::new(TextEditMemory::default()),
            input_allowed: true,
        }
    }
}

impl crate::mei::core::NodeContent for TextEditContent {
    fn type_name(&self) -> &'static str {
        "toolbar::menu::report_bug::TextEditContent"
    }
    fn interactive(&self) -> bool {
        true
    }
    /// Native `impl$11::text_edit` (0x140fa0bd0) initializes the shared editor
    /// when `+0x270 == 0` and `memory.status == 4`, else replaces the editor.
    /// The editor copy is U; the shared-status handshake is kept.
    fn prepare_layout(
        &mut self,
        _ctx: &mut crate::mei::core::NodePrepareContext<'_>,
    ) -> Option<[crate::mei::layout::LengthPercentage; 2]> {
        if !self.initialized {
            if let Ok(state) = self.memory.state.lock() {
                self.initialized = state.status == 4;
            }
        }
        None
    }
}

impl TextEditContent {
    /// O `TextEditView::input_allowed` (report_bug.rs:500): submit/input gate.
    pub fn input_allowed(&self) -> bool {
        self.input_allowed
    }

    /// O `TextEditView::disable_input` (report_bug.rs:504): clears the gate
    /// while the form is submitting.
    pub fn disable_input(&mut self) {
        self.input_allowed = false;
    }

    /// Replaces the shared editor text through the real memory owner. U: the
    /// native call is `cosmic_text::Editor::set_text`.
    pub fn set_text(&mut self, text: String) {
        if let Ok(mut state) = self.memory.state.lock() {
            state.text = text;
        }
    }

    pub fn text(&self) -> String {
        self.memory
            .state
            .lock()
            .map(|state| state.text.clone())
            .unwrap_or_default()
    }
}

impl crate::mei::core::NodeContext {
    /// O `impl$11::text_edit<&str>` (0x140fa0bd0): named multi-line text-edit
    /// node with the shared memory Arc. The four lookup layers and click focus
    /// byte (`+0x268`) are represented by `focused`; the editor copy is U.
    pub fn text_edit(
        &self,
        name: &str,
        memory: std::sync::Arc<TextEditMemory>,
    ) -> Self {
        let child = self.child(name);
        let content = child.get_content_mut_impl::<TextEditContent>(false);
        content.memory = memory;
        content.initialized = false;
        child
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O: only an existing file is unlinked; missing paths are a no-op.
    #[test]
    fn removes_existing_file_and_ignores_missing_path() {
        let dir = std::env::temp_dir().join(format!(
            "tb-report-bug-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("shot.png");
        std::fs::write(&file, b"x").unwrap();
        assert!(file.is_file());
        try_remove_file(&file);
        assert!(!file.exists());
        // Missing path: no panic, no file created.
        let missing = dir.join("absent.png");
        try_remove_file(&missing);
        assert!(!missing.exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn text_edit_input_gate_and_shared_memory_are_real() {
        let mut content = <TextEditContent as crate::mei::widgets::DefaultNodeContent>::from_font_table(
            std::sync::Arc::new(tg_text::FontTable::default()),
        );
        assert!(content.input_allowed());
        content.disable_input();
        assert!(!content.input_allowed());
        content.set_text("bug report".to_owned());
        assert_eq!(content.text(), "bug report");
        let shared = content.memory.clone();
        content.set_text("second".to_owned());
        assert_eq!(
            shared.state.lock().unwrap().text,
            "second",
            "the Arc is the single editor owner"
        );
    }
}
