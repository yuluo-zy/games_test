//! tg-clutter-audio：system-clutter 与 country-core systems/audio 证据草稿的可编译接入。
//!
//! 来源草稿：reconstruction/clutter/src/*.rs、reconstruction/audio/src/*.rs
//! （clutter-extra-analysis.md / audio-events-analysis.md / audio-extra-analysis.md，B 主基线
//! 反汇编级证据）。
//!
//! 依赖（原锁版本）：bevy_app / bevy_ecs 0.16.0、glam 0.29.3、hashbrown 0.15.2、
//! serde 1.0.200（auto_clutter_edit 序列化）。`utils` / `tabloid` 为游戏私有 crate（原锁
//! 0.1.0，本地无源码），此处以 glue 占位，不还原实现。
//!
//! 编译/接线记录见 evidence/multi-baseline/compile-clutter-audio.json。

pub mod utils;
pub mod tabloid;
pub mod glue;
pub mod clutter;
pub mod audio;
