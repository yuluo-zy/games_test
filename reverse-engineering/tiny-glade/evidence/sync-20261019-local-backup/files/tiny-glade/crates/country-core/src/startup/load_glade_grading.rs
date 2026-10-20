//! Glade 分级 LUT 载入（B `startup::load_glade_grading` @0x1408bf9c0）。
//!
//! 证据（R10 契约 §2.3，O）：
//! * batch2/OnEnter(Game)；输入 `ResMut<GradingLut>`、`Res<ActiveSession>`
//!   （assets root、is_halloween 判别）。
//! * 读 `grading.json`（12 字符，相对会话 assets root）。
//! * LUT 名 `RoastMeat_Cinema_709_1`（0x142aa0e00）与 `default`；
//!   路径模板 `assets/luts/<name>.png`（0x142ae6bef / `.png`）。
//!
//! I：halloween 分支选 `RoastMeat_Cinema_709_1`，否则 `default`。U：grading.json
//! 的 schema；`GradingLut` 字段（属 resources/grading_lut 域）。
use std::path::{Path, PathBuf};

pub const GRADING_JSON: &str = "grading.json";
pub const DEFAULT_GRADING: &str = "default";
pub const HALLOWEEN_GRADING: &str = "RoastMeat_Cinema_709_1";
pub const LUT_DIRECTORY: &str = "assets/luts/";
pub const LUT_EXTENSION: &str = ".png";

/// O：`assets/luts/<name>.png`。
pub fn grading_lut_path(name: &str) -> String {
    format!("{LUT_DIRECTORY}{name}{LUT_EXTENSION}")
}

/// O：`<assets_root>/grading.json`。
pub fn grading_json_path(assets_root: impl AsRef<Path>) -> PathBuf {
    assets_root.as_ref().join(GRADING_JSON)
}

/// I：halloween → `RoastMeat_Cinema_709_1`，否则 `default`。
pub fn select_grading_lut(is_halloween: bool) -> &'static str {
    if is_halloween {
        HALLOWEEN_GRADING
    } else {
        DEFAULT_GRADING
    }
}

/// 宿主 LUT 装载面：`name` 是 LUT 名，宿主按 `assets/luts/<name>.png` 解码。
pub trait GradingHost {
    type Output;
    fn load_lut(&mut self, name: &str) -> Result<Self::Output, String>;
}

/// 文件外壳：读取真实 grading.json 后选择 LUT。
pub fn load_glade_grading<H: GradingHost>(
    host: &mut H,
    assets_root: impl AsRef<Path>,
    is_halloween: bool,
) -> Result<H::Output, String> {
    let path = grading_json_path(&assets_root);
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    load_glade_grading_from_text(host, &text, is_halloween)
}

/// U：grading.json schema 未恢复；只保留“非空即视为有效配置”的读取语义。
pub fn load_glade_grading_from_text<H: GradingHost>(
    host: &mut H,
    grading_text: &str,
    is_halloween: bool,
) -> Result<H::Output, String> {
    if grading_text.trim().is_empty() {
        return Err(format!("{GRADING_JSON} is empty"));
    }
    host.load_lut(select_grading_lut(is_halloween))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec(Vec<&'static str>);
    impl GradingHost for Rec {
        type Output = u32;
        fn load_lut(&mut self, name: &str) -> Result<u32, String> {
            let known = if name == HALLOWEEN_GRADING {
                HALLOWEEN_GRADING
            } else {
                DEFAULT_GRADING
            };
            self.0.push(known);
            Ok(self.0.len() as u32)
        }
    }

    #[test]
    fn theme_selection_and_lut_path_match_the_original_names() {
        let mut host = Rec::default();
        assert_eq!(
            load_glade_grading_from_text(&mut host, "{}", true).unwrap(),
            1
        );
        assert_eq!(
            load_glade_grading_from_text(&mut host, "{}", false).unwrap(),
            2
        );
        assert_eq!(host.0, [HALLOWEEN_GRADING, DEFAULT_GRADING]);
        assert_eq!(
            grading_lut_path(DEFAULT_GRADING),
            "assets/luts/default.png"
        );
        assert_eq!(
            grading_json_path("root"),
            PathBuf::from("root/grading.json")
        );
        assert!(load_glade_grading_from_text(&mut Rec::default(), " ", false).is_err());
    }
}
