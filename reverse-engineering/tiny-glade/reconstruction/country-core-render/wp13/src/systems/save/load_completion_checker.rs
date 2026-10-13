//! `systems::save::load_completion_checker` — 读档完成检查（B 骨架对齐，WP13）。
//!
//! O：`SessionLoadReadiness::is_ok` 0x140833230（is_initial 折叠）、`deny_load`
//! 0x140a9fa70（与 GladeType::is_olden 同址折叠，tag==1 判定）、
//! `is_session_ok_to_load` 0x140a9fa80、`check_load_completion` 0x140a9f810。

use crate::typed_gap::Gap;

/// O：`SessionLoadReadiness`（读档就绪状态；is_ok 0x140833230 与 is_initial 同址
/// 折叠；deny_load 0x140a9fa70 与 GladeType::is_olden 同址折叠 = tag 字节 == 1）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionLoadReadiness {
    Ready,
    DenyLoad,
    /// 未证状态。
    Unknown,
}

impl SessionLoadReadiness {
    /// O：`is_ok` 0x140833230。
    pub fn is_ok(self) -> bool {
        matches!(self, SessionLoadReadiness::Ready)
    }
    /// O：`deny_load` 0x140a9fa70（tag==1 判定）。
    pub fn deny_load(self) -> bool {
        matches!(self, SessionLoadReadiness::DenyLoad)
    }
}

/// O：`is_session_ok_to_load` 0x140a9fa80（读档许可判定；规则 U）。
pub fn is_session_ok_to_load(_session: &Gap) -> bool {
    true
}

/// O：`check_load_completion` 0x140a9f810（待处理命令计数 → 完成判定；
/// 不足时走 `SessionId::save_folder_path` 补查）。
pub fn check_load_completion(_pending: &Gap) -> bool {
    true
}
