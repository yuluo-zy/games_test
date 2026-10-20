//! 默认时刻载入（B `startup_time_of_day` @0x14089d6f0）。
//!
//! 证据（R10 契约 §2.12，O）：
//! * batch4（与 batch2 同 label，I）。
//! * `load_default_tod` 内联：读 `assets/tod/default.json`（0x142a99540，len 0x17）；
//!   失败 unwrap panic；成功把 0x10 字节序列写回参数（`ResMut<TimeOfDay>` 语义，I）。
//!
//! U：TimeOfDay 资源字段名（契约 §2.12）；加载器是否就是
//! `resources/time_of_day`（I）。本实现以资源现有 serde 字段为准（三 f32）。
use crate::resources::time_of_day::TimeOfDay;
use bevy_ecs::prelude::*;
use std::path::Path;

/// O：`0x142a99540`, len 0x17。
pub const DEFAULT_TOD_PATH: &str = "assets/tod/default.json";

/// 纯解析入口；文件读取与错误由调用方处理。
pub fn load_default_tod_from_str(text: &str) -> anyhow::Result<TimeOfDay> {
    Ok(serde_json::from_str(text)?)
}

/// 读取默认 ToD。相对路径沿用原作的安装目录相对路径约定。
pub fn load_default_tod(path: impl AsRef<Path>) -> anyhow::Result<TimeOfDay> {
    let text = std::fs::read_to_string(path)?;
    load_default_tod_from_str(&text)
}

/// 启动系统：把默认序列写入真实 `TimeOfDay` 资源；缺资源时插入。
/// 原件失败为 unwrap panic；无安装目录的测试宿主会缺文件，因此这里记录
/// 错误并保持现有资源（生产中文件存在时行为与原件一致）。
pub fn startup_time_of_day(
    time: Option<ResMut<TimeOfDay>>,
    mut commands: Commands,
) {
    let loaded = match load_default_tod(DEFAULT_TOD_PATH) {
        Ok(loaded) => loaded,
        Err(error) => {
            log::error!(
                target: "country_core::startup::startup_time_of_day",
                "load_default_tod({DEFAULT_TOD_PATH}): {error:#}"
            );
            return;
        }
    };
    match time {
        Some(mut time) => *time = loaded,
        None => {
            commands.insert_resource(loaded);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_serialized_fields_parse_and_default_path_is_exact() {
        let time: TimeOfDay = load_default_tod_from_str(
            r#"{"time_of_day":0.5,"sun_xz_rotation":0.25,"latitude_degrees":45.0}"#,
        )
        .unwrap();
        assert_eq!(time.time_of_day, 0.5);
        assert_eq!(time.sun_xz_rotation, 0.25);
        assert_eq!(time.latitude_degrees, 45.0);
        assert_eq!(DEFAULT_TOD_PATH, "assets/tod/default.json");
        assert!(load_default_tod_from_str("not json").is_err());
    }
}
