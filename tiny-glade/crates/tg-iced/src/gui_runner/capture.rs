//! Screenshot-only wrapper: render the supplied view without running its business updates.
use super::{
    Command, GuiError, NativeAction, NativeProgram, Options, ProgramWithOutcome, run_native_gui,
};
use std::{
    fmt,
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
enum CaptureMessage {
    IgnoreInput,
    Tick,
    Captured(iced::window::Screenshot),
}

const WARMUP: Duration = Duration::from_millis(200);
const DEADLINE: Duration = Duration::from_secs(5);

// Active timestamps share the first tick's origin. Terminal states retain the
// first image or timeout so queued responses cannot change the session result.
enum CaptureState {
    Unstarted,
    WarmingUp(Instant),
    Requested(Instant),
    Complete(iced::window::Screenshot),
    TimedOut,
}

#[derive(Debug, PartialEq, Eq)]
enum CaptureEffect {
    Wait,
    RequestScreenshot,
    Close,
}

struct CaptureProgram<P> {
    controls: P,
    state: CaptureState,
}
impl<P> CaptureProgram<P> {
    fn new(controls: P) -> Self {
        Self {
            controls,
            state: CaptureState::Unstarted,
        }
    }

    // Keep clock reads and iced commands outside the state transition so deadline
    // and queued-response behavior can be checked without a window or sleeps.
    fn advance(
        &mut self,
        message: CaptureMessage,
        now: Instant,
    ) -> Result<CaptureEffect, GuiError> {
        match &self.state {
            CaptureState::Complete(_) => return Ok(CaptureEffect::Close),
            CaptureState::TimedOut => return Err(GuiError::CaptureTimedOut),
            CaptureState::WarmingUp(started) | CaptureState::Requested(started)
                if now.saturating_duration_since(*started) >= DEADLINE =>
            {
                self.state = CaptureState::TimedOut;
                return Err(GuiError::CaptureTimedOut);
            }
            _ => {}
        }

        match (&self.state, message) {
            (CaptureState::Unstarted, CaptureMessage::Tick) => {
                self.state = CaptureState::WarmingUp(now);
                Ok(CaptureEffect::Wait)
            }
            (CaptureState::WarmingUp(started), CaptureMessage::Tick)
                if now.saturating_duration_since(*started) >= WARMUP =>
            {
                self.state = CaptureState::Requested(*started);
                Ok(CaptureEffect::RequestScreenshot)
            }
            (CaptureState::Requested(_), CaptureMessage::Captured(image)) => {
                self.state = CaptureState::Complete(image);
                Ok(CaptureEffect::Close)
            }
            _ => Ok(CaptureEffect::Wait),
        }
    }
}
impl<P> ProgramWithOutcome for CaptureProgram<P> {
    type Message = CaptureMessage;
    type Outcome = iced::window::Screenshot;
    fn update(&mut self, _: CaptureMessage) -> Command {
        Command::None
    }
    fn program_outcome(&self) -> Option<Self::Outcome> {
        match &self.state {
            // Screenshot clones share pixel storage through Arc.
            CaptureState::Complete(image) => Some(image.clone()),
            _ => None,
        }
    }
    fn tick_message(&self) -> Option<CaptureMessage> {
        match self.state {
            CaptureState::Complete(_) | CaptureState::TimedOut => None,
            _ => Some(CaptureMessage::Tick),
        }
    }
}
impl<P> NativeProgram for CaptureProgram<P>
where
    P: NativeProgram,
    P::Message: Clone + fmt::Debug + Send + 'static,
{
    fn view_native(&self) -> iced::Element<'static, CaptureMessage> {
        self.controls
            .view_native()
            .map(|_| CaptureMessage::IgnoreInput)
    }
    fn update_native(
        &mut self,
        message: CaptureMessage,
    ) -> Result<NativeAction<CaptureMessage>, GuiError> {
        Ok(match self.advance(message, Instant::now())? {
            CaptureEffect::Wait => NativeAction::Continue(iced::Command::none()),
            CaptureEffect::RequestScreenshot => NativeAction::Continue(iced::window::screenshot(
                iced::window::Id::MAIN,
                CaptureMessage::Captured,
            )),
            CaptureEffect::Close => NativeAction::Close,
        })
    }
}

/// Render a static view and request a screenshot 200 ms after the first capture tick.
/// Business inputs, updates, ticks, and outcome queries are suppressed; the supplied
/// `view_native` must remain free of side effects. This uses the main-thread, one-loop
/// backend described by [`run_native_gui`], and does not wait for business data to load.
///
/// # Errors
/// Returns `CaptureTimedOut` if no image arrives within five seconds of the first
/// capture tick, or `ClosedWithoutOutcome` if the user closes the window first.
/// Native backend errors are propagated unchanged.
pub fn capture_native_gui<P>(
    controls: P,
    options: Options,
) -> Result<iced::window::Screenshot, GuiError>
where
    P: NativeProgram,
    P::Message: Clone + fmt::Debug + Send + 'static,
{
    run_native_gui(CaptureProgram::new(controls), options)?.ok_or(GuiError::ClosedWithoutOutcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct SideEffectSentinel;
    impl ProgramWithOutcome for SideEffectSentinel {
        type Message = ();
        type Outcome = ();
        fn update(&mut self, _: ()) -> Command {
            panic!("business updates must not run")
        }
        fn program_outcome(&self) -> Option<()> {
            panic!("business outcome must not be queried")
        }
        fn tick_message(&self) -> Option<()> {
            panic!("business ticks must not be queried")
        }
    }
    impl NativeProgram for SideEffectSentinel {
        fn view_native(&self) -> iced::Element<'static, ()> {
            iced::widget::text("fixture").into()
        }
        fn update_native(&mut self, _: ()) -> Result<NativeAction<()>, GuiError> {
            panic!("business IO must not run")
        }
    }
    #[test]
    fn capture_ignores_business_input_and_ticks() {
        let mut capture = CaptureProgram::new(SideEffectSentinel);
        let _ = capture.view_native();
        assert!(capture.update_native(CaptureMessage::IgnoreInput).is_ok());
        assert!(capture.update_native(CaptureMessage::Tick).is_ok());
        assert!(capture.program_outcome().is_none());
    }

    fn screenshot(value: u8) -> iced::window::Screenshot {
        iced::window::Screenshot::new(vec![value; 4], iced::Size::new(1, 1))
    }

    #[test]
    fn capture_starts_on_first_tick_requests_once_and_keeps_first_result() {
        let mut capture = CaptureProgram::new(SideEffectSentinel);
        let before_tick = Instant::now();
        let started = before_tick + Duration::from_secs(1);
        assert_eq!(
            capture.advance(CaptureMessage::IgnoreInput, before_tick),
            Ok(CaptureEffect::Wait)
        );
        assert_eq!(
            capture.advance(CaptureMessage::Captured(screenshot(0)), before_tick),
            Ok(CaptureEffect::Wait)
        );
        for (now, expected) in [
            (started, CaptureEffect::Wait),
            (
                started + WARMUP - Duration::from_nanos(1),
                CaptureEffect::Wait,
            ),
            (started + WARMUP, CaptureEffect::RequestScreenshot),
            (
                started + WARMUP + Duration::from_millis(1),
                CaptureEffect::Wait,
            ),
        ] {
            assert_eq!(capture.advance(CaptureMessage::Tick, now), Ok(expected));
        }
        let completed = started + Duration::from_secs(1);
        assert_eq!(
            capture.advance(CaptureMessage::Captured(screenshot(1)), completed),
            Ok(CaptureEffect::Close)
        );
        assert_eq!(
            capture.advance(CaptureMessage::Captured(screenshot(2)), completed),
            Ok(CaptureEffect::Close)
        );
        assert_eq!(
            capture.advance(CaptureMessage::Tick, started + DEADLINE),
            Ok(CaptureEffect::Close)
        );
        assert_eq!(&**capture.program_outcome().unwrap().bytes, &[1; 4]);
        assert!(capture.tick_message().is_none());
    }

    #[test]
    fn capture_deadline_is_terminal_even_if_the_response_arrives_before_the_timeout_tick() {
        for response_before_tick in [false, true] {
            let mut capture = CaptureProgram::new(SideEffectSentinel);
            let started = Instant::now();
            capture.advance(CaptureMessage::Tick, started).unwrap();
            capture
                .advance(CaptureMessage::Tick, started + WARMUP)
                .unwrap();
            let deadline = started + DEADLINE;
            let message = if response_before_tick {
                CaptureMessage::Captured(screenshot(1))
            } else {
                CaptureMessage::Tick
            };
            assert_eq!(
                capture.advance(message, deadline),
                Err(GuiError::CaptureTimedOut)
            );
            assert_eq!(
                capture.advance(CaptureMessage::Captured(screenshot(2)), deadline),
                Err(GuiError::CaptureTimedOut)
            );
            assert!(capture.program_outcome().is_none());
            assert!(capture.tick_message().is_none());
        }
    }
}
