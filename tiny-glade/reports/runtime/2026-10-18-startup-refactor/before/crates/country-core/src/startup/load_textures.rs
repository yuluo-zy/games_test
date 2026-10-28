//! `load_textures.rs` — 启动期纹理白名单（B `startup::load_textures::should_load`，0x1409e6950，汇编逐条核对）。
//!
//! 谓词是一串 `Path::starts_with`（按路径分量比较，0x1412e6680 / 0x140fcb800 展开 `Path::components`）：
//! `logo`、`brush`、`loading`、`menu/pounce_light_logo`、`menu/heart`、`menu/noknok_logo`、`menu/fmod_logo`、`blue_noise`
//! （串池 0x142ac4780 起），任一命中即加载。末两项受 demo 开关（0x143634d90）门控：
//! 非 demo 看 `default_loading_background`（0x142ac47d8，长 0x1a），demo 看 `default_loading_background_demo`
//! （0x142ac47f2，长 0x1f）；另一项永远不检查。
use std::path::Path;

/// 无条件加载的前缀（O：串池 0x142ac4780..0x142ac47d8，顺序即检查顺序）。
pub const ALWAYS_LOADED_PREFIXES: [&str; 8] = [
    "logo",
    "brush",
    "loading",
    "menu/pounce_light_logo",
    "menu/heart",
    "menu/noknok_logo",
    "menu/fmod_logo",
    "blue_noise",
];

/// 非 demo 时的加载背景前缀（O：0x142ac47d8）。
pub const LOADING_BACKGROUND: &str = "default_loading_background";
/// demo 时的加载背景前缀（O：0x142ac47f2）。
pub const LOADING_BACKGROUND_DEMO: &str = "default_loading_background_demo";

/// `should_load`（0x1409e6950）。`demo` 是进程级开关 `0x143634d90`（`tiny_glade::main` 恒写 0）。
pub fn should_load(path: &Path, demo: bool) -> bool {
    if ALWAYS_LOADED_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
    {
        return true;
    }
    let background = if demo {
        LOADING_BACKGROUND_DEMO
    } else {
        LOADING_BACKGROUND
    };
    path.starts_with(background)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_lengths_match_the_original_literals() {
        let lens: Vec<_> = ALWAYS_LOADED_PREFIXES.iter().map(|p| p.len()).collect();
        assert_eq!(lens, [4, 5, 7, 0x16, 10, 0x10, 0xe, 10]);
        assert_eq!(LOADING_BACKGROUND.len(), 0x1a);
        assert_eq!(LOADING_BACKGROUND_DEMO.len(), 0x1f);
    }

    #[test]
    fn always_loaded_prefixes_match_by_path_components() {
        for p in [
            "logo",
            "logo/small",
            "brush/round",
            "loading/spinner",
            "menu/heart",
            "menu/fmod_logo/x",
            "blue_noise",
        ] {
            assert!(should_load(Path::new(p), false), "{p}");
            assert!(should_load(Path::new(p), true), "{p}");
        }
    }

    #[test]
    fn component_comparison_rejects_partial_names() {
        for p in [
            "logos",
            "logo.png",
            "brushes",
            "menu",
            "menu/hearts",
            "menu/other",
            "terrain/rock",
        ] {
            assert!(!should_load(Path::new(p), false), "{p}");
        }
    }

    #[test]
    fn loading_background_depends_on_the_demo_flag() {
        let normal = Path::new("default_loading_background");
        let demo = Path::new("default_loading_background_demo");
        assert!(should_load(normal, false));
        assert!(!should_load(normal, true));
        assert!(should_load(demo, true));
        assert!(!should_load(demo, false));
    }
}
