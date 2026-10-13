//! B GUI policy interface. Platform window/event implementations belong to iced/winit.
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Options {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    None,
    CloseWindow,
    OpenUrl(String),
    OpenPath(std::path::PathBuf),
    CopyToClipboard(String),
    RemovePackageFile(String),
}
pub trait ProgramWithOutcome {
    type Message;
    type Outcome;
    fn update(&mut self, message: Self::Message) -> Command;
    fn program_outcome(&self) -> Option<Self::Outcome>;
    fn tick_message(&self) -> Option<Self::Message> {
        None
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiError {
    BackendUnavailable,
    Backend(String),
    ClosedWithoutOutcome,
}
impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for GuiError {}
/// A real backend owns window creation, rendering, event delivery and command errors.
pub trait GuiBackend<P: ProgramWithOutcome> {
    fn run(&mut self, program: &mut P, options: &Options) -> Result<(), GuiError>;
}
pub fn run_gui_with_options<P: ProgramWithOutcome>(
    mut controls: P,
    options: Options,
    backend: &mut impl GuiBackend<P>,
) -> Result<Option<P::Outcome>, GuiError> {
    backend.run(&mut controls, &options)?;
    Ok(controls.program_outcome())
}
pub fn run_gui<P: ProgramWithOutcome>(
    controls: P,
    options: Options,
    backend: &mut impl GuiBackend<P>,
) -> Result<Option<P::Outcome>, GuiError> {
    run_gui_with_options(controls, options, backend)
}

#[cfg(feature = "native-gui")]
mod native {
    use super::*;
    use iced::{Application, Element, Subscription, Theme};
    use std::{
        sync::{Arc, Mutex},
        time::{Duration, Instant},
    };
    /// Game controls construct owned widgets; the original fork owns the renderer/event loop.
    pub trait NativeProgram: ProgramWithOutcome + Send + 'static
    where
        Self::Message: Clone + fmt::Debug + Send + 'static,
    {
        fn view_native(&self) -> Element<'static, Self::Message>;
        fn update_native(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
            let command = self.update(message);
            // Window adds one close when a terminal outcome is available.
            if command == Command::CloseWindow && self.program_outcome().is_some() {
                iced::Command::none()
            } else {
                platform_command(command)
            }
        }
    }
    pub fn platform_command<M>(command: Command) -> iced::Command<M> {
        match command {
            Command::CloseWindow => iced::window::close(iced::window::Id::MAIN),
            Command::None => iced::Command::none(),
            other => panic!("NativeProgram must supply the adapter for {other:?}"),
        }
    }
    struct Flags<P> {
        controls: Arc<Mutex<P>>,
        title: String,
    }
    #[derive(Clone, Debug)]
    enum WindowMessage<M> {
        Control(M),
        Tick,
    }
    struct Window<P> {
        controls: Arc<Mutex<P>>,
        title: String,
    }
    impl<P> Application for Window<P>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        type Executor = iced::executor::Default;
        type Message = WindowMessage<P::Message>;
        type Theme = Theme;
        type Flags = Flags<P>;
        fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
            (
                Self {
                    controls: flags.controls,
                    title: flags.title,
                },
                iced::Command::none(),
            )
        }
        fn title(&self) -> String {
            self.title.clone()
        }
        fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
            let mut controls = self.controls.lock().expect("GUI controls poisoned");
            let message = match message {
                WindowMessage::Control(message) => Some(message),
                WindowMessage::Tick => controls.tick_message(),
            };
            let command = message
                .map(|message| controls.update_native(message).map(WindowMessage::Control))
                .unwrap_or_else(iced::Command::none);
            if controls.program_outcome().is_some() {
                iced::Command::batch([command, iced::window::close(iced::window::Id::MAIN)])
            } else {
                command
            }
        }
        fn view(&self) -> Element<'_, Self::Message> {
            self.controls
                .lock()
                .expect("GUI controls poisoned")
                .view_native()
                .map(WindowMessage::Control)
        }
        fn subscription(&self) -> Subscription<Self::Message> {
            let message = self
                .controls
                .lock()
                .expect("GUI controls poisoned")
                .tick_message();
            if message.is_some() {
                iced::time::every(Duration::from_millis(16)).map(|_| WindowMessage::Tick)
            } else {
                Subscription::none()
            }
        }
    }
    fn run<P>(controls: P, options: Options, visible: bool) -> Result<Option<P::Outcome>, GuiError>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        let controls = Arc::new(Mutex::new(controls));
        let mut settings = iced::Settings::with_flags(Flags {
            controls: controls.clone(),
            title: options.title,
        });
        settings.window.size = iced::Size::new(options.width as f32, options.height as f32);
        settings.window.visible = visible;
        Window::<P>::run(settings).map_err(|e| GuiError::Backend(e.to_string()))?;
        let result = controls
            .lock()
            .expect("GUI controls poisoned")
            .program_outcome();
        Ok(result)
    }
    pub fn run_native_gui<P>(controls: P, options: Options) -> Result<Option<P::Outcome>, GuiError>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        run(controls, options, true)
    }
    #[derive(Clone, Debug)]
    pub enum ProbeMessage<M> {
        Control(M),
        Tick,
        Captured(iced::window::Screenshot),
    }
    struct Probe<P> {
        controls: P,
        start: Instant,
        requested: bool,
        image: Option<iced::window::Screenshot>,
    }
    impl<P> ProgramWithOutcome for Probe<P>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        type Message = ProbeMessage<P::Message>;
        type Outcome = iced::window::Screenshot;
        fn update(&mut self, _: Self::Message) -> Command {
            Command::None
        }
        fn program_outcome(&self) -> Option<Self::Outcome> {
            self.image.clone()
        }
        fn tick_message(&self) -> Option<Self::Message> {
            Some(ProbeMessage::Tick)
        }
    }
    impl<P> NativeProgram for Probe<P>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        fn view_native(&self) -> Element<'static, Self::Message> {
            self.controls.view_native().map(ProbeMessage::Control)
        }
        fn update_native(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
            match message {
                ProbeMessage::Control(m) => {
                    self.controls.update_native(m).map(ProbeMessage::Control)
                }
                ProbeMessage::Captured(image) => {
                    self.image = Some(image);
                    iced::Command::none()
                }
                ProbeMessage::Tick => {
                    if self.start.elapsed() > Duration::from_secs(5) {
                        return iced::window::close(iced::window::Id::MAIN);
                    }
                    if !self.requested && self.start.elapsed() > Duration::from_millis(200) {
                        self.requested = true;
                        return iced::window::screenshot(
                            iced::window::Id::MAIN,
                            ProbeMessage::Captured,
                        );
                    }
                    if let Some(message) = self.controls.tick_message() {
                        self.controls
                            .update_native(message)
                            .map(ProbeMessage::Control)
                    } else {
                        iced::Command::none()
                    }
                }
            }
        }
    }
    /// Brief real native window, tiny-skia rendering and screenshot command; no input/IO actions.
    /// Returns an error if capture does not arrive, rather than accepting an empty GUI stub.
    pub fn capture_native_gui<P>(
        controls: P,
        options: Options,
    ) -> Result<iced::window::Screenshot, GuiError>
    where
        P: NativeProgram,
        P::Message: Clone + fmt::Debug + Send + 'static,
    {
        run(
            Probe {
                controls,
                start: Instant::now(),
                requested: false,
                image: None,
            },
            options,
            true,
        )?
        .ok_or(GuiError::ClosedWithoutOutcome)
    }
}
#[cfg(feature = "native-gui")]
pub use native::{NativeProgram, capture_native_gui, platform_command, run_native_gui};

#[cfg(feature = "native-gui")]
pub fn write_native_capture(
    image: &iced::window::Screenshot,
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    let path = path.as_ref();
    std::fs::write(path, image.bytes.as_slice())?;
    let meta = path.with_extension("json");
    std::fs::write(
        meta,
        format!(
            "{{\"width\":{},\"height\":{},\"bytes\":{},\"format\":\"RGBA8 sRGB\"}}",
            image.size.width,
            image.size.height,
            image.bytes.len()
        ),
    )
}
