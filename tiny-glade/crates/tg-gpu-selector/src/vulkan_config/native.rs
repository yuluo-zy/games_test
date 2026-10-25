use super::*;
impl tg_iced::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::Length;
        use tg_iced::iced::widget::{button, checkbox, column, container, scrollable};
        use tg_iced::native_text as text;
        let mut body=column![text("Your system has the following Vulkan layers available"),text("Select which ones should be active in Tiny Glade. Some might cause crashes or bugs.")].spacing(10);
        for group in self.allowlist_view() {
            let vendor = &group.vendor;
            let layers = &group.layers;
            if let Some(vendor) = vendor {
                let vendor_name = vendor.clone();
                let selected = self
                    .view()
                    .allowlist
                    .items
                    .iter()
                    .any(|item| matches!(item, AllowlistItem::Vendor(name) if name == vendor));
                body = body.push(
                    checkbox(vendor_name.clone(), selected)
                        .on_toggle(move |_| Message::ToggleVendor(vendor_name.clone())),
                );
            }
            for layer in layers {
                let name = layer.name.clone();
                let selected = self.view().allowlist.items.iter().any(|item| {
                    matches!(item, AllowlistItem::Layer(enabled_name) if enabled_name == &name)
                });
                body = body
                    .push(
                        checkbox(name.clone(), selected)
                            .on_toggle(move |_| Message::ToggleLayer(name.clone())),
                    )
                    .push(text(&layer.description).size(14));
            }
        }
        body=body.push(text("Tiny Glade rendering workarounds:").size(20)).push(text("Only enable if experiencing graphics-related crashes. Some may have a performance cost."));
        for workaround in Workaround::ALL {
            body = body.push(
                checkbox(workaround.label(), self.view().workarounds.get(workaround))
                    .on_toggle(move |_| Message::ToggleWorkaround(workaround)),
            );
        }
        let actions = column![
            button(text(tg_iced::text("Ok")))
                .width(Length::Fill)
                .on_press(Message::Apply),
            button(text(tg_iced::text("Cancel")))
                .width(Length::Fill)
                .on_press(Message::Cancel),
        ]
        .spacing(12);
        body = body.push(container(actions).width(Length::Fill).max_width(360));
        container(scrollable(
            container(body.padding(16))
                .width(Length::Fill)
                .max_width(960),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
