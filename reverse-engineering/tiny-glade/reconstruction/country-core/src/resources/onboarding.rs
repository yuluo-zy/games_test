//! onboarding.rs — 新手指引事件（WP12 facade）。

#![allow(dead_code, unused_imports)]

/// `NotifyHintSystemSnapping`（O 名；事件）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct NotifyHintSystemSnapping;
