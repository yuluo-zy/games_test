//! 完整输入录制的版本化文件协议。运行时帧和持久化帧使用同一语义结构。
use super::{action_set::MainActionSet, input_stream::InputStream};
use anyhow::{Context, Result};
use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Read, Write},
    path::Path,
};

// 快照改用实际工具栏和 AppMode，并记录捕获锚点；旧版缺少恢复所需的数据。
// The current versioned host format gains the native per-frame depth values.
// Reject old incomplete recordings instead of silently filling those values.
pub const RECORDING_FORMAT_VERSION: u32 = 3;
const MAX_RECORDING_FILE_BYTES: u64 = 64 * 1024 * 1024;

/// The three values copied by original read_mouse_depth_value_from_gpu into
/// the actual input frame (+48/+4c/+58). This is a semantic host projection,
/// not a native208-byte frame ABI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MouseDepthReplayFrame {
    pub depth: f32,
    pub mouse_position: Vec3,
    pub center_position: Vec3,
}
impl MouseDepthReplayFrame {
    pub fn is_finite(self) -> bool {
        self.depth.is_finite()
            && self.mouse_position.is_finite()
            && self.center_position.is_finite()
    }
}

/// 动作保留按下沿和连发计时；游标是消费动作前的位置。
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputFrame {
    pub actions: MainActionSet,
    pub cursor_position: [u32; 2],
    pub screen_cursor_position: Vec2,
    pub captured_cursor_position: Option<Vec2>,
    pub time_delta_seconds: f32,
    pub mouse_depth: MouseDepthReplayFrame,
}
impl InputFrame {
    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(self.actions.is_finite(), "录制动作包含非有限值");
        anyhow::ensure!(self.mouse_depth.is_finite(), "录制深度值包含非有限值");
        anyhow::ensure!(
            self.screen_cursor_position.is_finite(),
            "录制游标包含非有限值"
        );
        anyhow::ensure!(
            self.captured_cursor_position
                .is_none_or(|position| position.is_finite()),
            "录制捕获锚点包含非有限值"
        );
        anyhow::ensure!(
            core::time::Duration::try_from_secs_f32(self.time_delta_seconds).is_ok(),
            "录制帧时间无效或超出时钟可表示范围"
        );
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordingFile {
    pub format_version: u32,
    pub stream: InputStream<InputFrame>,
}
impl RecordingFile {
    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.format_version == RECORDING_FORMAT_VERSION,
            "不支持的输入录制版本：{}",
            self.format_version
        );
        validate_stream(&self.stream)
    }
}

pub fn validate_stream(stream: &InputStream<InputFrame>) -> Result<()> {
    stream.validate_structure()?;
    let mut total_duration = core::time::Duration::ZERO;
    for (index, frame) in stream.frames().enumerate() {
        frame
            .validate()
            .with_context(|| format!("第 {index} 帧无效"))?;
        let step = core::time::Duration::try_from_secs_f32(frame.time_delta_seconds)?;
        total_duration = total_duration
            .checked_add(step)
            .with_context(|| format!("第 {index} 帧导致录制累计时间溢出"))?;
    }
    Ok(())
}

/// 先完整解析和校验，成功后才交给调用方替换当前状态。
/// 缺少版本号的旧首帧文件不能恢复完整动作，明确拒绝这种有损格式。
pub fn load_recording(path: &Path) -> Result<InputStream<InputFrame>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("无法打开输入录制：{}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(MAX_RECORDING_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .context("读取输入录制失败")?;
    anyhow::ensure!(
        bytes.len() as u64 <= MAX_RECORDING_FILE_BYTES,
        "输入录制超过 64 MiB 限制"
    );
    let recording: RecordingFile = serde_json::from_slice(&bytes)
        .context("输入录制格式无效：需要带 format_version、完整同步点和帧列表的新格式")?;
    recording.validate()?;
    Ok(recording.stream)
}

/// 流式写入同目录临时文件，再原子替换目标；失败时保留原文件。
pub fn save_recording(path: &Path, stream: &InputStream<InputFrame>) -> Result<()> {
    validate_stream(stream)?;
    #[derive(Serialize)]
    struct BorrowedRecording<'a> {
        format_version: u32,
        stream: &'a InputStream<InputFrame>,
    }
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temporary =
        tempfile::NamedTempFile::new_in(parent).context("创建输入录制临时文件失败")?;
    {
        let mut writer = BufWriter::new(temporary.as_file_mut());
        serde_json::to_writer(
            &mut writer,
            &BorrowedRecording {
                format_version: RECORDING_FORMAT_VERSION,
                stream,
            },
        )
        .context("序列化输入录制失败")?;
        writer.flush().context("刷新输入录制失败")?;
    }
    anyhow::ensure!(
        temporary.as_file().metadata()?.len() <= MAX_RECORDING_FILE_BYTES,
        "输入录制超过 64 MiB 限制"
    );
    // Windows 的 rename/persist 不能覆盖已存在目标。先把旧文件移到同目录的
    // 唯一备份，再 persist；persist 失败则恢复备份，保持“失败保留原文件”。
    #[cfg(windows)]
    {
        let backup = if path.exists() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("recording");
            let backup = parent.join(format!(
                ".{name}.{}.tg-rec-backup",
                fastrand::u64(..)
            ));
            std::fs::rename(path, &backup).context("备份输入录制文件失败")?;
            Some(backup)
        } else {
            None
        };
        match temporary.persist(path) {
            Ok(_) => {
                if let Some(backup) = backup {
                    let _ = std::fs::remove_file(backup);
                }
            }
            Err(error) => {
                if let Some(backup) = backup {
                    let _ = std::fs::rename(backup, path);
                }
                return Err(error.error).context("替换输入录制文件失败");
            }
        }
    }
    #[cfg(not(windows))]
    {
        temporary
            .persist(path)
            .map_err(|error| error.error)
            .context("替换输入录制文件失败")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        input::{
            action_set::{DigitalActionPhase, DigitalActionState},
            input_stream::{InputSnapshot, RecordingRetention},
        },
        resources::toolbar::Toolbar,
        systems::{camera_rig::main_camera::MainCamera, mode_manager::AppMode},
    };
    fn stream() -> InputStream<InputFrame> {
        let mut stream = InputStream::new();
        for index in 0..3 {
            let mut frame = InputFrame {
                cursor_position: [index, 25],
                screen_cursor_position: Vec2::new(0.25, 0.75),
                time_delta_seconds: 0.016,
                ..Default::default()
            };
            frame.mouse_depth = MouseDepthReplayFrame {
                depth: 0.125 + index as f32 * 0.125,
                mouse_position: Vec3::new(index as f32, 5.0, 6.0),
                center_position: Vec3::new(7.0, 8.0, index as f32),
            };
            frame.actions.digital[16] = DigitalActionState {
                phase: DigitalActionPhase::JustPressed,
                timer_seconds: 0.0,
            };
            frame.actions.analog[2] = Vec2::new(index as f32, 1.0);
            stream.record_frame(
                frame,
                || {
                    InputSnapshot::capture(
                        &Toolbar::default(),
                        AppMode::default(),
                        &MainCamera::new(1.0),
                    )
                },
                RecordingRetention::All,
            );
        }
        stream
    }
    #[test]
    fn file_round_trip_keeps_every_frame_cursor_and_action() {
        let stream = stream();
        let file = tempfile::NamedTempFile::new().unwrap();
        save_recording(file.path(), &stream).unwrap();
        assert_eq!(load_recording(file.path()).unwrap(), stream);
        assert_eq!(load_recording(file.path()).unwrap().frame_count(), 3);
    }
    #[test]
    fn incomplete_depth_recordings_are_rejected_instead_of_filled() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut value = serde_json::to_value(RecordingFile {
            format_version: RECORDING_FORMAT_VERSION,
            stream: stream(),
        })
        .unwrap();
        // The versioned format requires the three real native replay values.
        let frames = value["stream"]["frames"].as_array_mut().unwrap();
        frames[0].as_object_mut().unwrap().remove("mouse_depth");
        std::fs::write(file.path(), serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(load_recording(file.path()).is_err());
    }
    #[test]
    fn invalid_version_and_sync_indices_are_rejected() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut json = serde_json::to_value(RecordingFile {
            format_version: RECORDING_FORMAT_VERSION,
            stream: stream(),
        })
        .unwrap();
        json["stream"]["sync_points"][0]["frame_index"] = 99.into();
        std::fs::write(file.path(), serde_json::to_vec(&json).unwrap()).unwrap();
        assert!(load_recording(file.path()).is_err());
        json["stream"]["sync_points"][0]["frame_index"] = 0.into();
        json["format_version"] = 99.into();
        std::fs::write(file.path(), serde_json::to_vec(&json).unwrap()).unwrap();
        assert!(load_recording(file.path()).is_err());
    }
    #[test]
    fn invalid_save_does_not_replace_the_previous_file() {
        let file = tempfile::NamedTempFile::new().unwrap();
        save_recording(file.path(), &stream()).unwrap();
        let original = std::fs::read(file.path()).unwrap();
        let mut invalid = stream();
        for invalid_delta in [f32::NAN, f32::INFINITY, -1.0, f32::MAX] {
            invalid.last_frame_mut().unwrap().time_delta_seconds = invalid_delta;
            assert!(save_recording(file.path(), &invalid).is_err());
            assert_eq!(std::fs::read(file.path()).unwrap(), original);
        }
    }

    #[test]
    fn cumulative_duration_overflow_is_rejected_on_save_and_load() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut invalid = stream();
        // 每一帧单独都能转为 Duration，但整段录制无法播放完。
        let snapshot = invalid.sync_points()[0].state.clone();
        invalid.clear();
        for _ in 0..2 {
            invalid.record_frame(
                InputFrame {
                    time_delta_seconds: 1.0e19,
                    ..Default::default()
                },
                || snapshot.clone(),
                RecordingRetention::All,
            );
        }
        save_recording(file.path(), &stream()).unwrap();
        let original = std::fs::read(file.path()).unwrap();
        assert!(save_recording(file.path(), &invalid).is_err());
        assert_eq!(std::fs::read(file.path()).unwrap(), original);
        let bytes = serde_json::to_vec(&RecordingFile {
            format_version: RECORDING_FORMAT_VERSION,
            stream: invalid,
        })
        .unwrap();
        std::fs::write(file.path(), bytes).unwrap();
        assert!(load_recording(file.path())
            .unwrap_err()
            .to_string()
            .contains("累计时间溢出"));
    }

    #[test]
    fn version_one_cannot_be_mistaken_for_a_complete_tool_state() {
        let recording = RecordingFile {
            format_version: 1,
            stream: stream(),
        };
        assert!(recording.validate().is_err());
    }
}
