//! Localized message dialog configuration and confirmation state.
use crate::gui_runner::{Command, GuiBackend, GuiError, Options, ProgramWithOutcome, run_gui};

/// Presentation severity; it selects the dialog icon without changing choice semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}
impl MessageLevel {
    /// Return the Unicode icon displayed above the dialog text.
    pub fn icon(self) -> &'static str {
        match self {
            Self::Info => "🐑",
            Self::Warning => "⚠️",
            Self::Error => "❌",
        }
    }
}

/// Available localized button groups; each maps to confirmation or cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageButtons {
    Ok,
    OkCancel,
    Continue,
}
/// Owned dialog content and requested window settings.
/// Title and description are final display text; button labels are translated by the renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct MessageDialog {
    pub title: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub level: MessageLevel,
    pub buttons: MessageButtons,
}
impl MessageDialog {
    /// Create an empty informational dialog with an OK button.
    /// Dimensions are logical pixels; the native backend clamps zero to one.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            width,
            height,
            level: MessageLevel::Info,
            buttons: MessageButtons::Ok,
        }
    }
    /// Select the severity icon, retaining the text and buttons.
    pub fn set_level(mut self, level: MessageLevel) -> Self {
        self.level = level;
        self
    }
    /// Set the final display title without performing a localization lookup.
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    /// Set the final display body without performing a localization lookup.
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    /// Select the localized button group.
    pub fn set_buttons(mut self, buttons: MessageButtons) -> Self {
        self.buttons = buttons;
        self
    }
    /// Run an injected backend: `Some(true)` confirms, `Some(false)` cancels,
    /// and `None` closes without a choice.
    ///
    /// # Errors
    /// Propagates backend or command failures before returning a choice.
    pub fn show(self, backend: &mut impl GuiBackend<Controls>) -> Result<Option<bool>, GuiError> {
        let options = Options::new(self.title.clone(), self.width, self.height);
        run_gui(Controls::new(self), options, backend)
    }
}

/// A dialog choice; the first delivered choice becomes the immutable result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Confirm,
    Cancel,
}
/// Dialog state with a one-way transition from no choice to confirmation or cancellation.
#[derive(Debug, Clone)]
pub struct Controls {
    pub dialog: MessageDialog,
    outcome: Option<bool>,
}
/// Borrowed display content and static `(language key, message)` button descriptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogView<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub icon: &'static str,
    pub buttons: &'static [(&'static str, Message)],
}
impl Controls {
    /// Start the dialog with no recorded choice.
    pub fn new(dialog: MessageDialog) -> Self {
        Self {
            dialog,
            outcome: None,
        }
    }
    /// Borrow the text and static button group without allocating or translating labels.
    pub fn view(&self) -> DialogView<'_> {
        let buttons = match self.dialog.buttons {
            MessageButtons::Ok => &[("Ok", Message::Confirm)][..],
            MessageButtons::OkCancel => {
                &[("Ok", Message::Confirm), ("Cancel", Message::Cancel)][..]
            }
            MessageButtons::Continue => &[
                ("Continue.Yep", Message::Confirm),
                ("Continue.Nope", Message::Cancel),
            ][..],
        };
        DialogView {
            title: &self.dialog.title,
            description: &self.dialog.description,
            icon: self.dialog.level.icon(),
            buttons,
        }
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = bool;
    fn update(&mut self, message: Message) -> Command {
        self.outcome.get_or_insert(message == Message::Confirm);
        Command::CloseWindow
    }
    fn program_outcome(&self) -> Option<bool> {
        self.outcome
    }
}

#[cfg(feature = "native-gui")]
mod native;

#[cfg(test)]
mod tests {
    use super::*;
    struct ClosingBackend(Option<Message>);
    impl GuiBackend<Controls> for ClosingBackend {
        fn run(&mut self, controls: &mut Controls, _: &Options) -> Result<(), GuiError> {
            if let Some(message) = self.0 {
                assert_eq!(controls.update(message), Command::CloseWindow);
            }
            Ok(())
        }
    }
    #[test]
    fn choice_and_window_close_remain_distinct() {
        for (message, expected) in [
            (Some(Message::Confirm), Some(true)),
            (Some(Message::Cancel), Some(false)),
            (None, None),
        ] {
            assert_eq!(
                MessageDialog::new(640, 480)
                    .show(&mut ClosingBackend(message))
                    .unwrap(),
                expected
            );
        }
        let mut controls = Controls::new(MessageDialog::new(640, 480));
        controls.update(Message::Confirm);
        controls.update(Message::Cancel);
        assert_eq!(
            controls.program_outcome(),
            Some(true),
            "terminal choice is immutable"
        );
    }
    #[test]
    fn button_groups_retain_localization_keys() {
        let controls =
            Controls::new(MessageDialog::new(640, 480).set_buttons(MessageButtons::Continue));
        assert_eq!(
            controls.view().buttons,
            &[
                ("Continue.Yep", Message::Confirm),
                ("Continue.Nope", Message::Cancel)
            ]
        );
    }
    #[test]
    fn builder_defaults_and_custom_fields() {
        let dialog = MessageDialog::new(640, 480);
        assert!(dialog.title.is_empty() && dialog.description.is_empty());
        assert_eq!((dialog.width, dialog.height), (640, 480));
        assert_eq!(dialog.buttons, MessageButtons::Ok);
        assert_eq!(dialog.level, MessageLevel::Info);
        for level in [
            MessageLevel::Info,
            MessageLevel::Warning,
            MessageLevel::Error,
        ] {
            let dialog = dialog
                .clone()
                .set_level(level)
                .set_title("Title")
                .set_description("Body")
                .set_buttons(MessageButtons::OkCancel);
            assert_eq!(dialog.level, level);
            assert_eq!(dialog.title, "Title");
            assert_eq!(dialog.description, "Body");
            assert_eq!(dialog.buttons, MessageButtons::OkCancel);
        }
    }
}
