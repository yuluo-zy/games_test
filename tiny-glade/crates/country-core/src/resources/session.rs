//! B session owners. Metadata field names follow the original serializers.
use super::image_formation::ImageFormationConfig;
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);
impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn save_folder_path(&self) -> PathBuf {
        paths::saves().join(self.0.to_string())
    }
    pub fn backup_folder_path(&self) -> PathBuf {
        paths::backup_saves().join(self.0.to_string())
    }
    pub fn snapshots_folder_path(&self) -> PathBuf {
        self.save_folder_path().join("snapshots")
    }
    /// B 0x140b46300 returns these paths in this order.
    pub fn session_file_list(&self) -> [PathBuf; 3] {
        let folder = self.save_folder_path();
        [
            folder.join("history.json"),
            folder.join("screenshot.jpg"),
            folder.join("meta.json"),
        ]
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
pub enum GladeType {
    #[default]
    Summer = 0,
    Olden = 1,
    Flowery = 2,
    Autumn = 3,
    Winter = 4,
}
impl GladeType {
    pub const ALL: [Self; 5] = [
        Self::Summer,
        Self::Olden,
        Self::Flowery,
        Self::Autumn,
        Self::Winter,
    ];
    pub fn native_tag(self) -> u8 {
        self as u8
    }
    pub fn assets_folder(self) -> &'static str {
        match self {
            Self::Summer => "summer",
            Self::Olden => "olden",
            Self::Flowery => "flowery",
            Self::Autumn => "autumn",
            Self::Winter => "winter",
        }
    }
    pub fn is_olden(self) -> bool {
        self == Self::Olden
    }
    pub fn is_flowery(self) -> bool {
        self == Self::Flowery
    }
    pub fn is_autumn(self) -> bool {
        self == Self::Autumn
    }
    pub fn is_winter(self) -> bool {
        self == Self::Winter
    }
    pub fn audio_id(self) -> f32 {
        [0.0, 3.0, 1.0, 2.0, 4.0][self as usize]
    }
    fn parse<E: serde::de::Error>(name: &str) -> Result<Self, E> {
        match name {
            "Summer" | "summer" => Ok(Self::Summer),
            "Olden" | "olden" => Ok(Self::Olden),
            "Flowery" | "flowery" => Ok(Self::Flowery),
            "Autumn" | "autumn" => Ok(Self::Autumn),
            "Winter" | "winter" => Ok(Self::Winter),
            _ => Err(E::unknown_variant(
                name,
                &["Summer", "Olden", "Flowery", "Autumn", "Winter"],
            )),
        }
    }
}
impl From<u8> for GladeType {
    fn from(tag: u8) -> Self {
        Self::ALL
            .get(tag as usize)
            .copied()
            .expect("invalid native GladeType")
    }
}
impl<'de> Deserialize<'de> for GladeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GladeType;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a GladeType variant")
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                GladeType::parse(value)
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                let (name, variant) = data.variant::<String>()?;
                let theme = GladeType::parse(&name)?;
                serde::de::VariantAccess::unit_variant(variant)?;
                Ok(theme)
            }
        }
        deserializer.deserialize_enum(
            "GladeType",
            &["Summer", "Olden", "Flowery", "Autumn", "Winter"],
            Visitor,
        )
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionMutator {
    Halloween2024 = 0,
    Halloween2025 = 1,
    Xmas2024 = 2,
}

/// Exactly three fields (0x141329ca0, table @0x142c3a890). `state` is an
/// outer file-envelope field. Mutators default empty; image None is omitted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionMeta {
    pub glade: GladeType,
    #[serde(default)]
    pub mutators: Vec<SessionMutator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageFormationConfig>,
}
impl SessionMeta {
    pub fn assets_root(&self) -> PathBuf {
        Path::new("assets/glade").join(self.glade.assets_folder())
    }
    pub fn is_halloween2024(&self) -> bool {
        self.mutators.contains(&SessionMutator::Halloween2024)
    }
    pub fn is_halloween_any(&self) -> bool {
        self.mutators.iter().any(|m| {
            matches!(
                m,
                SessionMutator::Halloween2024 | SessionMutator::Halloween2025
            )
        })
    }
    pub fn is_xmas2024(&self) -> bool {
        self.mutators.contains(&SessionMutator::Xmas2024)
    }
    /// 0x140b46480: read JSON, get `state`, deserialize. Outer version is not checked.
    pub fn try_load_from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
        Ok(serde_json::from_value(
            value.get("state").cloned().unwrap_or_default(),
        )?)
    }
    pub fn write_to_disk(&self, id: SessionId) -> anyhow::Result<()> {
        self.write_to_path(id.save_folder_path().join("meta.json"))
    }
    pub fn write_to_path(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        use std::io::Write;
        let path = path.as_ref();
        let envelope = serde_json::json!({"version": 1, "state": self});
        // 0x1417522e0: ensure parent, compact Value display, write/sync the
        // sibling with extension "tmp", close it, then rename to the target.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let temporary = path.with_extension("tmp");
        let data = envelope.to_string();
        let mut file = std::fs::File::create(&temporary)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(temporary, path)?;
        Ok(())
    }
}

/// Native serializer 0x1403b5920: meta @0, id @0x60, no path/name fields.
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSession {
    pub id: SessionId,
    pub meta: SessionMeta,
}
impl ActiveSession {
    pub fn new(id: SessionId, meta: SessionMeta) -> Self {
        Self { id, meta }
    }
    pub fn assets_root(&self) -> PathBuf {
        self.meta.assets_root()
    }
    pub fn is_olden(&self) -> bool {
        self.meta.glade.is_olden()
    }
    pub fn is_flowery(&self) -> bool {
        self.meta.glade.is_flowery()
    }
    pub fn is_autumn(&self) -> bool {
        self.meta.glade.is_autumn()
    }
    pub fn is_winter(&self) -> bool {
        self.meta.glade.is_winter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_session_metadata_schema_and_file_roundtrip() {
        for (tag, names) in [
            (GladeType::Summer, ["Summer", "summer"]),
            (GladeType::Olden, ["Olden", "olden"]),
            (GladeType::Flowery, ["Flowery", "flowery"]),
            (GladeType::Autumn, ["Autumn", "autumn"]),
            (GladeType::Winter, ["Winter", "winter"]),
        ] {
            for name in names {
                assert_eq!(
                    serde_json::from_value::<GladeType>(name.into()).unwrap(),
                    tag
                );
            }
        }
        assert!(serde_json::from_str::<GladeType>("\"SuMmEr\"").is_err());
        assert!(serde_json::from_str::<GladeType>("\"SUMMER\"").is_err());
        let absent: SessionMeta = serde_json::from_str(r#"{"glade":"summer"}"#).unwrap();
        assert!(absent.mutators.is_empty());
        assert!(absent.image.is_none());
        assert_eq!(
            serde_json::to_value(&absent).unwrap(),
            serde_json::json!({"glade":"Summer", "mutators":[]})
        );

        let meta = SessionMeta {
            glade: GladeType::Winter,
            mutators: vec![SessionMutator::Xmas2024],
            image: Some(ImageFormationConfig {
                name: "warm".into(),
                params_18: [0.8, -0.5, 1.1, 0.9],
                flag_28: 1,
                params_30: [0.2, 0.015],
                word_38: 0.3_f32.to_bits(),
                flag_3c: 1,
                flag_3d: 0,
                ..ImageFormationConfig::default()
            }),
        };
        let value = serde_json::to_value(&meta).unwrap();
        assert_eq!(
            value["image"],
            serde_json::json!({
                "grading": {"bypass_color":true,"current_grading_lut":"warm",
                    "grading_strength":0.8_f32,"ev_shift":-0.5_f32,
                    "contrast":1.1_f32,"saturation":0.9_f32},
                "camera_lens":{"vignette":0.2_f32,"chromatic_aberration":0.015_f32,"barrel_distortion":0.3_f32},
                "artistic":{"lighting_only":true,"tree_see_thru":false}
            })
        );
        let mut missing = value["image"].clone();
        missing["grading"]
            .as_object_mut()
            .unwrap()
            .remove("saturation");
        assert!(serde_json::from_value::<ImageFormationConfig>(missing).is_err());
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("nested/meta.json");
        meta.write_to_path(&path).unwrap();
        assert_eq!(SessionMeta::try_load_from_path(&path).unwrap(), meta);
        assert!(!path.with_extension("tmp").exists());
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(saved.as_object().unwrap().len(), 2);
        assert_eq!(saved["version"], 1);
        assert_eq!(
            serde_json::from_value::<SessionMeta>(saved["state"].clone()).unwrap(),
            meta
        );
        let active = ActiveSession::new(SessionId(Uuid::nil()), meta);
        let value = serde_json::to_value(&active).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 2);
        assert!(value.get("id").is_some() && value.get("meta").is_some());
    }
}
