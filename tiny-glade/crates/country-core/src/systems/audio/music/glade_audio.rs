//! `GladeAudio` — glade 音频描述（B `startup::load_glade_audio::GladeAudio`，
//! 72 B = 9 qword）。
//!
//! 证据：
//! - `GladeAudio::load` @0x1408bf460：读 `{path}/audio.json` → `serde_json::from_str`
//!   → 成功写 9 qword（`param_1[0..8]`）；失败 → `GladeAudio::default` @0x1408bf2c0
//!   回退（[O，decomp-0x1408bf460.txt]）。
//! - **字段名与 72 B 布局由发行资产核实**（2026-10-16，`D:/game/ljxsj_92385/Tiny Glade/
//!   assets/glade/{summer,olden,flowery,autumn,winter}/audio.json` 全 5 主题逐字节读）：
//!   键 `ambience`（String，24 B = 3 qword）+ `music`（Vec<String>，24 B）+
//!   `select_glade_stinger`（Option<String>，24 B）= 9 qword = 72 B。
//!   default@1408bf2c0 writes the None niche, not an empty String.
//! - 5 主题样例：`ambience: event:/Amb/amb_Summer`、`music: [5 × event:/Music/...]`、
//!   `select_glade_stinger: event:/UI/ChooseSummer`。
use serde::{Deserialize, Serialize};

/// 72 B glade 音频描述。字段序 = 发行 `audio.json` 键序（O）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GladeAudio {
    /// `event:/Amb/amb_<Theme>`（native String 24 B）。
    pub ambience: String,
    /// 主题音乐事件（native Vec<String> 24 B；发行文件 5 条）。
    pub music: Vec<String>,
    /// `event:/UI/Choose<Theme>`; native Option<String> uses String's niche.
    pub select_glade_stinger: Option<String>,
}

impl Default for GladeAudio {
    /// Original constants @142aa0d18/142aa0d31 and None @+0x30.
    fn default() -> Self {
        Self {
            ambience: "event:/Amb/Ambience".into(),
            music: vec!["event:/Music/mus_MainMenu".into()],
            select_glade_stinger: None,
        }
    }
}

impl GladeAudio {
    /// 读取主题配置；会话加载器按原作策略在失败时使用默认描述。
    pub fn try_load_path(glade_dir: &std::path::Path) -> anyhow::Result<Self> {
        use anyhow::Context;
        let path = glade_dir.join("audio.json");
        let input = std::fs::File::open(&path)
            .with_context(|| format!("读取音频配置：{}", path.display()))?;
        serde_json::from_reader(std::io::BufReader::new(input))
            .with_context(|| format!("解析音频配置：{}", path.display()))
    }

    /// `GladeAudio::load` @0x1408bf460：读 `{glade_dir}/audio.json`，反序列化；
    /// 读文件或反序列化失败 → default 回退（不 panic，O）。
    pub fn load(glade_dir: &str) -> Self {
        Self::try_load_path(std::path::Path::new(glade_dir)).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_layout_is_three_24_byte_fields() {
        // 72 B = ambience String(24) + music Vec<String>(24) + stinger String(24)。
        assert_eq!(
            std::mem::size_of::<GladeAudio>(),
            72,
            "native 9-qword 布局 = 3 × 24 B 字段"
        );
    }

    #[test]
    fn wire_schema_matches_the_shipped_audio_json() {
        let text = r#"{
            "ambience": "event:/Amb/amb_Olden",
            "music": [
                "event:/Music/Olden/SingleLeaf",
                "event:/Music/Olden/Tiny Olden Glade",
                "event:/Music/Olden/Forgotten Ruins",
                "event:/Music/Olden/Enchanted Forest",
                "event:/Music/Olden/Timberframes"
            ],
            "select_glade_stinger": "event:/UI/ChooseOlden"
        }"#;
        let parsed: GladeAudio = serde_json::from_str(text).unwrap();
        assert_eq!(parsed.ambience, "event:/Amb/amb_Olden");
        assert_eq!(parsed.music.len(), 5);
        assert_eq!(parsed.music[0], "event:/Music/Olden/SingleLeaf");
        assert_eq!(parsed.select_glade_stinger.as_deref(), Some("event:/UI/ChooseOlden"));
        assert_eq!(
            serde_json::to_value(&parsed).unwrap(),
            serde_json::json!({
                "ambience": "event:/Amb/amb_Olden",
                "music": [
                    "event:/Music/Olden/SingleLeaf",
                    "event:/Music/Olden/Tiny Olden Glade",
                    "event:/Music/Olden/Forgotten Ruins",
                    "event:/Music/Olden/Enchanted Forest",
                    "event:/Music/Olden/Timberframes"
                ],
                "select_glade_stinger": "event:/UI/ChooseOlden"
            })
        );
    }

    #[test]
    fn load_reads_the_audio_json_and_falls_back_to_default() {
        let dir = tempfile::tempdir().unwrap();
        let glade = dir.path().join("olden");
        std::fs::create_dir_all(&glade).unwrap();
        std::fs::write(
            glade.join("audio.json"),
            r#"{"ambience":"event:/Amb/amb_Olden","music":["event:/Music/Olden/A"],
                "select_glade_stinger":"event:/UI/ChooseOlden"}"#,
        )
        .unwrap();
        let loaded = GladeAudio::load(glade.to_str().unwrap());
        assert_eq!(loaded.ambience, "event:/Amb/amb_Olden");
        assert_eq!(loaded.music, ["event:/Music/Olden/A"]);
        // 缺失目录 → default 回退（O：失败 → 0x1408bf2c0）。
        assert_eq!(
            GladeAudio::load(dir.path().join("missing").to_str().unwrap()),
            GladeAudio::default()
        );
    }
}
