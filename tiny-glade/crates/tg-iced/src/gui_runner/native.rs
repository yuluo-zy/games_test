//! Native window lifecycle and fallible consumer command adaptation.
use super::{Command, GuiError, Options, ProgramWithOutcome};
use iced::{Application, Element, Subscription, Theme};
use std::{
    fmt,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};

/// A native effect with an explicit session lifecycle decision.
/// Close requests stay visible to the runner before iced drains queued messages.
#[derive(Debug)]
#[must_use]
pub enum NativeAction<M> {
    /// Execute a platform command and continue receiving business messages.
    /// Use `Close` for window shutdown so the runner can freeze updates immediately.
    Continue(iced::Command<M>),
    /// Stop business updates and ticks, then close the main window.
    Close,
}

/// Native view construction and fallible adaptation of business effects.
/// Consumers handle their own I/O and return explicit close requests to the runner.
pub trait NativeProgram: ProgramWithOutcome + Send + 'static
where
    Self::Message: Clone + fmt::Debug + Send + 'static,
{
    /// Build a view without business side effects.
    /// The element owns its text and widgets because the program's mutex guard
    /// is released before iced renders it; borrowed program data cannot escape.
    fn view_native(&self) -> Element<'static, Self::Message>;
    /// Apply one message and adapt its effect to iced or request closure.
    ///
    /// Return consumer I/O failures as `GuiError`. The default adapter reports
    /// `UnsupportedCommand` for URL/path operations that need a consumer adapter.
    fn update_native(
        &mut self,
        message: Self::Message,
    ) -> Result<NativeAction<Self::Message>, GuiError> {
        platform_command(self.update(message))
    }
}

/// Adapt no-op, clipboard, and close effects without executing consumer I/O.
///
/// # Errors
/// Returns `UnsupportedCommand` for URL and filesystem path operations.
pub fn platform_command<M>(command: Command) -> Result<NativeAction<M>, GuiError> {
    match command {
        Command::CloseWindow => Ok(NativeAction::Close),
        Command::None => Ok(NativeAction::Continue(iced::Command::none())),
        Command::CopyToClipboard(text) => Ok(NativeAction::Continue(iced::clipboard::write(text))),
        other => Err(GuiError::UnsupportedCommand(other)),
    }
}

// The application consumes its flags; shared ownership lets the caller retrieve
// the final result after the event loop returns.
struct SharedProgram<P> {
    controls: P,
    error: Option<GuiError>,
    // Once set, no further messages or tick queries may mutate business state.
    closing: bool,
}

// No user callback runs on a second thread. Recovering the guard allows the runner
// to inspect a stored result even if a backend unwinds while holding it.
fn lock_program<P>(state: &Mutex<SharedProgram<P>>) -> MutexGuard<'_, SharedProgram<P>> {
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

struct WindowFlags<P> {
    state: Arc<Mutex<SharedProgram<P>>>,
    title: String,
}
#[derive(Clone, Debug)]
enum WindowMessage<M> {
    Control(M),
    Tick,
}
struct NativeWindow<P> {
    state: Arc<Mutex<SharedProgram<P>>>,
    title: String,
}
impl<P> Application for NativeWindow<P>
where
    P: NativeProgram,
    P::Message: Clone + fmt::Debug + Send + 'static,
{
    type Executor = iced::executor::Default;
    type Message = WindowMessage<P::Message>;
    type Theme = Theme;
    type Flags = WindowFlags<P>;
    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let command = {
            let mut state = lock_program(&flags.state);
            if state.error.is_some() || state.controls.program_outcome().is_some() {
                state.closing = true;
                iced::window::close(iced::window::Id::MAIN)
            } else {
                iced::Command::none()
            }
        };
        (
            Self {
                state: flags.state,
                title: flags.title,
            },
            command,
        )
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        let mut state = lock_program(&self.state);
        if state.closing {
            return iced::Command::none();
        }
        if state.error.is_some() || state.controls.program_outcome().is_some() {
            state.closing = true;
            return iced::window::close(iced::window::Id::MAIN);
        }
        let message = match message {
            WindowMessage::Control(message) => Some(message),
            WindowMessage::Tick => state.controls.tick_message(),
        };
        let result = message
            .map(|message| state.controls.update_native(message))
            .unwrap_or_else(|| Ok(NativeAction::Continue(iced::Command::none())));
        match result {
            Ok(NativeAction::Close) => {
                state.closing = true;
                iced::window::close(iced::window::Id::MAIN)
            }
            Ok(NativeAction::Continue(command)) => {
                let command = command.map(WindowMessage::Control);
                if state.controls.program_outcome().is_some() {
                    state.closing = true;
                    iced::Command::batch([command, iced::window::close(iced::window::Id::MAIN)])
                } else {
                    command
                }
            }
            Err(error) => {
                state.error = Some(error);
                state.closing = true;
                iced::window::close(iced::window::Id::MAIN)
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        lock_program(&self.state)
            .controls
            .view_native()
            .map(WindowMessage::Control)
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        let state = lock_program(&self.state);
        if state.closing || state.error.is_some() || state.controls.program_outcome().is_some() {
            return Subscription::none();
        }
        if state.controls.tick_message().is_some() {
            iced::time::every(Duration::from_millis(16)).map(|_| WindowMessage::Tick)
        } else {
            Subscription::none()
        }
    }
}

/// Run one native GUI session, returning its result or `None` for a close without one.
/// An already completed program returns immediately without opening a window.
/// Otherwise this must run on the main thread, and the backend permits one event
/// loop per process. Zero width or height is clamped to one logical pixel.
///
/// # Errors
/// Returns consumer command failures and backend initialization/execution errors.
/// Rust unwinds during backend execution become `GuiError::Backend`; the existing panic
/// hook still runs. Result callbacks before and after backend execution are outside
/// that unwind boundary.
pub fn run_native_gui<P>(controls: P, options: Options) -> Result<Option<P::Outcome>, GuiError>
where
    P: NativeProgram,
    P::Message: Clone + fmt::Debug + Send + 'static,
{
    // A completed program needs neither a window nor a running display backend.
    if let Some(outcome) = controls.program_outcome() {
        return Ok(Some(outcome));
    }
    let state = Arc::new(Mutex::new(SharedProgram {
        controls,
        error: None,
        closing: false,
    }));
    let mut settings = iced::Settings::with_flags(WindowFlags {
        state: state.clone(),
        title: options.title,
    });
    settings.window.size =
        iced::Size::new(options.width.max(1) as f32, options.height.max(1) as f32);
    run_backend(|| NativeWindow::<P>::run(settings))?;
    let mut state = lock_program(&state);
    match state.error.take() {
        Some(error) => Err(error),
        None => Ok(state.controls.program_outcome()),
    }
}

// This iced fork expects event-loop creation to succeed, so initialization
// failures can unwind before its Result exists. Controls are discarded on failure.
fn run_backend<E: fmt::Display>(run: impl FnOnce() -> Result<(), E>) -> Result<(), GuiError> {
    match catch_unwind(AssertUnwindSafe(run)) {
        Ok(result) => result.map_err(|error| GuiError::Backend(error.to_string())),
        Err(payload) => {
            let detail = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("non-text panic payload");
            Err(GuiError::Backend(format!(
                "native GUI panicked: {detail}; this backend requires the main thread and permits only one event loop per process"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Clone, Copy, Debug)]
    enum EndMessage {
        Close,
        Finish,
        Fail,
    }
    #[derive(Default)]
    struct LifecycleProgram {
        updates: usize,
        tick_queries: Cell<usize>,
        outcome: Option<bool>,
    }
    impl ProgramWithOutcome for LifecycleProgram {
        type Message = EndMessage;
        type Outcome = bool;

        fn update(&mut self, message: EndMessage) -> Command {
            self.updates += 1;
            match message {
                EndMessage::Close => Command::CloseWindow,
                EndMessage::Finish => {
                    self.outcome = Some(true);
                    Command::None
                }
                EndMessage::Fail => Command::OpenPath("fixture".into()),
            }
        }
        fn program_outcome(&self) -> Option<bool> {
            self.outcome
        }
        fn tick_message(&self) -> Option<EndMessage> {
            self.tick_queries.set(self.tick_queries.get() + 1);
            Some(EndMessage::Finish)
        }
    }
    impl NativeProgram for LifecycleProgram {
        fn view_native(&self) -> Element<'static, EndMessage> {
            iced::widget::text("fixture").into()
        }
    }

    #[test]
    fn close_outcome_and_error_freeze_queued_business_messages() {
        for ending in [EndMessage::Close, EndMessage::Finish, EndMessage::Fail] {
            let state = Arc::new(Mutex::new(SharedProgram {
                controls: LifecycleProgram::default(),
                error: None,
                closing: false,
            }));
            let (mut window, _) = NativeWindow::new(WindowFlags {
                state: state.clone(),
                title: "fixture".into(),
            });
            let _ = window.update(WindowMessage::Control(ending));
            for queued in [
                WindowMessage::Control(EndMessage::Finish),
                WindowMessage::Control(EndMessage::Fail),
                WindowMessage::Tick,
            ] {
                assert!(window.update(queued).actions().is_empty());
            }
            assert!(window.subscription().into_recipes().is_empty());
            let state = lock_program(&state);
            assert!(state.closing);
            assert_eq!(state.controls.updates, 1);
            assert_eq!(state.controls.tick_queries.get(), 0);
            assert_eq!(
                state.controls.outcome,
                matches!(ending, EndMessage::Finish).then_some(true)
            );
            let expected_error = matches!(ending, EndMessage::Fail)
                .then(|| GuiError::UnsupportedCommand(Command::OpenPath("fixture".into())));
            assert_eq!(state.error, expected_error);
        }
    }

    struct UnsupportedProgram;
    impl ProgramWithOutcome for UnsupportedProgram {
        type Message = ();
        type Outcome = ();
        fn update(&mut self, _: ()) -> Command {
            Command::OpenPath("fixture".into())
        }
        fn program_outcome(&self) -> Option<()> {
            None
        }
    }
    impl NativeProgram for UnsupportedProgram {
        fn view_native(&self) -> Element<'static, ()> {
            iced::widget::text("fixture").into()
        }
    }
    struct CompletedProgram;
    impl ProgramWithOutcome for CompletedProgram {
        type Message = ();
        type Outcome = bool;
        fn update(&mut self, _: ()) -> Command {
            panic!("completed program must not update")
        }
        fn program_outcome(&self) -> Option<bool> {
            Some(true)
        }
        fn tick_message(&self) -> Option<()> {
            panic!("completed program must not request ticks")
        }
    }
    impl NativeProgram for CompletedProgram {
        fn view_native(&self) -> Element<'static, ()> {
            panic!("completed program must not render")
        }
    }
    #[test]
    fn completed_program_returns_without_a_window_or_business_ticks() {
        assert_eq!(
            run_native_gui(CompletedProgram, Options::new("unused", 0, 0)),
            Ok(Some(true))
        );
        let state = Arc::new(Mutex::new(SharedProgram {
            controls: CompletedProgram,
            error: None,
            closing: false,
        }));
        let (mut window, _) = NativeWindow::new(WindowFlags {
            state,
            title: "fixture".into(),
        });
        let _ = window.subscription();
        let _ = window.update(WindowMessage::Tick);
    }
    #[test]
    fn native_window_retains_consumer_failure_for_the_caller() {
        let state = Arc::new(Mutex::new(SharedProgram {
            controls: UnsupportedProgram,
            error: None,
            closing: false,
        }));
        let (mut window, _) = NativeWindow::new(WindowFlags {
            state: state.clone(),
            title: "fixture".into(),
        });
        let _ = window.update(WindowMessage::Control(()));
        assert!(matches!(
            lock_program(&state).error,
            Some(GuiError::UnsupportedCommand(Command::OpenPath(_)))
        ));
    }
    #[test]
    fn backend_initialization_panics_become_diagnostic_errors() {
        for detail in [
            "Create event loop: RecreationAttempt",
            "event loop must be created on the main thread",
        ] {
            let error = run_backend::<GuiError>(|| panic!("{detail}")).unwrap_err();
            assert!(
                matches!(error, GuiError::Backend(message) if message.contains(detail)
                && message.contains("main thread") && message.contains("one event loop"))
            );
        }
        assert_eq!(
            run_backend(|| Err("fixture error")),
            Err(GuiError::Backend("fixture error".into()))
        );
        assert_eq!(run_backend::<GuiError>(|| Ok(())), Ok(()));
    }
    #[test]
    fn unsupported_commands_are_recoverable_errors() {
        let command = Command::OpenUrl("https://example.invalid".into());
        assert!(
            matches!(platform_command::<()>(command.clone()), Err(GuiError::UnsupportedCommand(actual)) if actual == command)
        );
        assert!(platform_command::<()>(Command::None).is_ok());
        assert!(platform_command::<()>(Command::CopyToClipboard("fixture".into())).is_ok());
        assert!(matches!(
            platform_command::<()>(Command::CloseWindow),
            Ok(NativeAction::Close)
        ));
    }
}
