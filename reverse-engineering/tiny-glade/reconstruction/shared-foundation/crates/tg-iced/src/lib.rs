//! Recovered B game dialog policy. See the ui evidence manifest for ABI limits.
pub mod dialog;
pub mod gui_runner;
/// Game localization entry: controls keep keys until a renderer asks for text.
pub fn text(key: &str) -> std::sync::Arc<str> {
    lang_lib::lookup(key)
}

#[cfg(feature = "native-gui")]
pub use iced;

/// Read-only locale setup for a standalone GUI consumer; the caller selects its asset cwd.
#[cfg(feature = "native-gui")]
pub fn load_gui_locale() -> Result<(), gui_runner::GuiError> {
    lang_lib::load_last_used_locale().map_err(|e| gui_runner::GuiError::Backend(e.to_string()))
}

/// Native font fallback glue; game strings/icons are preserved unchanged.
#[cfg(feature = "native-gui")]
pub fn native_text(content: impl ToString) -> iced::widget::Text<'static> {
    iced::widget::text(content).shaping(iced::widget::text::Shaping::Advanced)
}
