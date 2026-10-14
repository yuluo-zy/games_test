//! MessageDialog hand-written builder and button policy, B 0x140631ce0 / 0x1406331a0.
use crate::gui_runner::{Command, GuiBackend, GuiError, Options, ProgramWithOutcome, run_gui};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}
impl MessageLevel {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Info => "🐑",
            Self::Warning => "⚠️",
            Self::Error => "❌",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageButtons {
    Ok,
    OkCancel,
    Continue,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageDialog {
    pub title: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub level: MessageLevel,
    pub buttons: MessageButtons,
}
impl MessageDialog {
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
    pub fn set_level(mut self, level: MessageLevel) -> Self {
        self.level = level;
        self
    }
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    pub fn set_buttons(mut self, buttons: MessageButtons) -> Self {
        self.buttons = buttons;
        self
    }
    pub fn show(self, backend: &mut impl GuiBackend<Controls>) -> Result<Option<bool>, GuiError> {
        let options = Options::new(self.title.clone(), self.width, self.height);
        run_gui(Controls::new(self), options, backend)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Confirm,
    Cancel,
}
#[derive(Debug, Clone)]
pub struct Controls {
    pub dialog: MessageDialog,
    outcome: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogView<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub icon: &'static str,
    pub buttons: Vec<(&'static str, Message)>,
}
impl Controls {
    pub fn new(dialog: MessageDialog) -> Self {
        Self {
            dialog,
            outcome: None,
        }
    }
    pub fn view(&self) -> DialogView<'_> {
        let buttons = match self.dialog.buttons {
            MessageButtons::Ok => vec![("Ok", Message::Confirm)],
            MessageButtons::OkCancel => vec![("Ok", Message::Confirm), ("Cancel", Message::Cancel)],
            MessageButtons::Continue => vec![
                ("Continue.Yep", Message::Confirm),
                ("Continue.Nope", Message::Cancel),
            ],
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
        self.outcome = Some(message == Message::Confirm);
        Command::CloseWindow
    }
    fn program_outcome(&self) -> Option<bool> {
        self.outcome
    }
}

#[cfg(feature = "native-gui")]
impl crate::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> iced::Element<'static, Message> {
        use crate::native_text as text;
        use iced::widget::{button, column, container, row, scrollable};
        use iced::{Alignment, Length};
        let view = self.view();
        let buttons = view
            .buttons
            .into_iter()
            .fold(row!().spacing(20), |r, (key, message)| {
                r.push(
                    button(text(crate::text(key).to_string()))
                        .padding(10)
                        .width(150)
                        .on_press(message),
                )
            });
        container(
            column![
                text(view.icon).size(50),
                text(view.title.to_owned()).size(20),
                scrollable(text(view.description.to_owned())).height(Length::Fill),
                buttons
            ]
            .spacing(20)
            .padding(50)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
#[cfg(feature = "native-gui")]
impl MessageDialog {
    pub fn show_native(self) -> Result<Option<bool>, GuiError> {
        let options = Options::new(self.title.clone(), self.width, self.height);
        crate::gui_runner::run_native_gui(Controls::new(self), options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    include!("fixtures/dialog_cases.rs");
    #[test]
    fn original_rne_new_and_set_level_contract() {
        for &(width, height, level) in ORIGINAL_DIALOG_CASES {
            let level = match level {
                0 => MessageLevel::Info,
                1 => MessageLevel::Warning,
                2 => MessageLevel::Error,
                _ => unreachable!(),
            };
            let d = MessageDialog::new(width, height).set_level(level);
            assert_eq!((d.width, d.height, d.level), (width, height, level));
            assert!(d.title.is_empty() && d.description.is_empty());
            assert_eq!(d.buttons, MessageButtons::Ok);
        }
    }
}
