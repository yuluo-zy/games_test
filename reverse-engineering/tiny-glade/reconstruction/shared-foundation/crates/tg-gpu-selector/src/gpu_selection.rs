//! B selected-device outcome and driver advice. Driver classification is provided by rhapsody.
use tg_iced::gui_runner::{Command, ProgramWithOutcome};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Nvidia,
    Intel,
    Other,
    Amd,
    Qualcomm,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverLink {
    pub label: &'static str,
    pub url: &'static str,
}
pub fn get_vendor_driver_update_links(vendor: Vendor) -> Vec<DriverLink> {
    match vendor {
        Vendor::Nvidia => vec![DriverLink {
            label: "NVIDIA downloads",
            url: "https://www.nvidia.com/Download/index.aspx",
        }],
        Vendor::Intel => vec![DriverLink {
            label: "Intel Arc, Iris Xe & 11th+ Gen Processor Graphics downloads",
            url: "https://www.intel.com/content/www/us/en/download/785597/intel-arc-iris-xe-graphics-windows.html",
        }],
        Vendor::Amd => vec![DriverLink {
            label: "AMD downloads",
            url: "https://www.amd.com/en/support/download/drivers.html",
        }],
        Vendor::Qualcomm => vec![DriverLink {
            label: "Qualcomm Snapdragon X driver downloads",
            url: "https://softwarecenter.qualcomm.com/catalog/item/Windows_Graphics_Driver",
        }],
        Vendor::Other | Vendor::Unknown => vec![],
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverSupport {
    Unsupported { minimum: String },
    UnsupportedLegacyIntel { minimum: String },
    Emulated,
    UnknownVendor(u32),
    KnownBad { known_good: String },
    Supported,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverAdvice {
    pub version: String,
    pub key: &'static str,
    pub version_requirement: Option<String>,
    pub links: Vec<DriverLink>,
}
pub fn device_driver_advice(
    vendor: Vendor,
    version: &str,
    support: &DriverSupport,
) -> DriverAdvice {
    let mut links = get_vendor_driver_update_links(vendor);
    let (key, version_requirement) = match support {
        DriverSupport::Unsupported { minimum } => {
            ("Driver.UnsupportedDriver", Some(minimum.clone()))
        }
        DriverSupport::UnsupportedLegacyIntel { minimum } => {
            links = vec![DriverLink {
                label: "Intel 7th-10th Gen Processor Graphics downloads",
                url: "https://www.intel.com/content/www/us/en/download/776137/intel-7th-10th-gen-processor-graphics-windows.html",
            }];
            ("Driver.UnsupportedDriver", Some(minimum.clone()))
        }
        DriverSupport::Emulated => {
            links.clear();
            ("Driver.EmulatedDriver", None)
        }
        DriverSupport::UnknownVendor(id) => {
            links.clear();
            ("Driver.UnknownVendor", Some(format!("0x{id:x}")))
        }
        DriverSupport::KnownBad { known_good } => ("Driver.KnownBad", Some(known_good.clone())),
        DriverSupport::Supported => {
            links.clear();
            ("Driver.SupportedDriver", None)
        }
    };
    DriverAdvice {
        version: version.into(),
        key,
        version_requirement,
        links,
    }
}
/// Original raw error tags are preserved until rhapsody's Rust enum ABI is recovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicsInitError {
    pub tag: u64,
    pub details: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicsErrorView {
    pub heading: &'static str,
    pub details: Option<String>,
    pub recommendation: Option<&'static str>,
}
pub fn localize_graphics_init_error(error: &GraphicsInitError) -> GraphicsErrorView {
    let mut recommendation = Some("GraphicsInitError.PleaseUpdateDrivers");
    let mut details = None;
    let heading = match error.tag {
        0x12 => "GraphicsInitError.UnsupportedWindowingSystem",
        0x0b | 0x0c => "GraphicsInitError.NoCompatibleDevices",
        0x14 => {
            details = Some(error.details.clone());
            recommendation = Some("GraphicsInitError.SwapchainErrorTip");
            "GraphicsInitError.SwapchainError"
        }
        0x15 => {
            details = Some(error.details.clone());
            recommendation = Some("GraphicsInitError.OutOfVideoMemoryRecommendation");
            "GraphicsInitError.OutOfVideoMemory"
        }
        0x16 => {
            details = Some(error.details.clone());
            "GraphicsInitError.InternalError"
        }
        0x17 => {
            details = Some(error.details.clone());
            recommendation = None;
            "Unexpected error"
        }
        _ => "GraphicsInitError.NoDriver",
    };
    GraphicsErrorView {
        heading,
        details,
        recommendation,
    }
}
pub fn render_error_to_ui(error: &GraphicsInitError) -> GraphicsErrorView {
    localize_graphics_init_error(error)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub name: String,
    pub vendor: Vendor,
    pub driver_version: String,
    pub driver_support: DriverSupport,
    pub init_error: Option<GraphicsInitError>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Select(usize),
    ShowMoreInfo,
    OpenUrl(String),
    Close,
    OpenLog,
    Noop,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionReason {
    Requested,
    PrimaryUnsupported,
    WhichOne,
    Changed,
    UnavailableDetected,
    GraphicsInitFailed,
}
impl SelectionReason {
    pub fn key(self) -> &'static str {
        match self {
            Self::Requested => "Driver.Header.Reason.Requested",
            Self::PrimaryUnsupported => "Driver.Header.Reason.PrimaryUnsupported",
            Self::WhichOne => "Driver.Header.Reason.WhichOne",
            Self::Changed => "Driver.Header.Reason.Changed",
            Self::UnavailableDetected => "Driver.Header.Reason.UnavailableDetected",
            Self::GraphicsInitFailed => "GraphicsInitError.Prelude",
        }
    }
}
#[derive(Debug, Clone)]
pub struct Controls {
    pub devices: Vec<Device>,
    pub reason: SelectionReason,
    pub more_info: bool,
    pub log_path: std::path::PathBuf,
    outcome: Option<usize>,
}
impl Controls {
    pub fn new(devices: Vec<Device>, log_path: std::path::PathBuf) -> Self {
        Self {
            devices,
            reason: SelectionReason::Requested,
            more_info: false,
            log_path,
            outcome: None,
        }
    }
    pub fn with_reason(mut self, reason: SelectionReason) -> Self {
        self.reason = reason;
        self
    }
    pub fn view(&self) -> &[Device] {
        &self.devices
    }
    pub fn heading(&self) -> &'static str {
        self.reason.key()
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = usize;
    fn update(&mut self, m: Message) -> Command {
        match m {
            Message::Select(i) => {
                self.outcome = Some(i);
                Command::None
            }
            Message::ShowMoreInfo => {
                self.more_info = true;
                Command::None
            }
            Message::OpenUrl(url) => Command::OpenUrl(url),
            Message::Close => Command::CloseWindow,
            Message::OpenLog => Command::OpenPath(self.log_path.clone()),
            Message::Noop => Command::None,
        }
    }
    fn program_outcome(&self) -> Option<usize> {
        self.outcome
    }
}

#[cfg(feature = "native-gui")]
impl tg_iced::gui_runner::NativeProgram for Controls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::Length;
        use tg_iced::iced::widget::{button, column, container, row, scrollable, text};
        let mut cards = column!().spacing(20);
        for (index, device) in self.devices.iter().enumerate() {
            let advice = device_driver_advice(
                device.vendor,
                &device.driver_version,
                &device.driver_support,
            );
            let mut card = column![
                text(device.name.clone()).size(20),
                text(format!(
                    "{}: {}",
                    tg_iced::text("Driver.DriverVersion"),
                    advice.version
                )),
                text(tg_iced::text(advice.key).to_owned())
            ]
            .spacing(8);
            if let Some(version) = advice.version_requirement {
                card = card.push(text(format!(
                    "{} {version}",
                    tg_iced::text("Driver.MinimumSupportedVersionIs")
                )));
            }
            if self.more_info {
                for link in advice.links {
                    card = card
                        .push(button(text(link.label)).on_press(Message::OpenUrl(link.url.into())));
                }
            }
            if let Some(error) = &device.init_error {
                let v = localize_graphics_init_error(error);
                card = card.push(text(tg_iced::text(v.heading).to_owned()));
                if let Some(details) = v.details {
                    card = card.push(text(details));
                }
                if let Some(key) = v.recommendation {
                    card = card.push(text(tg_iced::text(key).to_owned()));
                }
            }
            let select = button(text(
                tg_iced::text(
                    if matches!(device.driver_support, DriverSupport::Supported) {
                        "Play"
                    } else {
                        "Driver.IgnoreAndPlay"
                    },
                )
                .to_owned(),
            ))
            .on_press_maybe(
                device
                    .init_error
                    .is_none()
                    .then_some(Message::Select(index)),
            );
            card = card.push(select);
            cards = cards.push(container(card).padding(15).width(Length::Fill));
        }
        container(
            column![
                text(tg_iced::text(self.heading()).to_owned()).size(22),
                scrollable(cards).height(Length::Fill),
                row![
                    button(text(tg_iced::text("Crash.ShowDetails").to_owned()))
                        .on_press(Message::ShowMoreInfo),
                    button(text(tg_iced::text("Driver.ShowLogFiles").to_owned()))
                        .on_press(Message::OpenLog),
                    button(text(tg_iced::text("Menu.Quit").to_owned())).on_press(Message::Close)
                ]
                .spacing(12)
            ]
            .spacing(20)
            .padding(30),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    fn update_native(&mut self, message: Message) -> tg_iced::iced::Command<Message> {
        match self.update(message) {
            Command::OpenUrl(url) => {
                if let Err(e) = webbrowser::open(&url) {
                    eprintln!("Error opening web browser: {e}");
                }
                tg_iced::iced::Command::none()
            }
            Command::OpenPath(path) => {
                if let Err(e) = open::that(path) {
                    eprintln!("Error opening log: {e}");
                }
                tg_iced::iced::Command::none()
            }
            other => tg_iced::gui_runner::platform_command(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_error_families_keep_distinct_recommendations() {
        for tag in 0..32 {
            let v = localize_graphics_init_error(&GraphicsInitError {
                tag,
                details: "detail".into(),
            });
            if tag == 0x17 {
                assert_eq!(v.recommendation, None);
            } else if tag == 0x15 {
                assert_eq!(
                    v.recommendation,
                    Some("GraphicsInitError.OutOfVideoMemoryRecommendation")
                );
            } else if tag == 0x14 {
                assert_eq!(
                    v.recommendation,
                    Some("GraphicsInitError.SwapchainErrorTip")
                );
            } else {
                assert_eq!(
                    v.recommendation,
                    Some("GraphicsInitError.PleaseUpdateDrivers")
                );
            }
        }
    }
    #[test]
    fn unknown_and_emulated_have_no_vendor_links() {
        for s in [DriverSupport::UnknownVendor(7), DriverSupport::Emulated] {
            assert!(
                device_driver_advice(Vendor::Nvidia, "1", &s)
                    .links
                    .is_empty()
            );
        }
    }
}
