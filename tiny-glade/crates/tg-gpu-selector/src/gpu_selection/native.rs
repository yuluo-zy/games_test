use super::*;
use tg_iced::gui_runner::{Command, NativeAction, ProgramWithOutcome};
impl tg_iced::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::Length;
        use tg_iced::iced::widget::{button, column, container, scrollable};
        use tg_iced::native_text as text;
        let mut cards = column!().spacing(20);
        for (index, device) in self.view().iter().enumerate() {
            let advice = device_driver_advice(
                device.vendor,
                &device.driver_version,
                &device.driver_support,
            );
            let mut card = column![
                text(&device.name).size(20),
                text(format!(
                    "{}: {}",
                    tg_iced::text("Driver.DriverVersion"),
                    advice.version
                )),
                text(tg_iced::text(advice.key))
            ]
            .spacing(8);
            if let Some(requirement) = advice.requirement {
                let detail = match requirement {
                    DriverRequirement::MinimumVersion(version) => format!(
                        "{} {version}",
                        tg_iced::text("Driver.MinimumSupportedVersionIs")
                    ),
                    DriverRequirement::KnownGoodVersion(version) => {
                        format!("Known good driver version: {version}")
                    }
                    DriverRequirement::VendorId(id) => format!("Vendor ID: 0x{id:x}"),
                };
                card = card.push(text(detail));
            }
            if self.more_info() {
                for link in advice.links {
                    card = card
                        .push(button(text(link.label)).on_press(Message::OpenUrl(link.url.into())));
                }
            }
            if let Some(error) = &device.init_error {
                let error_view = localize_graphics_init_error(error);
                card = card.push(text(tg_iced::text(error_view.heading)));
                if let Some(details) = error_view.details {
                    card = card.push(text(details));
                }
                if let Some(key) = error_view.recommendation {
                    card = card.push(text(tg_iced::text(key)));
                }
            }
            let select = button(text(tg_iced::text(
                if matches!(device.driver_support, DriverSupport::Supported) {
                    "Play"
                } else {
                    "Driver.IgnoreAndPlay"
                },
            )))
            .on_press_maybe(
                device
                    .is_selectable()
                    .then_some(Message::Select(DeviceId::new(index))),
            );
            card = card.push(select);
            cards = cards.push(container(card).padding(15).width(Length::Fill));
        }
        let actions = column![
            button(text(tg_iced::text("Crash.ShowDetails")))
                .width(Length::Fill)
                .on_press(Message::ShowMoreInfo),
            button(text(tg_iced::text("Driver.ShowLogFiles")))
                .width(Length::Fill)
                .on_press(Message::OpenLog),
            button(text(tg_iced::text("Menu.Quit")))
                .width(Length::Fill)
                .on_press(Message::Close),
        ]
        .spacing(12);
        let body = column![
            text(tg_iced::text(self.heading())).size(22),
            cards,
            container(actions).width(Length::Fill).max_width(360),
        ]
        .spacing(20)
        .padding(16);
        container(scrollable(
            container(body).width(Length::Fill).max_width(960),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn update_native(
        &mut self,
        message: Message,
    ) -> Result<NativeAction<Message>, tg_iced::gui_runner::GuiError> {
        match self.update(message) {
            Command::OpenUrl(url) => {
                webbrowser::open(&url)
                    .map_err(|error| tg_iced::gui_runner::GuiError::Backend(error.to_string()))?;
                Ok(NativeAction::Continue(tg_iced::iced::Command::none()))
            }
            Command::OpenPath(path) => {
                open::that(path)
                    .map_err(|error| tg_iced::gui_runner::GuiError::Backend(error.to_string()))?;
                Ok(NativeAction::Continue(tg_iced::iced::Command::none()))
            }
            other => tg_iced::gui_runner::platform_command(other),
        }
    }
}
