//! B 0x1408b5500–0x1408b7681: music ownership, playlist and transition policy.
use crate::resources::audio_instance::PlaybackState;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::Relaxed};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopMode {
    AllowFadeout,
    Immediate,
}

/// Real fallible FMOD operations. Disabled audio is a separate owned variant.
pub trait MusicEvent: Send + Sync + 'static {
    type Error: std::fmt::Debug;
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self, mode: StopMode) -> Result<(), Self::Error>;
    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error>;
    fn paused(&self) -> Result<bool, Self::Error>;
    fn playback_state(&self) -> Result<PlaybackState, Self::Error>;
    fn timeline_position(&self) -> Result<i32, Self::Error>;
    fn install_beat_callback(&mut self) -> Result<(), Self::Error>;
}
pub trait MusicBackend: Send + Sync + 'static {
    type Event: MusicEvent;
    /// None is the original disabled AudioCore branch. Errors propagate.
    fn instantiate(
        &self,
        path: &str,
    ) -> Result<Option<Self::Event>, <Self::Event as MusicEvent>::Error>;
    fn set_global_parameter(
        &self,
        name: &str,
        value: f32,
    ) -> Result<(), <Self::Event as MusicEvent>::Error>;
}
pub enum MusicInstance<E> {
    Disabled { name: String },
    Active { name: String, event: E },
}
impl<E: MusicEvent> MusicInstance<E> {
    pub fn create<B: MusicBackend<Event = E>>(backend: &B, name: &str) -> Self {
        match backend.instantiate(name).unwrap() {
            Some(event) => Self::Active {
                name: name.into(),
                event,
            },
            None => Self::Disabled { name: name.into() },
        }
    }
    pub fn event(&self) -> Option<&E> {
        match self {
            Self::Active { event, .. } => Some(event),
            _ => None,
        }
    }
    pub fn event_mut(&mut self) -> Option<&mut E> {
        match self {
            Self::Active { event, .. } => Some(event),
            _ => None,
        }
    }
    pub fn start(&mut self) {
        if let Some(e) = self.event_mut() {
            e.start().unwrap();
        }
    }
    pub fn stop(&mut self, mode: StopMode) {
        if let Some(e) = self.event_mut() {
            e.stop(mode).unwrap();
        }
    }
    pub fn set_paused(&mut self, paused: bool) {
        if let Some(e) = self.event_mut() {
            e.set_paused(paused).unwrap();
        }
    }
    pub fn start_if_not_playing(&mut self) {
        if let Some(e) = self.event_mut() {
            if e.playback_state().unwrap().should_start() {
                e.start().unwrap();
            }
        }
    }
}

/// Variant names confirmed by native Debug implementation 0x14083fec0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerState {
    SwitchTrack,
    Playing { elapsed_time: f32 },
    PlayingLoopedOnce { elapsed_time: f32, ticks: u64 },
    SwitchAtNextOpportunity,
    Switching { elapsed_time: f32 },
}
pub struct MusicPlayer<E> {
    pub current_track: Option<usize>,
    pub track_switching: MusicInstance<E>,
    pub fade_in: MusicInstance<E>,
    pub photomode: MusicInstance<E>,
    pub first_person: MusicInstance<E>,
    pub track_names: Vec<String>,
    pub tracks: Vec<MusicInstance<E>>,
    pub playlist: Vec<usize>,
    pub state: PlayerState,
    pub last_timeline_position: i32,

}
impl<E: MusicEvent> MusicPlayer<E> {
    pub fn init<B: MusicBackend<Event = E>>(backend: &B) -> Self {
        let track_switching = MusicInstance::create(backend, "snapshot:/Track Switching");
        let photomode = MusicInstance::create(backend, "snapshot:/PhotoMode");
        let first_person = MusicInstance::create(backend, "snapshot:/FirstPerson");
        let fade_in = MusicInstance::create(backend, "snapshot:/MusicFadeIn");
        Self {
            current_track: None,
            track_switching,
            fade_in,
            photomode,
            first_person,
            track_names: Vec::new(),
            tracks: Vec::new(),
            playlist: Vec::new(),
            state: PlayerState::SwitchTrack,
            last_timeline_position: 0,
        }
    }
    /// Native1408b6b10 releases old owners before creating replacement tracks.
    /// Stopping playback belongs to the enclosing session transition.
    /// 字符串切片与 String 共用入口，启动常量无需先分配 Vec<String>。
    pub fn load_tracks<B: MusicBackend<Event = E>, P: AsRef<str>>(
        &mut self,
        backend: &B,
        paths: &[P],
    ) {
        self.tracks.clear();
        self.track_names.clear();
        let mut tracks = Vec::with_capacity(paths.len());
        let mut track_names = Vec::with_capacity(paths.len());
        for path in paths {
            let path = path.as_ref();
            let mut instance = MusicInstance::create(backend, path);
            if let Some(event) = instance.event_mut() {
                if event.install_beat_callback().is_ok() {
                    tracks.push(instance);
                    track_names.push(path.to_owned());
                }
            }
        }
        self.current_track = None;
        self.tracks = tracks;
        self.track_names = track_names;
        self.playlist.clear();
        self.last_timeline_position = 0;
        self.state = PlayerState::SwitchTrack;
    }
    /// Native compares element zero before popping the end, including for one track.
    pub fn switch_track(&mut self) {
        if self.playlist.is_empty() {
            self.playlist = (0..self.tracks.len()).collect();
            fastrand::shuffle(&mut self.playlist);
            if let Some(previous) = self.current_track {
                if self.playlist.first() == Some(&previous) {
                    let middle = self.playlist.len() / 2;
                    self.playlist.swap(0, middle);
                }
            }
        }
        self.current_track = self.playlist.pop();
    }
    pub fn stop_immediately(&mut self) {
        for track in &mut self.tracks {
            track.stop(StopMode::Immediate);
        }
        self.current_track = None;
        self.track_switching.stop(StopMode::Immediate);
    }
    pub fn pause(&mut self) {
        if let Some(track) = self.current_track.and_then(|i| self.tracks.get_mut(i)) {
            track.set_paused(true);
        }
        self.track_switching.set_paused(true);
    }
    pub fn stop_photomode_snapshots(&mut self) {
        self.photomode.stop(StopMode::AllowFadeout);
        self.first_person.stop(StopMode::AllowFadeout);
    }
    pub fn update(&mut self, dt: f32) {
        self.update_with_random(dt, fastrand::f32);
    }
    /// The random supplier is sampled only after crossing the next integer tick.
    pub fn update_with_random(&mut self, dt: f32, mut random: impl FnMut() -> f32) {
        let position = self
            .current_track
            .and_then(|i| self.tracks.get(i))
            .and_then(MusicInstance::event)
            .map_or(0, |e| e.timeline_position().unwrap_or(0))
            .max(0);
        if self.state == PlayerState::SwitchTrack {
            self.last_timeline_position = 0;
            BAR_COUNT.store(0, Relaxed);
            self.switch_track();
            if let Some(track) = self.current_track.and_then(|i| self.tracks.get_mut(i)) {
                self.fade_in.start();
                track.start();
                self.state = PlayerState::Playing { elapsed_time: 0.0 };
            } else {
                log::trace!("No music track available");
            }
        } else {
            let looped = position < self.last_timeline_position;
            if looped {
                BAR_COUNT.store(0, Relaxed);
            }
            self.last_timeline_position = position;
            match &mut self.state {
                PlayerState::Playing { elapsed_time } => {
                    *elapsed_time += dt;
                    if *elapsed_time > 1.5 {
                        self.fade_in.stop(StopMode::AllowFadeout);
                    }
                    if looped {
                        // B1 0x1408b5615：只写 state=2、ticks=0，**不清 +0x11c**
                        // （reviewer 复核；概率 elapsed/120 与采样时机偏移保持）。
                        self.state = PlayerState::PlayingLoopedOnce {
                            elapsed_time: *elapsed_time,
                            ticks: 0,
                        };
                    }
                }
                PlayerState::PlayingLoopedOnce {
                    elapsed_time,
                    ticks,
                } => {
                    *elapsed_time += dt;
                    if *elapsed_time > *ticks as f32 {
                        *ticks = elapsed_time.ceil() as u64;
                        let probability = *elapsed_time / 120.0;
                        if probability * probability > random() {
                            self.state = PlayerState::SwitchAtNextOpportunity;
                        }
                    }
                    self.fade_in.stop(StopMode::AllowFadeout);
                }
                PlayerState::SwitchAtNextOpportunity => {
                    if (looped || BAR_COUNT.load(Relaxed) & 3 == 0)
                        && CURRENT_MUSIC_BEAT.load(Relaxed) == 1
                    {
                        self.state = PlayerState::Switching { elapsed_time: 0.0 };
                        self.track_switching.start();
                    }
                    self.fade_in.stop(StopMode::AllowFadeout);
                }
                PlayerState::Switching { elapsed_time } => {
                    *elapsed_time += dt;
                    let elapsed = *elapsed_time;
                    if elapsed > 18.5 {
                        self.track_switching.stop(StopMode::Immediate);
                    }
                    if elapsed > 19.0 {
                        self.state = PlayerState::SwitchTrack;
                    }
                    self.fade_in.stop(StopMode::AllowFadeout);
                    if elapsed > 18.0 {
                        if let Some(track) = self.current_track.and_then(|i| self.tracks.get_mut(i))
                        {
                            track.stop(StopMode::Immediate);
                        }
                    }
                }
                PlayerState::SwitchTrack => unreachable!(),
            }
        }
        // Native disabled instances also take the unpause-snapshot path.
        if let Some(track) = self.current_track.and_then(|i| self.tracks.get_mut(i)) {
            if track.event().map_or(true, |e| e.paused().unwrap()) {
                track.set_paused(false);
                self.track_switching.set_paused(false);
            }
        }
    }
}

static CURRENT_MUSIC_BEAT: AtomicU64 = AtomicU64::new(0);
static CURRENT_MUSIC_BAR: AtomicU64 = AtomicU64::new(0);
static BAR_COUNT: AtomicU64 = AtomicU64::new(0);
static CURRENT_TEMPO: AtomicU32 = AtomicU32::new(0);
static CURRENT_TIME_UPPER: AtomicU64 = AtomicU64::new(0);
static CURRENT_TIME_LOWER: AtomicU64 = AtomicU64::new(0);
#[derive(Clone, Copy, Debug)]
pub struct TimelineBeat {
    pub bar: i32,
    pub beat: i32,
    pub tempo: f32,
    pub time_signature_upper: i32,
    pub time_signature_lower: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct TimelineSnapshot {
    pub bar: u64,
    pub beat: u64,
    pub bar_count: u64,
    pub tempo: f32,
    pub time_signature_upper: u64,
    pub time_signature_lower: u64,
}
pub fn timeline_snapshot() -> TimelineSnapshot {
    TimelineSnapshot {
        bar: CURRENT_MUSIC_BAR.load(Relaxed),
        beat: CURRENT_MUSIC_BEAT.load(Relaxed),
        bar_count: BAR_COUNT.load(Relaxed),
        tempo: f32::from_bits(CURRENT_TEMPO.load(Relaxed)),
        time_signature_upper: CURRENT_TIME_UPPER.load(Relaxed),
        time_signature_lower: CURRENT_TIME_LOWER.load(Relaxed),
    }
}
/// 0x1408b7620 ignores null properties; increments on changed bars, then takes
/// the unsigned maximum with the sign-extended bar number. Atomics avoid a race
/// between FMOD's mixer callback and the frame consumer. Reviewer 复核：存储顺序
/// beat → tempo → count → upper → lower → **bar 最后写**。
pub fn get_timeline_beat_properties(properties: Option<TimelineBeat>) {
    if let Some(p) = properties {
        let bar = p.bar as i64 as u64;
        let count = BAR_COUNT
            .load(Relaxed)
            .wrapping_add(u64::from(CURRENT_MUSIC_BAR.load(Relaxed) != bar))
            .max(bar);
        CURRENT_MUSIC_BEAT.store(p.beat as i64 as u64, Relaxed);
        CURRENT_TEMPO.store(p.tempo.to_bits(), Relaxed);
        BAR_COUNT.store(count, Relaxed);
        CURRENT_TIME_UPPER.store(p.time_signature_upper as i64 as u64, Relaxed);
        CURRENT_TIME_LOWER.store(p.time_signature_lower as i64 as u64, Relaxed);
        CURRENT_MUSIC_BAR.store(bar, Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        convert::Infallible,
        sync::{Arc, Mutex, atomic::AtomicI32},
    };

    struct Event {
        name: String,
        calls: Arc<Mutex<Vec<String>>>,
        position: Arc<AtomicI32>,
    }
    impl Event {
        fn record(&self, operation: &str) {
            self.calls
                .lock()
                .unwrap()
                .push(format!("{}:{operation}", self.name));
        }
    }
    impl MusicEvent for Event {
        type Error = Infallible;
        fn start(&mut self) -> Result<(), Self::Error> {
            self.record("start");
            Ok(())
        }
        fn stop(&mut self, mode: StopMode) -> Result<(), Self::Error> {
            self.record(match mode {
                StopMode::AllowFadeout => "fade",
                StopMode::Immediate => "stop",
            });
            Ok(())
        }
        fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error> {
            self.record(if paused { "pause" } else { "resume" });
            Ok(())
        }
        fn paused(&self) -> Result<bool, Self::Error> {
            Ok(false)
        }
        fn playback_state(&self) -> Result<PlaybackState, Self::Error> {
            Ok(PlaybackState::Playing)
        }
        fn timeline_position(&self) -> Result<i32, Self::Error> {
            Ok(self.position.load(Relaxed))
        }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    struct Backend {
        calls: Arc<Mutex<Vec<String>>>,
        position: Arc<AtomicI32>,
    }
    impl MusicBackend for Backend {
        type Event = Event;
        fn instantiate(&self, path: &str) -> Result<Option<Event>, Infallible> {
            Ok(Some(Event {
                name: path.into(),
                calls: self.calls.clone(),
                position: self.position.clone(),
            }))
        }
        fn set_global_parameter(&self, _name: &str, _value: f32) -> Result<(), Infallible> {
            Ok(())
        }
    }
    #[test]
    fn native_transition_thresholds_preserve_command_order() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let position = Arc::new(AtomicI32::new(0));
        let backend = Backend {
            calls: calls.clone(),
            position: position.clone(),
        };
        let mut player = MusicPlayer::init(&backend);
        player.load_tracks(&backend, &["track"]);
        let take = || std::mem::take(&mut *calls.lock().unwrap());
        player.update_with_random(100.0, || panic!("not sampled before first loop"));
        assert_eq!(player.state, PlayerState::Playing { elapsed_time: 0.0 });
        assert_eq!(take(), ["snapshot:/MusicFadeIn:start", "track:start"]);
        position.store(100, Relaxed);
        player.update_with_random(1.5, || panic!("not sampled"));
        assert!(take().is_empty());
        position.store(0, Relaxed);
        player.update_with_random(0.1, || panic!("not sampled on loop transition"));
        assert_eq!(
            player.state,
            PlayerState::PlayingLoopedOnce {
                // B1 0x1408b5615 不清 +0x11c：1.5 + 0.1 保持（reviewer 复核）。
                elapsed_time: 1.6,
                ticks: 0
            }
        );
        assert_eq!(take(), ["snapshot:/MusicFadeIn:fade"]);
        let mut sampled = 0;
        player.update_with_random(1.0, || {
            sampled += 1;
            0.0
        });
        assert_eq!(sampled, 1);
        assert_eq!(player.state, PlayerState::SwitchAtNextOpportunity);
        assert_eq!(take(), ["snapshot:/MusicFadeIn:fade"]);
        get_timeline_beat_properties(Some(TimelineBeat {
            bar: 4,
            beat: 1,
            tempo: 120.0,
            time_signature_upper: 4,
            time_signature_lower: 4,
        }));
        player.update_with_random(10.0, || panic!("wait state must not sample"));
        assert_eq!(player.state, PlayerState::Switching { elapsed_time: 0.0 });
        assert_eq!(
            take(),
            [
                "snapshot:/Track Switching:start",
                "snapshot:/MusicFadeIn:fade"
            ]
        );
        player.update_with_random(18.0, || unreachable!());
        assert_eq!(take(), ["snapshot:/MusicFadeIn:fade"]);
        player.update_with_random(0.5, || unreachable!());
        assert_eq!(take(), ["snapshot:/MusicFadeIn:fade", "track:stop"]);
        player.update_with_random(0.5, || unreachable!());
        assert_eq!(player.state, PlayerState::Switching { elapsed_time: 19.0 });
        assert_eq!(
            take(),
            [
                "snapshot:/Track Switching:stop",
                "snapshot:/MusicFadeIn:fade",
                "track:stop"
            ]
        );
        player.update_with_random(0.01, || unreachable!());
        assert_eq!(player.state, PlayerState::SwitchTrack);
        assert_eq!(
            take(),
            [
                "snapshot:/Track Switching:stop",
                "snapshot:/MusicFadeIn:fade",
                "track:stop"
            ]
        );
    }
}
