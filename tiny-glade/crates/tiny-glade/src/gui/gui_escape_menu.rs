//! `tiny_glade::gui::gui_escape_menu` — escape menu.
//!
//! fresh Ghidra shared 2026-10-17（证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`）：
//! `do_gui_escape_menu` `0x1403cf9b0`（3,634 B）、`wishlist_button`
//! `0x1403d07f0`（388 B）、`open_community_server` `0x1403d09d0`（272 B）、
//! `open_newsletter` `0x1403d0af0`（370 B）。
//!
//! ## 已判等：obfstr URL 解码
//! 两个 `open_*` 用 `obfstr::xref::inner` 取编码字节后 XOR：
//! - community：`0x1403cf2e0` → 数据地址 0x1429d1f47，29 B，
//!   密钥 `0x9a50bb85b50c41b8 / 0x8d73efbb292bffb5 / 0x812e7379e01a08d9 /
//!   0xa76feff9 / 0x91`；
//! - newsletter：`0x1403cf310` → 数据地址 0x1429d1f64，68 B，8 个 qword 密钥
//!   + `0x1c5217d1`。
//! 逐字节解得（UTF-8，全部可打印）：
//! - `https://discord.gg/jcae7kejne`
//! - `https://dashboard.mailerlite.com/forms/10395/78076204263933906/share`
//! 随后调用 `SteamOverlayOpenResources::open_overlay_or_browser_to_webpage`
//! （country-core 已恢复，签名 `&str`；原版传 `String`，源级等价）。
//!
//! ## 未实现
//! `do_gui_escape_menu` 的 MEI 节点树（8 节点 / 15 个闭包）与
//! `wishlist_button` 的 `MenuButtonView` 描述符（image 路径、lang 键、
//! linearstep/lerp 常量）仍在闭包环境/`.rdata` 中，未逐项取证。

use country_core::systems::steam::SteamOverlayOpenResources;

/// Discord 社区邀请（`open_community_server` 解码结果）。
pub const COMMUNITY_SERVER_URL: &str = "https://discord.gg/jcae7kejne";

/// 邮件订阅表单（`open_newsletter` 解码结果）。
pub const NEWSLETTER_URL: &str =
    "https://dashboard.mailerlite.com/forms/10395/78076204263933906/share";

/// Steam overlay/browser opener 别名（原 `SteamOverlayOpenResources`）。
pub type SteamOverlay = SteamOverlayOpenResources;

/// B `do_gui_escape_menu` (`0x1403cf9b0`) — 未实现-需重做.
pub fn do_gui_escape_menu() {
    todo!(
        "未实现-需重做: do_gui_escape_menu 0x1403cf9b0 —— MEI 节点树(8 节点)/\
         15 个闭包/EscMenuState 状态机未逐语句还原"
    );
}

/// B `wishlist_button` (`0x1403d07f0`) — 未实现-需重做.
pub fn wishlist_button() {
    todo!(
        "未实现-需重做: wishlist_button 0x1403d07f0 —— MenuButtonView image/lang 键/\
         linearstep 常量未取证"
    );
}

/// B `open_community_server` (`0x1403d09d0`) —— 判等。
pub fn open_community_server(overlay: &mut SteamOverlay) {
    overlay.open_overlay_or_browser_to_webpage(COMMUNITY_SERVER_URL);
}

/// B `open_newsletter` (`0x1403d0af0`) —— 判等。
pub fn open_newsletter(overlay: &mut SteamOverlay) {
    overlay.open_overlay_or_browser_to_webpage(NEWSLETTER_URL);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_plain_utf8_https() {
        assert!(COMMUNITY_SERVER_URL.starts_with("https://"));
        assert!(NEWSLETTER_URL.starts_with("https://"));
        assert!(COMMUNITY_SERVER_URL.is_ascii());
        assert!(NEWSLETTER_URL.is_ascii());
    }
}
