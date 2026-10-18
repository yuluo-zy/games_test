//! Scrollable, width-constrained native presentation of the dialog policy.
use super::{Controls, Message, MessageDialog};
use crate::gui_runner::{GuiError, Options};

impl crate::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> iced::Element<'static, Message> {
        use crate::native_text as text;
        use iced::widget::{button, column, container, scrollable};
        use iced::{Alignment, Length};
        let view = self.view();
        // Stacked, fill-width buttons also accommodate narrow windows and long translations.
        let buttons = view
            .buttons
            .iter()
            .fold(column!().spacing(12), |column, &(key, message)| {
                column.push(
                    button(text(crate::text(key)))
                        .padding(10)
                        .width(Length::Fill)
                        .on_press(message),
                )
            });
        let content = column![
            text(view.icon).size(50),
            text(view.title.to_owned()).size(20),
            text(view.description.to_owned()),
            container(buttons).width(Length::Fill).max_width(360),
        ]
        .spacing(20)
        .padding(16)
        .align_items(Alignment::Center);
        container(
            scrollable(container(content).width(Length::Fill).max_width(720)).height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
impl MessageDialog {
    /// Show a native dialog on the main thread; locale setup is the caller's responsibility.
    /// Returns confirmation, cancellation, or `None` for a close without a choice.
    /// The backend permits one native event loop per process.
    ///
    /// # Errors
    /// Propagates native initialization, execution, and command failures.
    pub fn show_native(self) -> Result<Option<bool>, GuiError> {
        let options = Options::new(self.title.clone(), self.width, self.height);
        crate::gui_runner::run_native_gui(Controls::new(self), options)
    }
}
