//! `load_textures.rs` — 启动期纹理白名单（B `startup::load_textures::should_load`，0x1409e6950，汇编逐条核对）。
//!
//! 谓词是一串 `Path::starts_with`（按路径分量比较，0x1412e6680 / 0x140fcb800 展开 `Path::components`）：
//! `logo`、`brush`、`loading`、`menu/pounce_light_logo`、`menu/heart`、`menu/noknok_logo`、`menu/fmod_logo`、`blue_noise`
//! （串池 0x142ac4780 起），任一命中即加载。末两项受 demo 开关（0x143634d90）门控：
//! 非 demo 看 `default_loading_background`（0x142ac47d8，长 0x1a），demo 看 `default_loading_background_demo`
//! （0x142ac47f2，长 0x1f）；另一项永远不检查。
use std::path::Path;

/// `GladeTextureMappings`（原版 `country_core::startup::load_textures::GladeTextureMappings`，
/// `assets/glade/<主题>/textures.json` 的 9 个字符串字段，字段顺序即 0x1409e6b60 里 9 次
/// `create_name_mapping` 的调用顺序；serde 派生按字段名读 JSON）。
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GladeTextureMappings {
    pub ivy_leaf_color: String,
    pub canopy_flowers_depth: String,
    pub leaf_particle_colors: String,
    pub terrain_texture: String,
    pub grass_patch: String,
    pub flower1: String,
    pub lilypad_flower: String,
    pub canopy_color: String,
    pub bark_color_naked: String,
}

impl GladeTextureMappings {
    /// 虚拟名（`glade:` 前缀，O：0x142ac4ace..0x142ac4b62 的 9 个串）与其目标真实纹理，按原版调用顺序。
    pub fn pairs(&self) -> [(&'static str, &str); 9] {
        [
            ("glade:ivy_leaf_color", &self.ivy_leaf_color),
            ("glade:canopy_flowers_depth", &self.canopy_flowers_depth),
            ("glade:leaf_particle_colors", &self.leaf_particle_colors),
            ("glade:terrain_texture", &self.terrain_texture),
            ("glade:grass_patch", &self.grass_patch),
            ("glade:flower1", &self.flower1),
            ("glade:lilypad_flower", &self.lilypad_flower),
            ("glade:canopy_color", &self.canopy_color),
            ("glade:bark_color_naked", &self.bark_color_naked),
        ]
    }
}

/// `load_glade_texture_mappings`（0x1409e6b60，OnEnter(Game) 的 8 成员会话链第 7 个）。
///
/// 机器码：路径 = `assets/glade/<主题>` join `textures.json`（主题串表 summer/olden/flowery/autumn/winter
/// = `ActiveSession::assets_root` 的目录），`fs::read_to_string` → `serde_json::from_str::<GladeTextureMappings>`
/// （失败 expect，文案 "Couldn't load "），再按字段顺序对 `AssetTextureLibrary::create_name_mapping` 调 9 次。
/// 目标纹理不存在由 `create_name_mapping` 自己 panic，不在此处吞掉。
pub fn load_glade_texture_mappings(
    session: bevy_ecs::prelude::Res<crate::resources::session::ActiveSession>,
    mut textures: bevy_ecs::prelude::ResMut<crate::asset_libraries::texture_library::AssetTextureLibrary>,
) -> bevy_ecs::error::Result {
    use anyhow::Context;
    let path = session.assets_root().join("textures.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Couldn't load {}", path.display()))?;
    let mappings: GladeTextureMappings = serde_json::from_str(&text)
        .with_context(|| format!("Couldn't load {}", path.display()))?;
    for (name, target) in mappings.pairs() {
        textures.create_name_mapping(name, target);
    }
    Ok(())
}

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
