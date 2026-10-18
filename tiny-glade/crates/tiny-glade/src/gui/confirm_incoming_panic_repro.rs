//! `tiny_glade::gui::confirm_incoming_panic_repro` — panic-repro confirmation.
//!
//! B `confirm_incoming_panic_repro` (`0x1400ed280`), fresh decompilation:
//!
//! * state lives at `*arg0` (`byte +0x00`); `+0x40` is an `Option` niche gate
//!   (compared against sentinel `0x8000_0000_0000_0002`) and `+0x48` is a
//!   frame index; the `[arg1+0x10] = [arg1+0x1c]` store is Bevy `ResMut`
//!   change-detection (not a dialog result);
//! * **state == 2**: show the first dialog (Yes/No). Its `show()` result is
//!   discarded and the state becomes 3 — a `No` does *not* return early;
//! * **state == 1** and the `+0x40` niche matches None and
//!   `+0x48 + 1 == frame_count()`: show the second (OK) dialog and set state = 2;
//! * any other state does nothing.
//!
//! Dialog literals (`.rdata`): first title = `😅` (4 B at `0x14293d868`);
//! second title = `Don't panic, it's only a panic` (30 B at `0x14293d86c`);
//! ready description = 151 B at `0x14293d88a`.
//! The first description is `227-byte prefix + message + 69-byte suffix`
//! (`0x14293d720` len 0xe3, `0x14293d803` len 0x45).
//! `message` is the 140-byte fixed text when `sync_point_count() <= 1`, else the
//! 201-byte head + `sync_points - 1` + 33-byte tail. All literals are read from
//! `.rdata` (`0x14293d588`/`0x14293d614`/`0x14293d6dd`).
//!
//! `rfd` is native-dialog glue and is abstracted by [`ConfirmDialog`].

use country_core::input::input_stream::InputStream;
use country_core::systems::backend::input::input_record_replay::InputFrame;

/// Fixed message for dumps with at most one sync point.
pub const NO_SYNC_POINT_MESSAGE: &str = "Unfortunately this panic dump only contains inputs following the last history command,\nso we won't be able to automatically find this panic.";

/// Message head for dumps with multiple sync points.
pub const MULTI_SYNC_POINT_HEAD: &str = "It looks like we have player inputs corresponding to multiple final history commands, so you could try skipping the last few, and replaying their inputs using:\n\n--panic-repro-rewind N\n\nwhere N ∈ [0..";

/// Message tail.
pub const MULTI_SYNC_POINT_TAIL: &str = "] for this particular panic dump.";

/// 227-byte wrapper before the first-dialog message.
pub const DESCRIPTION_PREFIX: &str = "Well, this is awkward... There was no panic.\n\nThis could be system-specific, timing-sensitive, or perhaps the panic happens as a result of earlier operations corrupting game state in a way that history replay doesn't trigger.\n\n";

/// 69-byte wrapper after the first-dialog message.
pub const DESCRIPTION_SUFFIX: &str = "\n\n... unless you've already fixed it, in which case, hooray! 🥳🍻";

/// Title of the first (reproduction warning) dialog (`0x14293d868`, includes
/// the 😅 prefix).
pub const FIRST_DIALOG_TITLE: &str = "😅Don't panic, it's only a panic";

/// Title of the second (ready) dialog (`0x14293d86c`, len 30).
pub const CONFIRM_TITLE: &str = "Don't panic, it's only a panic";

/// Description of the second (ready) dialog (`0x14293d88a`, 151 bytes).
pub const READY_DESCRIPTION: &str = "The next frame should cause a panic 👀\n\nIf you want to use a debugger, now's a good time to attach it 🧑‍🔬.\n\nPress OK and brace yourself! 🤠";

/// Dialog state (`*arg0`).
#[derive(Debug, Default)]
pub struct PanicReproDialogState {
    /// `+0x00`: 1 = ready to show the second dialog, 2 = show the first,
    /// 3 = first dialog already shown.
    pub state: u8,
    /// Result of the first dialog's `show()` (stored through `+0x1c` by the
    /// original machine code).
    pub confirmed: bool,
    /// `+0x40`: sentinel-gated marker for the second dialog (`is_some()`).
    ///
    /// 反汇编与常量 `0x8000_0000_0000_0002` 比较（指针 niche 哨兵，
    /// 不可映射地址 ⇒ 按 None 编码理解）。哨兵语义来自该 Rust fork 的
    /// `Option` niche；若后续证明它是 Some 标记，则本条门极性需翻转。
    pub pending: Option<u8>,
    /// `+0x48`: index compared against `frame_count() + 1`.
    pub input_index: u64,
}

/// Native dialog backend (rfd in the original).
pub trait ConfirmDialog {
    /// `rfd::MessageDialog` with a yes/no button set; returns the `show()`
    /// result.
    fn confirm(&mut self, title: &str, description: &str) -> bool;
    /// `rfd::MessageDialog` with OK only.
    fn show_info(&mut self, title: &str, description: &str);
}

/// Build the inner message from the input ring.
pub fn panic_repro_message(stream: &InputStream<InputFrame>) -> String {
    let sync_points = stream.sync_point_count();
    if sync_points <= 1 {
        NO_SYNC_POINT_MESSAGE.to_string()
    } else {
        format!(
            "{MULTI_SYNC_POINT_HEAD}{}{MULTI_SYNC_POINT_TAIL}",
            sync_points - 1
        )
    }
}

/// Wrap a message in the first dialog's prefix/suffix.
pub fn wrapped_description(message: &str) -> String {
    format!("{DESCRIPTION_PREFIX}{message}{DESCRIPTION_SUFFIX}")
}

/// B `tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro`
/// (`0x1400ed280`).
pub fn confirm_incoming_panic_repro(
    state: &mut PanicReproDialogState,
    stream: &InputStream<InputFrame>,
    dialog: &mut dyn ConfirmDialog,
) {
    match state.state {
        2 => {
            let description = wrapped_description(&panic_repro_message(stream));
            state.confirmed = dialog.confirm(FIRST_DIALOG_TITLE, &description);
            state.state = 3;
        }
        1 => {
            if state.pending.is_some()
                && state.input_index.wrapping_add(1) == stream.frame_count() as u64
            {
                dialog.show_info(CONFIRM_TITLE, READY_DESCRIPTION);
                state.state = 2;
            }
        }
        _ => {}
    }
}
