//! Localized dialogs, fallible native window adaptation and static screenshots.
#![doc = include_str!("../README.md")]
pub mod dialog;
pub mod gui_runner;
/// Look up a display string in the configured game locale, retaining shared text ownership.
/// Missing translations follow `lang_lib`'s fallback policy.
pub fn text(key: &str) -> std::sync::Arc<str> {
    lang_lib::lookup(key)
}

#[cfg(feature = "native-gui")]
pub use iced;

/// Load the last selected locale for a standalone GUI session.
/// The caller must select the asset working directory before calling this function.
///
/// # Errors
/// Returns `GuiError::Backend` with the locale service's setup or loading error.
#[cfg(feature = "native-gui")]
pub fn load_gui_locale() -> Result<(), gui_runner::GuiError> {
    lang_lib::load_last_used_locale()
        .map_err(|error| gui_runner::GuiError::Backend(error.to_string()))
}

/// Create owned iced text with advanced shaping for translated strings and icons.
/// This sets the shaping mode; font registration remains the application's responsibility.
#[cfg(feature = "native-gui")]
pub fn native_text(content: impl ToString) -> iced::widget::Text<'static> {
    iced::widget::text(content).shaping(iced::widget::text::Shaping::Advanced)
}
