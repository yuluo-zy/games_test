//! `tiny_glade::dlss_plugin` — DLSS integration plugin.
//!
//! B public symbol
//! `_$LT$tiny_glade..dlss_plugin..plugin_impl..DlssPlugin$u20$as$u20$bevy_app..plugin..Plugin$GT$::build`
//! is at `0x14003bb80`. Fresh disassembly of that address is a single `RET`
//! (the address is shared by several empty trait methods under identical-code
//! folding, e.g. `aftermath::description` and `run_game::Plugin::build`). The
//! recovered `build` is therefore an empty plugin registration; the DLSS SDK is
//! never touched from this path in the B binary.
//!
//! Classification: glue-legitimate (Bevy plugin registration).

use bevy_app::{App, Plugin};

/// B `tiny_glade::dlss_plugin::plugin_impl::DlssPlugin`.
#[derive(Default)]
pub struct DlssPlugin;

impl Plugin for DlssPlugin {
    /// B `build` `0x14003bb80` — empty body.
    fn build(&self, _app: &mut App) {}
}
