//! Validated candidate selection and one-way close/submit lifecycle.
use super::{Device, DeviceId, SelectionReason};
use std::path::{Path, PathBuf};
use tg_iced::gui_runner::{Command, ProgramWithOutcome};
#[derive(Debug, Clone, PartialEq, Eq)]
/// Selection inputs and requested external actions. Invalid or unavailable selections are ignored.
pub enum Message {
    /// Select an existing candidate without an initialization error; driver warnings may be overridden.
    Select(DeviceId),
    ShowMoreInfo,
    OpenUrl(String),
    Close,
    OpenLog,
    Noop,
}
#[derive(Debug, Clone)]
/// Selection state for one immutable candidate list. A valid choice or Close is terminal.
/// Once closed, messages cannot change the result or request external IO.
pub struct Controls {
    devices: Vec<Device>,
    reason: SelectionReason,
    more_info: bool,
    log_path: PathBuf,
    outcome: Option<usize>,
    closed: bool,
}
impl Controls {
    /// Start with no result and details hidden. Empty lists are allowed; the supplied log path is not opened yet.
    pub fn new(devices: Vec<Device>, log_path: PathBuf) -> Self {
        Self {
            devices,
            reason: SelectionReason::Requested,
            more_info: false,
            log_path,
            outcome: None,
            closed: false,
        }
    }
    /// Set the heading reason before close; terminal controls remain unchanged.
    pub fn with_reason(mut self, reason: SelectionReason) -> Self {
        if !self.closed {
            self.reason = reason;
        }
        self
    }
    /// Borrow candidates in their original order, which also defines outcome indices.
    pub fn view(&self) -> &[Device] {
        &self.devices
    }
    /// Return the current heading localization key.
    pub fn heading(&self) -> &'static str {
        self.reason.key()
    }
    /// Whether the player has requested additional driver information and links.
    pub fn more_info(&self) -> bool {
        self.more_info
    }
    /// Borrow the path used by the OpenLog action; construction does not check its existence.
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }
    /// Distinguish an active dialog from a terminal one; a terminal result may still be `None` after Close.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
impl ProgramWithOutcome for Controls {
    type Message = Message;
    type Outcome = usize;
    fn update(&mut self, message: Message) -> Command {
        if self.closed {
            return Command::CloseWindow;
        }
        match message {
            Message::Select(id) => {
                if self
                    .devices
                    .get(id.index())
                    .is_some_and(Device::is_selectable)
                {
                    self.outcome = Some(id.index());
                    self.closed = true;
                    Command::CloseWindow
                } else {
                    Command::None
                }
            }
            Message::ShowMoreInfo => {
                self.more_info = true;
                Command::None
            }
            Message::OpenUrl(url) => Command::OpenUrl(url),
            Message::Close => {
                self.closed = true;
                Command::CloseWindow
            }
            Message::OpenLog => Command::OpenPath(self.log_path.clone()),
            Message::Noop => Command::None,
        }
    }
    fn program_outcome(&self) -> Option<usize> {
        self.outcome
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_selection::{DriverSupport, GraphicsInitError, Vendor};
    use tg_iced::gui_runner::{GuiBackend, GuiError, Options};
    struct BackendMustNotRun;
    impl GuiBackend<Controls> for BackendMustNotRun {
        fn run(&mut self, _: &mut Controls, _: &Options) -> Result<(), GuiError> {
            panic!("closed controls must not enter the backend")
        }
    }
    fn device(error: bool) -> Device {
        Device {
            name: "fixture".into(),
            vendor: Vendor::Intel,
            driver_version: "1".into(),
            driver_support: DriverSupport::Unsupported {
                minimum: "2".into(),
            },
            init_error: error.then(|| GraphicsInitError {
                tag: 0x14,
                details: "failed".into(),
            }),
        }
    }
    #[test]
    fn selection_validates_bounds_and_initialization_before_freezing_outcome() {
        let mut controls = Controls::new(
            vec![device(true), device(false), device(false)],
            "log".into(),
        );
        for index in [0, 3, usize::MAX] {
            assert_eq!(
                controls.update(Message::Select(DeviceId::new(index))),
                Command::None
            );
            assert_eq!(controls.program_outcome(), None);
        }
        assert_eq!(
            controls.update(Message::Select(DeviceId::new(1))),
            Command::CloseWindow
        );
        assert_eq!(controls.program_outcome(), Some(1));
        for message in [
            Message::Select(DeviceId::new(2)),
            Message::ShowMoreInfo,
            Message::OpenLog,
            Message::Close,
        ] {
            assert_eq!(controls.update(message), Command::CloseWindow);
        }
        assert_eq!(controls.program_outcome(), Some(1));
        assert!(!controls.more_info());
        assert_eq!(
            crate::run_gpu_selection_gui(
                controls.clone(),
                Options::new("unused", 1, 1),
                &mut BackendMustNotRun
            ),
            Ok(Some(1))
        );
        let mut closed = Controls::new(vec![device(false)], "log".into());
        closed.update(Message::Close);
        closed.update(Message::Select(DeviceId::new(0)));
        assert!(closed.is_closed());
        assert_eq!(closed.program_outcome(), None);
        assert_eq!(
            crate::run_gpu_selection_gui(
                closed,
                Options::new("unused", 1, 1),
                &mut BackendMustNotRun
            ),
            Ok(None)
        );
    }
}
