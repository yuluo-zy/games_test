//! `input/action_set.rs` — `DigitalActionState` / `ActionSetAction` /
//! `MainActionSet<A, D>` / `ActionSetStates` / `ActionSetValues` /
//! `DigitalActionSetActionType` (the input action-set state machine).
//!
//! Source: B skeleton `crates/country-core/src/input/action_set.rs`.
//! PDB named entities: `ActionSetAction`, `ActionSetStates`, `ActionSetValues`,
//! `DigitalActionSetActionType`, `DigitalActionState`, `MainActionSet`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `DigitalActionState::{pressed,released,just_pressed,just_released}` —
//!   @0x14093fa70/80/90/c0; `just_pressed` dispatches on a u32 discriminant
//!   (jumptable) ⇒ an enum of 4+ states.
//! - `ActionSetAction` — tag `0` = Analog (carries an 8 B payload @+0x04,
//!   `as_analog` @0x140948800), tag `1` = Digital (state byte @+0x01,
//!   `just_pressed`/`pressed` @0x1409487b0/@0x1409487e0 check `tag==1 &&
//!   inner-state`).
//! - `MainActionSet<Analog,Digital>` and `<Vec2,DigitalActionState>`
//!   instantiations (O); `just_pressed` @0x140946620,
//!   `deconstruct` @0x140946b80 (writes per-action `(action-type u16, value)`
//!   triples of 0xc B into a Vec), `reconstruct` @0x140947c80.
//! - `DigitalActionSetActionType` is the hash key for the per-action state
//!   (`BuildHasher::hash_one<DigitalActionSetActionType>`).
//! - The MainActionSet is (de)serialized with serde_json (`impl$0::serialize`).

#![allow(dead_code, unused_imports)]

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::Gap;

/// `DigitalActionState` (O named entity; state-machine enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum DigitalActionState {
    /// Released (I: variant order U).
    #[default]
    Released,
    /// Pressed (I).
    Pressed,
    /// Just pressed this frame (I).
    JustPressed,
    /// Just released this frame (I).
    JustReleased,
}

impl DigitalActionState {
    /// `pressed` @0x14093fa70 (O).
    pub fn pressed(self) -> bool {
        matches!(self, DigitalActionState::Pressed | DigitalActionState::JustPressed)
    }
    /// `released` @0x14093fa80 (O).
    pub fn released(self) -> bool {
        matches!(self, DigitalActionState::Released | DigitalActionState::JustReleased)
    }
    /// `just_pressed` @0x14093fa90 (O).
    pub fn just_pressed(self) -> bool {
        matches!(self, DigitalActionState::JustPressed)
    }
    /// `just_released` @0x14093fac0 (O).
    pub fn just_released(self) -> bool {
        matches!(self, DigitalActionState::JustReleased)
    }
}

/// `DigitalActionSetActionType` (O named entity; hash key for action states).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct DigitalActionSetActionType(pub u16);

/// `ActionSetAction` (O named entity). tag 0 = Analog (Vec2 @+0x04), tag 1 =
/// Digital (state byte @+0x01).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionSetAction {
    /// Analog axis value (O: tag 0).
    Analog(Vec2),
    /// Digital state (O: tag 1).
    Digital(DigitalActionState),
}

impl Default for ActionSetAction {
    fn default() -> Self {
        ActionSetAction::Digital(DigitalActionState::Released)
    }
}

impl ActionSetAction {
    /// `just_pressed` @0x1409487b0 (O: tag==1 && inner just-pressed).
    pub fn just_pressed(self) -> bool {
        match self {
            ActionSetAction::Digital(s) => s.just_pressed(),
            ActionSetAction::Analog(_) => false,
        }
    }
    /// `pressed` @0x1409487e0 (O).
    pub fn pressed(self) -> bool {
        match self {
            ActionSetAction::Digital(s) => s.pressed(),
            ActionSetAction::Analog(_) => false,
        }
    }
    /// `as_analog` @0x140948800 (O: Some(Vec2) iff tag 0).
    pub fn as_analog(self) -> Option<Vec2> {
        match self {
            ActionSetAction::Analog(v) => Some(v),
            ActionSetAction::Digital(_) => None,
        }
    }
}

/// `ActionSetStates` (O named entity; resource holding per-action digital
/// states). Layout U.
#[derive(Clone, Debug, Default)]
pub struct ActionSetStates(pub Gap);

/// `ActionSetValues` (O named entity; resource holding per-action analog
/// values). Layout U.
#[derive(Clone, Debug, Default)]
pub struct ActionSetValues(pub Gap);

/// `MainActionSet<A, D>` (O named entity; generic over the analog type A and
/// digital state D). Instantiated as `<Analog,Digital>` and
/// `<Vec2,DigitalActionState>`. Field layout U.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MainActionSet<A, D> {
    /// analog values (O; generic type A).
    pub analog: A,
    /// digital states (O; generic type D).
    pub digital: D,
}

impl<A, D> MainActionSet<A, D> {
    /// `MainActionSet::just_pressed` @0x140946620 (O; per-action dispatch U).
    pub fn just_pressed(&self) -> bool {
        false
    }
}

/// `deconstruct` @0x140946b80 (O): enumerate the set into per-action records
/// of 0xc B (`(action-type u16, value)`) — used to reconcile replay input.
pub fn deconstruct(_set: &MainActionSet<Vec2, DigitalActionState>) -> Vec<[u8; 12]> {
    todo!("recover MainActionSet::deconstruct action-type packing (0x100/0x101/... tags)")
}

/// `reconstruct` @0x140947c80 (O): rebuild the set from per-action records.
pub fn reconstruct(
    _records: &[[u8; 12]],
) -> Result<MainActionSet<Vec2, DigitalActionState>, ()> {
    todo!("recover MainActionSet::reconstruct")
}
