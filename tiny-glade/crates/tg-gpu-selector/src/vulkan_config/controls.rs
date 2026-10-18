//! Draft configuration editing, preserving input order and freezing on Apply/Cancel.
use super::{AllowlistItem, Configuration, Layer, LayerGroup, Workaround};
use std::collections::HashMap;
use tg_iced::gui_runner::{Command, ProgramWithOutcome};
#[derive(Debug, Clone, PartialEq, Eq)]
/// Draft edits and terminal Apply/Cancel requests. Terminal controls ignore all further edits.
pub enum Message {
    ToggleLayer(String),
    ToggleVendor(String),
    Apply,
    Cancel,
    ToggleWorkaround(Workaround),
}
#[derive(Debug, Clone)]
/// A configuration draft and available-layer presentation for one editing session.
/// Apply freezes a result snapshot; Cancel freezes the draft without producing a result.
pub struct Controls {
    groups: Vec<LayerGroup>,
    configuration: Configuration,
    outcome: Option<Configuration>,
    closed: bool,
}
impl Controls {
    /// Group supplied layers in expected linear time while preserving first-seen vendor and layer order.
    /// Existing allowlist entries and opaque policy data are preserved, even if absent from the layer list.
    pub fn new(layers: Vec<Layer>, configuration: Configuration) -> Self {
        let mut groups: Vec<LayerGroup> = Vec::new();
        let mut group_indices = HashMap::<Option<String>, usize>::new();
        for layer in layers {
            let group_index = if let Some(&group_index) = group_indices.get(&layer.vendor) {
                group_index
            } else {
                let group_index = groups.len();
                group_indices.insert(layer.vendor.clone(), group_index);
                groups.push(LayerGroup {
                    vendor: layer.vendor.clone(),
                    layers: Vec::new(),
                });
                group_index
            };
            groups[group_index].layers.push(layer);
        }
        Self {
            groups,
            configuration,
            outcome: None,
            closed: false,
        }
    }
    /// Remove every equal tagged entry, or append it if absent. Does nothing after Apply/Cancel.
    pub fn toggle_allowlist_item(&mut self, item: AllowlistItem) {
        if self.closed {
            return;
        }
        let mut removed = false;
        self.configuration.allowlist.items.retain(|existing| {
            if *existing == item {
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
    /// Borrow available-layer groups; these describe availability, not the enabled allowlist.
    pub fn allowlist_view(&self) -> &[LayerGroup] {
        &self.groups
    }
    /// Read all switches in the order defined by `Workaround::ALL`.
    pub fn workarounds_view(&self) -> [(Workaround, bool); 6] {
        Workaround::ALL
            .map(|workaround| (workaround, self.configuration.workarounds.get(workaround)))
    }
    /// Borrow the current draft, which becomes immutable after Apply/Cancel.
    pub fn view(&self) -> &Configuration {
        &self.configuration
    }
    /// Whether the session has terminated; Cancel yields no configuration outcome.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = Configuration;
    fn update(&mut self, message: Message) -> Command {
        if self.closed {
            return Command::CloseWindow;
        }
        match message {
            Message::ToggleLayer(name) => self.toggle_allowlist_item(AllowlistItem::Layer(name)),
            Message::ToggleVendor(vendor) => {
                self.toggle_allowlist_item(AllowlistItem::Vendor(vendor))
            }
            Message::Apply => {
                self.outcome = Some(self.configuration.clone());
                self.closed = true;
                return Command::CloseWindow;
            }
            Message::Cancel => {
                self.closed = true;
                return Command::CloseWindow;
            }
            Message::ToggleWorkaround(workaround) => {
                self.configuration.workarounds.toggle(workaround)
            }
        }
        Command::None
    }
    fn program_outcome(&self) -> Option<Configuration> {
        self.outcome.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vulkan_config::{Allowlist, Workarounds};
    fn configuration() -> Configuration {
        Configuration {
            allowlist: Allowlist {
                items: vec![
                    AllowlistItem::Layer("same".into()),
                    AllowlistItem::Vendor("same".into()),
                    AllowlistItem::Layer("same".into()),
                ],
                policy_word: 123,
            },
            workarounds: Workarounds::default(),
        }
    }
    #[test]
    fn groups_preserve_first_vendor_and_layer_order_including_unowned_layers() {
        let layers = [
            ("a", Some("A")),
            ("unowned", None),
            ("b", Some("B")),
            ("a2", Some("A")),
            ("unowned2", None),
        ]
        .map(|(name, vendor)| Layer {
            name: name.into(),
            vendor: vendor.map(str::to_owned),
            description: String::new(),
        });
        let controls = Controls::new(layers.into(), configuration());
        let groups = controls.allowlist_view();
        assert_eq!(
            groups
                .iter()
                .map(|group| (group.vendor.as_deref(), group.layers.len()))
                .collect::<Vec<_>>(),
            [(Some("A"), 2), (None, 2), (Some("B"), 1)]
        );
        assert_eq!(
            groups[0]
                .layers
                .iter()
                .map(|layer| layer.name.as_str())
                .collect::<Vec<_>>(),
            ["a", "a2"]
        );
    }
    #[test]
    fn allowlist_tags_deduplicate_independently_and_terminal_messages_preserve_the_snapshot() {
        let mut controls = Controls::new(vec![], configuration());
        controls.update(Message::ToggleLayer("same".into()));
        assert_eq!(
            controls.view().allowlist.items,
            [AllowlistItem::Vendor("same".into())]
        );
        controls.update(Message::ToggleWorkaround(
            Workaround::VendorCrashBreadcrumbs,
        ));
        assert_eq!(controls.update(Message::Apply), Command::CloseWindow);
        let snapshot = controls.program_outcome().unwrap();
        for message in [
            Message::ToggleWorkaround(Workaround::VendorCrashBreadcrumbs),
            Message::ToggleVendor("same".into()),
            Message::Cancel,
            Message::Apply,
        ] {
            assert_eq!(controls.update(message), Command::CloseWindow);
        }
        controls.toggle_allowlist_item(AllowlistItem::Layer("later".into()));
        assert_eq!(controls.view(), &snapshot);
        assert_eq!(controls.program_outcome(), Some(snapshot));
        let mut canceled = Controls::new(vec![], configuration());
        canceled.update(Message::Cancel);
        canceled.update(Message::Apply);
        assert!(canceled.is_closed());
        assert_eq!(canceled.program_outcome(), None);
    }
    #[test]
    fn typed_switches_touch_exactly_the_corresponding_field_and_toggle_back() {
        for selected in Workaround::ALL {
            let mut switches = Workarounds::default();
            switches.toggle(selected);
            for other in Workaround::ALL {
                assert_eq!(switches.get(other), selected == other);
            }
            let explicit_fields = [
                switches.sync_before_each_pass,
                switches.sync_before_each_draw,
                switches.sync_before_each_dispatch,
                switches.serial_recording,
                switches.skip_loading_screen,
                switches.vendor_crash_breadcrumbs,
            ];
            assert_eq!(
                explicit_fields,
                Workaround::ALL.map(|other| other == selected)
            );
            assert!(!selected.label().is_empty());
            switches.toggle(selected);
            assert_eq!(switches, Workarounds::default());
        }
    }
}
