//! `tiny_glade::gui::gui_debug` — developer/debug window.
//!
//! fresh Ghidra shared 2026-10-17：`do_gui_debug` `0x140147e00`（1,620 B）、
//! `do_freecam_save` `0x1401485c0`（1,081 B）、`date_spinner` `0x140148b40`
//! （352 B）、`do_camera_snapshot_ui` `0x140148cb0`（2,542 B）。PDB 同文件
//! 另有 `render_gpu_allocator_gui`、`do_tod_save_load_ui`、
//! `do_tod_sequence_entry`。
//!
//! `date_spinner` is recovered: the body allocates a `DragValue` closure and
//! calls `egui::widgets::drag_value::impl$1::ui`; `.rdata` at `0x1429d3768`
//! carries the one-shot `0.1` speed (`0x3fb999999999999a`). The spinner's range
//! constants are loaded from `0x1429d3710` (a formatter/range descriptor) and are
//! not resolved to numbers, so the port keeps the recovered speed only.
//!
//! Camera snapshots now use the complete main body plus its two closures
//! (1401453d0 row, 140143040 file controls), preserving the actual core owners.

use country_core::{
    resources::gui::{DebugGui, EguiContextResource},
    systems::{
        camera_rig::{CameraRigParametersCache, main_camera::CameraRigParameters},
        main_camera::{CameraSnapshot, CameraSnapshotCommand, CameraSnapshotInterpolation},
    },
};
use egui::Ui;
use std::path::Path;

/// Original process static 143634b90 (zero-initialized), toggled by
/// do_gui_debug's closure7@1400b6157. This state survives App replacement.
pub static RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Native mutable byte14362c080 is initialized to 1 in B's .data. Its
/// calc_dof consumer@140a5a5b4 reads the byte directly, not through a pointer.
/// The original dynamic debug writer remains separately tracked.
pub static MULTIRES_DOF: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);

/// Exact rendering control retained from the original debug-panel body.
/// The enclosing do_gui_debug UI remains a separately tracked unrecovered body.
pub fn show_ray_tracing_view_control(ui: &mut Ui) {
    use std::sync::atomic::Ordering;
    let mut enabled = RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW.load(Ordering::Relaxed);
    ui.checkbox(&mut enabled, "Show ray tracing view");
    RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW.store(enabled, Ordering::Relaxed);
}

/// B `date_spinner` (`0x140148b40`).
///
/// `ui.add(egui::DragValue::new(value).speed(0.1))`; the original applies a
/// date-range descriptor whose numeric bounds are not recovered.
pub fn date_spinner(ui: &mut Ui, value: &mut u32) -> egui::Response {
    ui.add(egui::DragValue::new(value).speed(0.1))
}

/// B `do_gui_debug` (`0x140147e00`) — 未实现-需重做.
pub fn do_gui_debug() {
    todo!(
        "未实现-需重做: do_gui_debug 0x140147e00 —— egui::Window 体与 \
         DebugGui/gpu allocator/render_gpu_allocator_gui 子面板未逐语句还原"
    );
}

/// B `do_freecam_save` (`0x1401485c0`) — 未实现-需重做.
pub fn do_freecam_save() {
    todo!(
        "未实现-需重做: do_freecam_save 0x1401485c0 —— Button/WidgetText/输入快照 \
         保存流程依赖未落库的 DebugGui 字段"
    );
}

/// B do_camera_snapshot_ui@140148cb0. Buttons write the original DebugGui
/// request; camera_snapshot consumes it after keyboard priority in set9.
pub fn do_camera_snapshot_ui(ui: &mut Ui, debug: &mut DebugGui, snapshots: &mut CameraSnapshot) {
    ui.add(
        egui::Slider::new(&mut debug.camera_snapshot_interpolation_speed, 0.0..=10.0)
            .text("Interpolation Speed"),
    );
    if ui.button("Save Camera Position").clicked() {
        debug.camera_snapshot_command = CameraSnapshotCommand::Insert;
    }
    for index in 0..snapshots.snapshots.len() {
        ui.horizontal(|ui| {
            // The original row is a label, not an invented selection button.
            let marker = if index == snapshots.index { "▶" } else { " " };
            ui.label(format!("{marker}{index}"));
            if ui.button("Replace").clicked() {
                debug.camera_snapshot_command = CameraSnapshotCommand::Overwrite(index);
            }
            if ui.button("Delete").clicked() {
                debug.camera_snapshot_command = CameraSnapshotCommand::Remove(index);
            }
        });
    }
    match snapshots.snapshots.len() {
        0 => {}
        1 => {
            ui.label("Camera snapshot saved.");
            ui.label("Press F8 to resore it.");
        }
        _ => {
            ui.label("Camera snapshot saved.");
            ui.label("Press F8 to switch between snapshots.");
        }
    }
    ui.horizontal(|ui| {
        if ui.button("Save to disk").clicked() {
            let directory = snapshot_dialog_directory();
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("json", &["json"])
                .set_directory(directory)
                .save_file()
            {
                if save_camera_snapshots(&path, snapshots).is_ok() {
                    log::info!(target: "tiny_glade::gui::gui_debug",
                        "Saved camera snapshot to {}", path.display());
                }
            }
        }
        if ui.button("Load from disk").clicked() {
            let directory = snapshot_dialog_directory();
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("json", &["json"])
                .set_directory(directory)
                .pick_file()
            {
                // Native read/parse errors leave the current snapshot owner
                // and pending request intact; cancel does no file operation.
                if load_camera_snapshots(&path, snapshots).is_ok() {
                    debug.camera_snapshot_command = CameraSnapshotCommand::Select(0);
                }
            }
        }
    });
}

fn snapshot_dialog_directory() -> std::path::PathBuf {
    let directory = Path::new("tmp/camera");
    let _ = std::fs::create_dir_all(directory);
    directory.canonicalize().unwrap()
}

/// Native pretty Vec<CameraRigParametersCache>, not the in-memory body record
/// ABI. The original cache serializer skips vertical_fov_degrees.
pub fn save_camera_snapshots(path: &Path, snapshots: &CameraSnapshot) -> std::io::Result<()> {
    let records: Vec<_> = snapshots
        .snapshots
        .iter()
        .map(|p| CameraRigParametersCache {
            yaw: p.yaw,
            pitch: p.pitch,
            position: p.position,
            camera_rig_arm_length: p.camera_rig_arm_length,
            vertical_fov_degrees: p.vertical_fov_degrees,
            pitch_offset: p.pitch_offset,
            yaw_offset: p.yaw_offset,
        })
        .collect();
    let text = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write(path, text)
}

pub fn load_camera_snapshots(path: &Path, snapshots: &mut CameraSnapshot) -> anyhow::Result<()> {
    let records: Vec<CameraRigParametersCache> =
        serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let records = records
        .into_iter()
        .map(|p| CameraRigParameters {
            yaw: p.yaw,
            pitch: p.pitch,
            position: p.position,
            camera_rig_arm_length: p.camera_rig_arm_length,
            vertical_fov_degrees: p.vertical_fov_degrees,
            pitch_offset: p.pitch_offset,
            yaw_offset: p.yaw_offset,
        })
        .collect();
    snapshots.snapshots = records;
    snapshots.interpolation = CameraSnapshotInterpolation::Disabled;
    snapshots.index = 0;
    // Native +1c elapsed is preserved (irrelevant in Disabled).
    Ok(())
}

/// Actual experimental-window entry beside the already recovered debug
/// controls. Renderer owns begin/end egui frame and publishes this context.
pub fn show_camera_snapshot_panel(
    mut debug: bevy_ecs::prelude::ResMut<DebugGui>,
    mut snapshots: bevy_ecs::prelude::ResMut<CameraSnapshot>,
    context: Option<bevy_ecs::prelude::Res<EguiContextResource>>,
) {
    if !debug.experimental_window {
        return;
    }
    let Some(context) = context else {
        return;
    };
    egui::Window::new("(Experimental)").show(&context, |ui| {
        ui.label("Note: it only works with build mode camera.");
        do_camera_snapshot_ui(ui, &mut debug, &mut snapshots);
    });
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    #[test]
    fn native_snapshot_file_schema_roundtrips_and_failed_load_preserves_owner() {
        let path = tempfile::NamedTempFile::new().unwrap();
        let mut snapshots = CameraSnapshot::default();
        let mut record = country_core::systems::camera_rig::MainCamera::new(1.5).store_parameters();
        record.position = glam::Vec3::new(1.0, 2.0, 3.0);
        record.vertical_fov_degrees = 73.0;
        snapshots.snapshots.push(record);
        save_camera_snapshots(path.path(), &snapshots).unwrap();
        let text = std::fs::read_to_string(path.path()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(json[0].as_object().unwrap().len(), 6);
        assert!(json[0].get("vertical_fov_degrees").is_none());
        snapshots.index = 5;
        snapshots.elapsed = 17.0;
        snapshots.interpolation = CameraSnapshotInterpolation::Slow;
        load_camera_snapshots(path.path(), &mut snapshots).unwrap();
        assert_eq!(snapshots.snapshots[0].position, record.position);
        assert_eq!(snapshots.snapshots[0].vertical_fov_degrees, 45.0);
        assert_eq!(snapshots.index, 0);
        assert_eq!(snapshots.elapsed, 17.0);
        assert_eq!(
            snapshots.interpolation,
            CameraSnapshotInterpolation::Disabled
        );
        std::fs::write(path.path(), "invalid json").unwrap();
        assert!(load_camera_snapshots(path.path(), &mut snapshots).is_err());
        assert_eq!(snapshots.snapshots.len(), 1);
        assert_eq!(snapshots.snapshots[0].position, record.position);
    }
}
