//! Window options, application outcomes and an injectable GUI backend.
use std::fmt;

/// Window title and requested logical size. Native windows clamp each dimension to at least one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Options {
    /// Store window settings; injected backends decide how to apply the requested size.
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
        }
    }
}

/// Effects requested by a business update; the backend owns their execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Keep running without a platform effect.
    None,
    /// End the window session, with or without a business outcome.
    CloseWindow,
    /// Open a URL through a consumer-supplied adapter.
    OpenUrl(String),
    /// Open a filesystem path through a consumer-supplied adapter.
    OpenPath(std::path::PathBuf),
    /// Replace the system clipboard text.
    CopyToClipboard(String),
}

/// Business state driven by messages and observed through a stable optional result.
/// Once a result exists, it must remain unchanged until the backend returns.
pub trait ProgramWithOutcome {
    /// Input delivered by controls or the optional periodic subscription.
    type Message;
    /// Owned result returned to the caller after a successful backend run.
    type Outcome;
    /// Apply one message and return the requested platform effect.
    fn update(&mut self, message: Self::Message) -> Command;
    /// Observe the current result without side effects or consuming it.
    ///
    /// Runners may query repeatedly. This owning API can require cloning a stored
    /// result, so implementations should keep observation inexpensive.
    fn program_outcome(&self) -> Option<Self::Outcome>;
    /// Return an optional periodic message without changing business state.
    /// Native runners query this both when subscribing and when delivering a tick.
    fn tick_message(&self) -> Option<Self::Message> {
        None
    }
}

/// Backend execution failures and capture-specific missing results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiError {
    /// An injected backend cannot run in the current environment.
    BackendUnavailable,
    /// Backend or consumer I/O failure, including native backend panics.
    Backend(String),
    /// The native adapter has no implementation for this business effect.
    UnsupportedCommand(Command),
    /// The window closed before a required result was produced.
    ClosedWithoutOutcome,
    /// A screenshot was not received within five seconds of the first capture tick.
    CaptureTimedOut,
}
impl fmt::Display for GuiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable => formatter.write_str("no GUI backend is available"),
            Self::Backend(message) => write!(formatter, "GUI backend failed: {message}"),
            Self::UnsupportedCommand(command) => {
                write!(
                    formatter,
                    "GUI command needs a consumer adapter: {command:?}"
                )
            }
            Self::ClosedWithoutOutcome => {
                formatter.write_str("window closed without producing a result")
            }
            Self::CaptureTimedOut => {
                formatter.write_str("screenshot did not arrive before the capture deadline")
            }
        }
    }
}
impl std::error::Error for GuiError {}
/// A real backend owns window creation, rendering, event delivery and command errors.
pub trait GuiBackend<P: ProgramWithOutcome> {
    /// Drive the program until close or completion, honoring its requested effects.
    /// Return backend or command failures as `GuiError`; the runner propagates them
    /// before reading the final business result.
    fn run(&mut self, program: &mut P, options: &Options) -> Result<(), GuiError>;
}

/// Run an injected backend and retrieve its program's optional result.
/// `None` means the session ended without a business choice.
///
/// # Errors
/// Returns backend or command failures unchanged, even if a result already exists.
pub fn run_gui<P: ProgramWithOutcome>(
    mut controls: P,
    options: Options,
    backend: &mut impl GuiBackend<P>,
) -> Result<Option<P::Outcome>, GuiError> {
    backend.run(&mut controls, &options)?;
    Ok(controls.program_outcome())
}

#[cfg(feature = "native-gui")]
mod capture;
#[cfg(any(feature = "native-gui", test))]
mod capture_file;
#[cfg(feature = "native-gui")]
mod native;
#[cfg(feature = "native-gui")]
pub use capture::capture_native_gui;
#[cfg(feature = "native-gui")]
pub use capture_file::write_native_capture;
#[cfg(feature = "native-gui")]
pub use native::{NativeAction, NativeProgram, platform_command, run_native_gui};

#[cfg(test)]
mod tests {
    use super::*;
    struct Program;
    impl ProgramWithOutcome for Program {
        type Message = ();
        type Outcome = bool;
        fn update(&mut self, _: ()) -> Command {
            Command::None
        }
        fn program_outcome(&self) -> Option<bool> {
            Some(true)
        }
    }
    struct FailingBackend;
    impl GuiBackend<Program> for FailingBackend {
        fn run(&mut self, _: &mut Program, _: &Options) -> Result<(), GuiError> {
            Err(GuiError::Backend("fixture failure".into()))
        }
    }
    #[test]
    fn backend_error_is_not_replaced_with_program_outcome() {
        assert_eq!(
            run_gui(Program, Options::new("fixture", 1, 1), &mut FailingBackend),
            Err(GuiError::Backend("fixture failure".into()))
        );
    }
}
