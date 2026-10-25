//! systems/audio/music/music_player.rs — 音乐播放器（MusicPlayer）。
//!
//! PDB 骨架（O 名）：`MusicPlayer::{init, update, load_tracks, switch_track, current_track,
//! probabilistic_switch_track, pause, stop_immediately, stop_photomode_snapshots,
//! get_timeline_beat_properties, trigger_fpv_snapshot,
//! trigger_photomode_minus_fpv_snapshot}`、`generate_track_playlist`（fastrand，O imports）。
//! 反汇编证据（O，audio-events-analysis §6）：`update`（0x1408b5500）状态机跳表 0x142a9fa34，
//! 字段 `state@+0x118、last_position@+0x128、time_acc@+0x11c、event_state@+0x40、fmod_event@+0x48、
//! track Vec@+0xf0/+0xf8`（I）；`init`（0x1408b63c0）行 321 3×instantiate_audio
//! （`event:/Music/mus_GladePreview`×2 + `event:/Amb/GladePreview`）；曲库
//! `event:/Music/Halloween/{Tiny,Single,Forgotten,Enchanted}`；快照 `snapshot:/Track` 等。
//! FMOD/libfmod 全部边界。

#![allow(dead_code, unused_imports)]

use crate::resources::audio_resources::{AudioCore, AudioInstance};
use crate::typed_gap::Gap;

/// 曲目（O 名；stride 0x30 条目，`[+0]==1` 启用，I）。
#[derive(Debug, Default)]
pub struct MusicTrack {
    /// FMOD 事件实例（U）。
    pub _event: Gap,
    /// 事件名（I）。
    pub name: String,
    /// 其余（U）。
    pub _gap: Gap,
}

/// 音乐播放器（O 名；字段偏移 I）。
#[derive(Debug, Default)]
pub struct MusicPlayer {
    /// 曲目列表（O：Vec@+0xf0/+0xf8）。
    pub tracks: Vec<MusicTrack>,
    /// 状态机 `state@+0x118`（I）。
    pub state: u8,
    /// 上次位置 `@+0x128`、时间累加 `@+0x11c`（I）。
    pub _timing: Gap,
    /// 其余（U）。
    pub _gap: Gap,
}

impl MusicPlayer {
    /// `init`（0x1408b63c0，O）：3×instantiate_audio（mus_GladePreview ×2 + Amb/GladePreview）→ start。
    pub fn init(&mut self, core: &AudioCore) {
        for name in [
            "event:/Music/mus_GladePreview",
            "event:/Music/mus_GladePreview",
            "event:/Amb/GladePreview",
        ] {
            if let Ok(inst) = core.instantiate_audio(name) {
                inst.start(); // libfmod 边界（O）。
                self.tracks.push(MusicTrack {
                    _event: Gap,
                    name: name.to_string(),
                    _gap: Gap,
                });
            }
        }
    }

    /// `load_tracks`（0x1408b6b10，O）。
    pub fn load_tracks(&mut self, _playlist: Vec<&'static str>) {
        let _ = self;
    }

    /// `generate_track_playlist`（O；fastrand 随机曲目顺序）。
    pub fn generate_track_playlist() -> Vec<&'static str> {
        vec![]
    }

    /// `update`（0x1408b5500，O）：状态机 → start/stop/set_paused/switch_track。
    pub fn update(&mut self, _dt: f32) {
        let _ = self;
        let _ = _dt;
    }

    /// `switch_track`（0x1408b70d0，O）。
    pub fn switch_track(&mut self, _index: usize) {
        let _ = self;
        let _ = _index;
    }

    /// `probabilistic_switch_track`（O；fastrand）。
    pub fn probabilistic_switch_track(&mut self) {
        let _ = self;
    }

    /// `current_track`（O）。
    pub fn current_track(&self) -> Option<&MusicTrack> {
        self.tracks.first()
    }

    /// `pause`（0x1408b7490，O）。
    pub fn pause(&mut self) {
        let _ = self;
    }

    /// `stop_immediately`（0x1408b72e0，O）。
    pub fn stop_immediately(&mut self) {
        let _ = self;
    }

    /// `stop_photomode_snapshots`（0x1408b6250，O）。
    pub fn stop_photomode_snapshots(&mut self) {
        let _ = self;
    }

    /// `get_timeline_beat_properties`（0x1408b7620，O）。
    pub fn get_timeline_beat_properties(&self) -> (i32, f32) {
        (0, 0.0)
    }

    /// `trigger_fpv_snapshot`（O；`snapshot:/FirstPerson`）。
    pub fn trigger_fpv_snapshot(&mut self) {
        let _ = self;
    }

    /// `trigger_photomode_minus_fpv_snapshot`（O）。
    pub fn trigger_photomode_minus_fpv_snapshot(&mut self) {
        let _ = self;
    }
}
