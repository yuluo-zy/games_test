//! B Vulkan layer allowlist editing; six explicit workaround switches.
use tg_iced::gui_runner::{Command, ProgramWithOutcome};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowlistItem {
    Layer(String),
    Vendor(String),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allowlist {
    pub items: Vec<AllowlistItem>,
    /// Original offset 0x30 opaque u32; do not assign a guessed schema meaning.
    pub policy_word: u32,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Workarounds {
    pub sync_before_each_pass: bool,
    pub sync_before_each_draw: bool,
    pub sync_before_each_dispatch: bool,
    pub serial_recording: bool,
    pub skip_loading_screen: bool,
    pub vendor_crash_breadcrumbs: bool,
}
impl Workarounds {
    pub fn get(&self, index: usize) -> Option<bool> {
        match index {
            0 => Some(self.sync_before_each_pass),
            1 => Some(self.sync_before_each_draw),
            2 => Some(self.sync_before_each_dispatch),
            3 => Some(self.serial_recording),
            4 => Some(self.skip_loading_screen),
            5 => Some(self.vendor_crash_breadcrumbs),
            _ => None,
        }
    }
    fn toggle(&mut self, index: usize) {
        let p = match index {
            0 => &mut self.sync_before_each_pass,
            1 => &mut self.sync_before_each_draw,
            2 => &mut self.sync_before_each_dispatch,
            3 => &mut self.serial_recording,
            4 => &mut self.skip_loading_screen,
            5 => &mut self.vendor_crash_breadcrumbs,
            _ => return,
        };
        *p = !*p;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub name: String,
    pub vendor: Option<String>,
    pub description: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    pub allowlist: Allowlist,
    pub workarounds: Workarounds,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    ToggleLayer(String),
    ToggleVendor(String),
    Apply,
    Cancel,
    ToggleWorkaround(usize),
}
#[derive(Debug, Clone)]
pub struct Controls {
    pub groups: Vec<(Option<String>, Vec<Layer>)>,
    pub configuration: Configuration,
    outcome: Option<Configuration>,
}
impl Controls {
    pub fn new(layers: Vec<Layer>, configuration: Configuration) -> Self {
        let mut groups: Vec<(Option<String>, Vec<Layer>)> = vec![];
        for layer in layers {
            if let Some((_, group)) = groups
                .iter_mut()
                .find(|(vendor, _)| *vendor == layer.vendor)
            {
                group.push(layer);
            } else {
                groups.push((layer.vendor.clone(), vec![layer]));
            }
        }
        Self {
            groups,
            configuration,
            outcome: None,
        }
    }
    pub fn toggle_allowlist_item(&mut self, item: AllowlistItem) {
        let mut removed = false;
        self.configuration.allowlist.items.retain(|x| {
            if *x == item {
                removed = true;
                false
            } else {
                true
            }
        });
        if !removed {
            self.configuration.allowlist.items.push(item);
        }
    }
    pub fn allowlist_view(&self) -> &[(Option<String>, Vec<Layer>)] {
        &self.groups
    }
    pub fn workarounds_view(&self) -> [(&'static str, bool); 6] {
        let w = &self.configuration.workarounds;
        [
            (
                "Sync before each render graph pass",
                w.sync_before_each_pass,
            ),
            ("Sync before each draw", w.sync_before_each_draw),
            (
                "Sync before each compute shader dispatch",
                w.sync_before_each_dispatch,
            ),
            ("Serial command buffer recording", w.serial_recording),
            ("Skip loading screen", w.skip_loading_screen),
            (
                "Use vendor-specific crash breadcrumbs (NVIDIA Aftermath)",
                w.vendor_crash_breadcrumbs,
            ),
        ]
    }
    pub fn view(&self) -> &Configuration {
        &self.configuration
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = Configuration;
    fn update(&mut self, m: Message) -> Command {
        match m {
            Message::ToggleLayer(s) => self.toggle_allowlist_item(AllowlistItem::Layer(s)),
            Message::ToggleVendor(s) => self.toggle_allowlist_item(AllowlistItem::Vendor(s)),
            Message::Apply => self.outcome = Some(self.configuration.clone()),
            Message::Cancel => return Command::CloseWindow,
            Message::ToggleWorkaround(i) => self.configuration.workarounds.toggle(i),
        }
        Command::None
    }
    fn program_outcome(&self) -> Option<Configuration> {
        self.outcome.clone()
    }
}

#[cfg(feature = "native-gui")]
impl tg_iced::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::Length;
        use tg_iced::iced::widget::{button, checkbox, column, container, row, scrollable, text};
        let mut body=column![text("Your system has the following Vulkan layers available"),text("Select which ones should be active in Tiny Glade. Some might cause crashes or bugs.")].spacing(10);
        for (vendor, layers) in &self.groups {
            if let Some(vendor) = vendor {
                let v = vendor.clone();
                let selected = self
                    .configuration
                    .allowlist
                    .items
                    .contains(&AllowlistItem::Vendor(v.clone()));
                body = body.push(
                    checkbox(v.clone(), selected)
                        .on_toggle(move |_| Message::ToggleVendor(v.clone())),
                );
            }
            for layer in layers {
                let name = layer.name.clone();
                let selected = self
                    .configuration
                    .allowlist
                    .items
                    .contains(&AllowlistItem::Layer(name.clone()));
                body = body
                    .push(
                        checkbox(name.clone(), selected)
                            .on_toggle(move |_| Message::ToggleLayer(name.clone())),
                    )
                    .push(text(layer.description.clone()).size(14));
            }
        }
        body=body.push(text("Tiny Glade rendering workarounds:").size(20)).push(text("Only enable if experiencing graphics-related crashes. Some may have a performance cost."));
        for (index, (label, enabled)) in self.workarounds_view().into_iter().enumerate() {
            body = body.push(
                checkbox(label, enabled).on_toggle(move |_| Message::ToggleWorkaround(index)),
            );
        }
        container(
            column![
                scrollable(body).height(Length::Fill),
                row![
                    button(text(tg_iced::text("Ok").to_owned())).on_press(Message::Apply),
                    button(text(tg_iced::text("Cancel").to_owned())).on_press(Message::Cancel)
                ]
                .spacing(20)
            ]
            .spacing(20)
            .padding(30),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_groups_keep_first_seen_vendor_order_and_none_group() {
        let controls = Controls::new(
            vec![
                Layer {
                    name: "layer-a".into(),
                    vendor: Some("Vendor A".into()),
                    description: "a".into(),
                },
                Layer {
                    name: "layer-unowned".into(),
                    vendor: None,
                    description: "none".into(),
                },
                Layer {
                    name: "layer-b".into(),
                    vendor: Some("Vendor B".into()),
                    description: "b".into(),
                },
                Layer {
                    name: "layer-a2".into(),
                    vendor: Some("Vendor A".into()),
                    description: "a2".into(),
                },
                Layer {
                    name: "layer-unowned2".into(),
                    vendor: None,
                    description: "none2".into(),
                },
            ],
            Configuration {
                allowlist: Allowlist {
                    items: vec![],
                    policy_word: 0,
                },
                workarounds: Workarounds::default(),
            },
        );

        assert_eq!(
            controls
                .groups
                .iter()
                .map(|(vendor, layers)| (vendor.as_deref(), layers.len()))
                .collect::<Vec<_>>(),
            [(Some("Vendor A"), 2), (None, 2), (Some("Vendor B"), 1)]
        );
        assert_eq!(controls.groups[0].1[0].name, "layer-a");
        assert_eq!(controls.groups[0].1[1].name, "layer-a2");
    }

    #[test]
    fn allowlist_toggle_removes_all_equal_tagged_entries_only() {
        let mut controls = Controls::new(
            vec![],
            Configuration {
                allowlist: Allowlist {
                    items: vec![
                        AllowlistItem::Layer("same".into()),
                        AllowlistItem::Vendor("same".into()),
                        AllowlistItem::Layer("same".into()),
                    ],
                    policy_word: 0x1234,
                },
                workarounds: Workarounds::default(),
            },
        );
        controls.toggle_allowlist_item(AllowlistItem::Layer("same".into()));
        assert_eq!(
            controls.configuration.allowlist.items,
            [AllowlistItem::Vendor("same".into())]
        );
        assert_eq!(controls.configuration.allowlist.policy_word, 0x1234);

        controls.toggle_allowlist_item(AllowlistItem::Layer("same".into()));
        assert_eq!(
            controls.configuration.allowlist.items,
            [
                AllowlistItem::Vendor("same".into()),
                AllowlistItem::Layer("same".into())
            ]
        );
    }

    #[test]
    fn workaround_labels_and_index_order_are_stable() {
        let controls = Controls::new(
            vec![],
            Configuration {
                allowlist: Allowlist {
                    items: vec![],
                    policy_word: 0,
                },
                workarounds: Workarounds::default(),
            },
        );
        assert_eq!(
            controls.workarounds_view().map(|(label, _)| label),
            [
                "Sync before each render graph pass",
                "Sync before each draw",
                "Sync before each compute shader dispatch",
                "Serial command buffer recording",
                "Skip loading screen",
                "Use vendor-specific crash breadcrumbs (NVIDIA Aftermath)",
            ]
        );
        assert_eq!(
            (0..6)
                .map(|i| controls.configuration.workarounds.get(i))
                .collect::<Vec<_>>(),
            [Some(false); 6]
        );
        assert_eq!(controls.configuration.workarounds.get(6), None);
    }

    #[test]
    fn cancel_closes_and_invalid_workaround_indices_are_ignored() {
        let mut controls = Controls::new(
            vec![],
            Configuration {
                allowlist: Allowlist {
                    items: vec![],
                    policy_word: 0,
                },
                workarounds: Workarounds::default(),
            },
        );
        assert_eq!(controls.update(Message::Cancel), Command::CloseWindow);
        controls.update(Message::ToggleWorkaround(6));
        assert_eq!(
            (0..6)
                .map(|i| controls.configuration.workarounds.get(i))
                .collect::<Vec<_>>(),
            [Some(false); 6]
        );
    }

    #[test]
    fn apply_snapshots_and_duplicate_toggle_does_not_flip_other_variant() {
        let mut c = Controls::new(
            vec![],
            Configuration {
                allowlist: Allowlist {
                    items: vec![
                        AllowlistItem::Layer("x".into()),
                        AllowlistItem::Layer("x".into()),
                        AllowlistItem::Vendor("x".into()),
                    ],
                    policy_word: 123,
                },
                workarounds: Workarounds::default(),
            },
        );
        c.update(Message::ToggleLayer("x".into()));
        c.update(Message::ToggleWorkaround(5));
        c.update(Message::Apply);
        let o = c.program_outcome().unwrap();
        c.update(Message::ToggleWorkaround(5));
        assert!(o.workarounds.vendor_crash_breadcrumbs);
        assert_eq!(o.allowlist.policy_word, 123);
        assert_eq!(o.allowlist.items, [AllowlistItem::Vendor("x".into())]);
    }
}
