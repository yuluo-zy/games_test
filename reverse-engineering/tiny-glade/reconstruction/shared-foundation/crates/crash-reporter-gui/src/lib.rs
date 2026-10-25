//! B crash reporter's game-owned interaction and diagnostics policy.
//! Platform clipboard/browser and report delivery remain external adapters.
use std::{path::PathBuf, thread::JoinHandle, time::Instant};
use tg_iced::gui_runner::{
    Command, GuiBackend, GuiError, Options, ProgramWithOutcome, run_gui_with_options,
};
pub const TROUBLESHOOTING_GUIDE: &str = "https://pouncelight.games/tiny-glade/troubleshooting/";
pub const PRIVACY_POLICY: &str = "https://pouncelight.games/privacy/";
const DRIVER_FRAMES: [&str; 11] = [
    "ig11icd64.dll",
    "ig12icd64.dll",
    "igxelpicd64.dll",
    "igd12umd64.dll",
    "igd12um64xeh.dll",
    "amdxc64.dll",
    "amdvlk64.dll",
    "atio6axx.dll",
    "nvoglv64.dll",
    "libGLX_nvidia.so.0",
    "libnvidia-glcore.so",
];
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitialScreen {
    Integrity(tg_integrity_report::controls::VerificationStatus),
    Advice(u8),
    Main,
}
/// Original pre-report reason tag 13 permits diagnosis; explicit other reasons win.
/// original_hint preserves the observed tri-state byte; its semantic producer is unknown.
pub fn diagnose(
    frames: &[String],
    log_text: &str,
    original_hint: u8,
    explicit_reason: u8,
    integrity: Option<tg_integrity_report::controls::VerificationStatus>,
) -> InitialScreen {
    if explicit_reason != 13 {
        return InitialScreen::Advice(explicit_reason);
    }
    let has = |needle: &str| frames.iter().any(|f| f.contains(needle));
    let driver = DRIVER_FRAMES.iter().any(|needle| has(needle));
    let lost =
        log_text.contains("DEVICE_LOST") || log_text.contains("The GPU device has been lost");
    let computed = if has("vulkan_dzn.dll") {
        Some(8)
    } else if original_hint == 0 && (driver || lost) {
        Some(5)
    } else if frames.iter().take(8).any(|f| f.contains("fpsmonvk64.dll")) {
        Some(12)
    } else if frames
        .iter()
        .take(8)
        .any(|f| f.contains("graphics-hook64.dll"))
    {
        Some(9)
    } else if frames.first().is_some_and(|f| f.contains("vulkan-1.dll")) {
        Some(6)
    } else {
        None
    };
    if let Some(code) = computed {
        return InitialScreen::Advice(code);
    }
    if let Some(report) = integrity {
        return InitialScreen::Integrity(report);
    }
    if has("gtiii-osd64-vk.dll") {
        InitialScreen::Advice(10)
    } else if driver {
        InitialScreen::Advice(7)
    } else if lost {
        InitialScreen::Advice(11)
    } else {
        InitialScreen::Main
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashInfo {
    pub heading: String,
    pub diagnostic_text: String,
    pub log_path: Option<PathBuf>,
    pub recovery_succeeded: bool,
}
/// Candidate delivery adapter payload; the original callback result ABI is not established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportResult {
    pub delivered: bool,
    pub discussion_code: Option<String>,
}
pub enum Screen {
    Initial(InitialScreen),
    Consent,
    DismissedAdvice,
    Sending(JoinHandle<ReportResult>),
    Sent(ReportResult),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Close,
    OpenPath(PathBuf),
    PrivacyPolicy,
    SendReport,
    DismissAdvice,
    Copy(String),
    OpenUrl(String),
    ToggleMoreInfo,
    Tick,
    RemoveExtra(String),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardStatus {
    Copied,
    Error(String),
}
pub trait Clipboard {
    fn set_text(&mut self, text: &str) -> Result<(), String>;
}
pub struct Controls {
    pub info: CrashInfo,
    pub screen: Screen,
    pub more_info: bool,
    pub clipboard_status: Option<(Instant, ClipboardStatus)>,
    report_callback: Option<Box<dyn FnOnce() -> JoinHandle<ReportResult> + Send>>,
    pub integrity: Option<tg_integrity_report::Controls>,
}
impl Controls {
    pub fn new(
        info: CrashInfo,
        screen: InitialScreen,
        callback: impl FnOnce() -> JoinHandle<ReportResult> + Send + 'static,
    ) -> Self {
        Self {
            info,
            screen: Screen::Initial(screen),
            more_info: false,
            clipboard_status: None,
            report_callback: Some(Box::new(callback)),
            integrity: None,
        }
    }
    /// Attach the original installation-result consumer so its deletion updates the shown report.
    pub fn with_integrity_report(mut self, report: tg_integrity_report::Controls) -> Self {
        if matches!(self.screen, Screen::Initial(InitialScreen::Integrity(_))) {
            self.screen = Screen::Initial(InitialScreen::Integrity(report.view()));
        }
        self.integrity = Some(report);
        self
    }
    pub fn update_with_remover(
        &mut self,
        message: Message,
        remover: &mut impl tg_integrity_report::controls::PackageFileRemover,
    ) -> Command {
        if let Message::RemoveExtra(path) = message {
            let allowed = self.can_change_advice();
            if let Some(report) = self.integrity.as_mut().filter(|_| allowed) {
                report.remove_unrecognized_package_file(&path, remover);
                self.screen = Screen::Initial(InitialScreen::Integrity(report.view()));
            }
            Command::None
        } else {
            self.update(message)
        }
    }
    pub fn copy_to_clipboard(&mut self, text: &str, clipboard: &mut impl Clipboard) {
        let result = clipboard.set_text(text);
        self.clipboard_status = Some((
            Instant::now(),
            match result {
                Ok(()) => ClipboardStatus::Copied,
                Err(e) => ClipboardStatus::Error(e),
            },
        ));
    }
    pub fn more_info_button(&self, url: impl Into<String>) -> (&'static str, Message) {
        ("Crash.Advice.ClickForAdvice", Message::OpenUrl(url.into()))
    }
    pub fn clipboard_label(&self) -> &'static str {
        match &self.clipboard_status {
            Some((_, ClipboardStatus::Error(_))) => "Crash.Clipboard.Error",
            Some((when, ClipboardStatus::Copied)) if when.elapsed().as_secs_f32() < 1.0 => {
                "Crash.Clipboard.Copied"
            }
            _ => "Crash.Clipboard.Copy",
        }
    }
    pub fn main_view(&self) -> Vec<&'static str> {
        let mut keys = vec![
            "Crash.GameHasCrashed",
            if self.info.recovery_succeeded {
                "Crash.RecoveryOk"
            } else {
                "Crash.RecoveryFailed"
            },
            "Crash.AskConsent",
            "Crash.Under13",
            "Crash.PrivacyPolicy",
        ];
        match &self.screen {
            Screen::Sent(r) => keys.push(if r.delivered {
                "Crash.DeliveryOk"
            } else {
                "Crash.DeliveryFailed"
            }),
            Screen::DismissedAdvice => keys.push("Crash.AdviceDismissed"),
            _ => {}
        }
        keys
    }
    pub fn view(&self) -> Vec<&'static str> {
        match &self.screen {
            Screen::Initial(InitialScreen::Advice(code)) => match code {
                5 => vec!["Crash.Advice.UpdateDriver"],
                6 => vec!["Crash.FixDriver"],
                7 => vec!["Crash.Advice.TryUpdateDriver"],
                8 => vec!["Crash.Advice.RemoveDozen"],
                9 => vec!["Crash.Advice.UpdateObs"],
                11 => vec!["Crash.Advice.TroubleshootGraphics"],
                10 => vec![
                    "We suspect this crash might be caused by ASUS GPU Tweak III.\nTry disabling its OSD (On-Screen Display) 🧡",
                ],
                12 => vec![
                    "We suspect this crash might be caused by FPS Monitor.\nTry disabling or uninstalling it 🧡",
                ],
                _ => vec!["Crash.ShowDetails"],
            },
            Screen::Initial(InitialScreen::Integrity(_)) => vec![
                "Crash.Advice.CheckIntegrity",
                "Crash.Advice.ShowInstallationErrors",
            ],
            _ => self.main_view(),
        }
    }
    pub fn update_with_clipboard(
        &mut self,
        message: Message,
        clipboard: &mut impl Clipboard,
    ) -> Command {
        if let Message::Copy(text) = message {
            self.copy_to_clipboard(&text, clipboard);
            Command::None
        } else {
            self.update(message)
        }
    }
    fn can_change_advice(&self) -> bool {
        matches!(
            self.screen,
            Screen::Initial(_) | Screen::Consent | Screen::DismissedAdvice
        )
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = ();
    fn update(&mut self, message: Message) -> Command {
        match message {
            Message::Close => return Command::CloseWindow,
            Message::OpenPath(path) => return Command::OpenPath(path),
            Message::PrivacyPolicy => return Command::OpenUrl(PRIVACY_POLICY.into()),
            Message::SendReport => {
                let callback = if self.can_change_advice() {
                    self.report_callback.take()
                } else {
                    None
                };
                if let Some(callback) = callback {
                    self.screen = Screen::Sending(callback());
                }
            }
            Message::DismissAdvice => {
                if self.can_change_advice() {
                    self.screen = Screen::DismissedAdvice;
                }
            }
            Message::Copy(text) => return Command::CopyToClipboard(text),
            Message::OpenUrl(url) => return Command::OpenUrl(url),
            Message::ToggleMoreInfo => {
                if self.can_change_advice() {
                    self.more_info = !self.more_info;
                }
            }
            Message::Tick => {
                let done = matches!(&self.screen,Screen::Sending(handle) if handle.is_finished());
                if done {
                    let screen = std::mem::replace(&mut self.screen, Screen::Consent);
                    if let Screen::Sending(handle) = screen {
                        match handle.join() {
                            Ok(result) => self.screen = Screen::Sent(result),
                            Err(payload) => std::panic::resume_unwind(payload),
                        }
                    }
                }
            }
            Message::RemoveExtra(path) => return Command::RemovePackageFile(path),
        }
        Command::None
    }
    fn program_outcome(&self) -> Option<()> {
        None
    }
    fn tick_message(&self) -> Option<Message> {
        Some(Message::Tick)
    }
}
pub fn run_crash_reporter_gui(
    controls: Controls,
    options: Options,
    backend: &mut impl GuiBackend<Controls>,
) -> Result<(), GuiError> {
    run_gui_with_options(controls, options, backend).map(|_| ())
}

#[cfg(feature = "native-gui")]
pub struct NativeControls {
    pub controls: Controls,
    pub remover: tg_integrity_report::controls::ScopedPackageFileRemover,
}
#[cfg(feature = "native-gui")]
struct NativeClipboard;
#[cfg(feature = "native-gui")]
impl Clipboard for NativeClipboard {
    fn set_text(&mut self, text: &str) -> Result<(), String> {
        arboard::Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(text))
            .map_err(|e| e.to_string())
    }
}
#[cfg(feature = "native-gui")]
impl ProgramWithOutcome for NativeControls {
    type Message = Message;
    type Outcome = ();
    fn update(&mut self, message: Message) -> Command {
        self.controls.update(message)
    }
    fn program_outcome(&self) -> Option<()> {
        None
    }
    fn tick_message(&self) -> Option<Message> {
        Some(Message::Tick)
    }
}
#[cfg(feature = "native-gui")]
impl tg_iced::gui_runner::NativeProgram for NativeControls {
    fn view_native(&self) -> tg_iced::iced::Element<'static, Message> {
        use tg_iced::iced::Length;
        use tg_iced::iced::widget::{button, column, container, row, scrollable};
        use tg_iced::native_text as text;
        let c = &self.controls;
        let mut body = column!().spacing(15);
        match &c.screen {
            Screen::Sending(_) => {
                body = body.push(text("🐑💌             👩‍💻👨‍💻").size(30));
            }
            Screen::Sent(result) => {
                body = body.push(
                    text(
                        tg_iced::text(if result.delivered {
                            "Crash.DeliveryOk"
                        } else {
                            "Crash.DeliveryFailed"
                        })
                        .to_owned(),
                    )
                    .size(20),
                );
                if let Some(code) = &result.discussion_code {
                    body = body
                        .push(text(tg_iced::text("Crash.Discuss").to_owned()))
                        .push(text(code.clone()))
                        .push(
                            button(text(tg_iced::text(c.clipboard_label()).to_owned()))
                                .on_press(Message::Copy(code.clone())),
                        );
                }
                body = body
                    .push(button(text(tg_iced::text("Ok").to_owned())).on_press(Message::Close));
            }
            Screen::Initial(InitialScreen::Integrity(_)) => {
                body = body.push(text(
                    tg_iced::text("Crash.Advice.CheckIntegrity").to_owned(),
                ));
                if let Some(report) = &c.integrity {
                    body = body.push(report.native_element().map(|message| match message {
                        tg_integrity_report::controls::Message::Close => Message::DismissAdvice,
                        tg_integrity_report::controls::Message::OpenUrl(url) => {
                            Message::OpenUrl(url)
                        }
                        tg_integrity_report::controls::Message::RemoveUnrecognizedPackageFile(
                            path,
                        ) => Message::RemoveExtra(path),
                    }));
                }
                body = body.push(
                    button(text(tg_iced::text("Crash.Advice.IveTriedThat").to_owned()))
                        .on_press(Message::DismissAdvice),
                );
            }
            Screen::Initial(InitialScreen::Advice(code)) => {
                for key in c.view() {
                    body = body.push(text(tg_iced::text(key).to_owned()).size(20));
                }
                let url = match code {
                    6 => Some(
                        "https://pouncelight.games/tiny-glade/troubleshooting/runtime-crash/#broken-drivers",
                    ),
                    8 => Some(
                        "https://pouncelight.games/tiny-glade/troubleshooting/runtime-crash/#vulkanon12",
                    ),
                    9 => Some(
                        "https://pouncelight.games/tiny-glade/troubleshooting/runtime-crash/#obs",
                    ),
                    11 => Some(
                        "https://pouncelight.games/tiny-glade/troubleshooting/runtime-crash/#device-lost",
                    ),
                    _ => None,
                };
                if let Some(url) = url {
                    let (key, message) = c.more_info_button(url);
                    body = body.push(button(text(tg_iced::text(key).to_owned())).on_press(message));
                }
                if let Some(path) = &c.info.log_path {
                    body = body.push(
                        button(text(tg_iced::text("Driver.ShowLogFiles").to_owned()))
                            .on_press(Message::OpenPath(path.clone())),
                    );
                }
                body = body.push(
                    button(text(tg_iced::text("Crash.Advice.IveTriedThat").to_owned()))
                        .on_press(Message::DismissAdvice),
                );
            }
            _ => {
                for key in c.main_view() {
                    body = body.push(text(
                        tg_iced::text(key).replace("{nope}", tg_iced::text("Continue.Nope")),
                    ));
                }
                let send = button(text(tg_iced::text("Continue.Yep").to_owned()))
                    .on_press_maybe(c.report_callback.is_some().then_some(Message::SendReport));
                body = body
                    .push(
                        row![
                            send,
                            button(text(tg_iced::text("Continue.Nope").to_owned()))
                                .on_press(Message::Close)
                        ]
                        .spacing(20),
                    )
                    .push(
                        row![
                            button(text(tg_iced::text("Crash.ShowDetails").to_owned()))
                                .on_press(Message::ToggleMoreInfo),
                            button(text(tg_iced::text("Crash.PrivacyPolicy").to_owned()))
                                .on_press(Message::PrivacyPolicy),
                            button(text(tg_iced::text("Crash.TroubleshootingGuide").to_owned()))
                                .on_press(Message::OpenUrl(TROUBLESHOOTING_GUIDE.into()))
                        ]
                        .spacing(12),
                    );
            }
        }
        if c.more_info {
            body = body.push(text(c.info.diagnostic_text.clone()));
        }
        container(scrollable(body).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(30)
            .into()
    }
    fn update_native(&mut self, message: Message) -> tg_iced::iced::Command<Message> {
        let command = match message {
            Message::Copy(text) => {
                self.controls.copy_to_clipboard(&text, &mut NativeClipboard);
                Command::None
            }
            Message::RemoveExtra(path) => self
                .controls
                .update_with_remover(Message::RemoveExtra(path), &mut self.remover),
            other => self.controls.update(other),
        };
        match command {
            Command::OpenUrl(url) => {
                if let Err(e) = webbrowser::open(&url) {
                    eprintln!("Error opening web browser: {e}");
                }
                tg_iced::iced::Command::none()
            }
            Command::OpenPath(path) => {
                if let Err(e) = open::that(path) {
                    eprintln!("Error opening file: {e}");
                }
                tg_iced::iced::Command::none()
            }
            other => tg_iced::gui_runner::platform_command(other),
        }
    }
}
#[cfg(feature = "native-gui")]
pub fn run_crash_reporter_gui_native(
    controls: Controls,
    package_root: impl AsRef<std::path::Path>,
    options: Options,
) -> Result<(), GuiError> {
    let remover = tg_integrity_report::controls::ScopedPackageFileRemover::new(package_root)
        .map_err(|e| GuiError::Backend(e.to_string()))?;
    tg_iced::gui_runner::run_native_gui(NativeControls { controls, remover }, options).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_window_and_integrity_precedence() {
        let mut f = vec!["none".into(); 9];
        f[8] = "graphics-hook64.dll".into();
        assert_eq!(diagnose(&f, "", 2, 13, None), InitialScreen::Main);
        f[7] = "graphics-hook64.dll".into();
        assert_eq!(diagnose(&f, "", 2, 13, None), InitialScreen::Advice(9));
        f[0] = "vulkan_dzn.dll".into();
        assert_eq!(
            diagnose(&f, "DEVICE_LOST", 0, 13, None),
            InitialScreen::Advice(8)
        );
        assert_eq!(diagnose(&f, "", 0, 7, None), InitialScreen::Advice(7));
    }
    #[test]
    fn callback_is_consumed_once_and_tick_joins_real_worker() {
        let mut c = Controls::new(
            CrashInfo {
                heading: "crash".into(),
                diagnostic_text: "data".into(),
                log_path: None,
                recovery_succeeded: true,
            },
            InitialScreen::Main,
            || {
                std::thread::spawn(|| ReportResult {
                    delivered: false,
                    discussion_code: Some("fixture".into()),
                })
            },
        );
        c.update(Message::SendReport);
        c.update(Message::SendReport);
        let handle = match std::mem::replace(&mut c.screen, Screen::Consent) {
            Screen::Sending(h) => h,
            _ => panic!(),
        };
        while !handle.is_finished() {
            std::thread::yield_now();
        }
        c.screen = Screen::Sending(handle);
        c.update(Message::Tick);
        assert!(matches!(
            c.screen,
            Screen::Sent(ReportResult {
                delivered: false,
                ..
            })
        ));
        assert!(c.report_callback.is_none());
    }
    #[test]
    fn deletion_forwards_to_attached_integrity_consumer() {
        use tg_integrity_report::controls::{PackageFileRemover, VerificationResult};
        struct Remove {
            fail: bool,
        }
        impl PackageFileRemover for Remove {
            fn remove_file(&mut self, _: &str) -> std::io::Result<()> {
                if self.fail {
                    Err(std::io::Error::other("fixture failure"))
                } else {
                    Ok(())
                }
            }
        }
        let result: VerificationResult = Err(
            tg_integrity_report::controls::PackageVerificationError::Files(
                tg_integrity_report::controls::FileIssues {
                    extra: vec!["extra".into()],
                    ..Default::default()
                },
            ),
        );
        let report = tg_integrity_report::Controls::new(result);
        let screen = InitialScreen::Integrity(report.view());
        let mut c = Controls::new(
            CrashInfo {
                heading: String::new(),
                diagnostic_text: String::new(),
                log_path: None,
                recovery_succeeded: false,
            },
            screen,
            || {
                std::thread::spawn(|| ReportResult {
                    delivered: false,
                    discussion_code: None,
                })
            },
        )
        .with_integrity_report(report);
        c.update_with_remover(
            Message::RemoveExtra("extra".into()),
            &mut Remove { fail: true },
        );
        assert_eq!(c.integrity.as_ref().unwrap().view().extra, ["extra"]);
        c.update_with_remover(
            Message::RemoveExtra("extra".into()),
            &mut Remove { fail: false },
        );
        assert!(c.integrity.as_ref().unwrap().result.is_ok());
        assert!(
            matches!(&c.screen,Screen::Initial(InitialScreen::Integrity(v)) if v.extra.is_empty())
        );
    }
}
