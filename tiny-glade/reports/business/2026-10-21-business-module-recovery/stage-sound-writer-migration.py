"""Freeze current sources and stage only the canonical PlayUiSound migration."""
from pathlib import Path
import difflib
import hashlib
import json
import re

root = Path(__file__).resolve().parents[3]
report = Path(__file__).resolve().parent
spec = {
    "tiny-glade/crates/systems/toolbar/src/undo_redo_systems.rs": 1,
    "tiny-glade/crates/systems/toolbar/src/ui_presentation.rs": 1,
    "tiny-glade/crates/systems/toolbar/src/toolbar_systems.rs": 1,
    "tiny-glade/crates/systems/toolbar/src/photomode/ui_slider.rs": 1,
    "tiny-glade/crates/systems/onboarding/src/hint_focused_edit.rs": 1,
    "tiny-glade/crates/systems/onboarding/src/hint_clutter_focus_edit.rs": 1,
    "tiny-glade/crates/systems/onboarding/src/hint_display_system.rs": 3,
    "tiny-glade/crates/systems/glade-border/src/feedback_input_out_of_border.rs": 0,
    "tiny-glade/crates/country-core/src/systems/timelapse_replay.rs": 0,
}
manifest = []
patch = []
for relative, expected_literals in spec.items():
    source = root / relative
    raw = source.read_bytes()
    original = raw.decode("utf-8")
    modified, count = re.subn(
        r"_payload: \[0; 0x10\],\r?\n[ \t]*id: 0,",
        "_payload: [0; 20],", original,
    )
    assert count == expected_literals, (relative, count, expected_literals)
    if relative.endswith("hint_focused_edit.rs") or relative.endswith("hint_clutter_focus_edit.rs"):
        modified = modified.replace(
            "组件类型 U（seam `FocusSubjectByte`）、PlayUiSound id U。",
            "组件类型 U（seam `FocusSubjectByte`）；音效实例 EventId 由 Bevy 管理。",
        )
    if relative.endswith("hint_display_system.rs"):
        modified = modified.replace(
            "/// PlayUiSound 判别值（tag）由 country-core 承载（当前 widget_params::PlayUiSound\r\n/// 为 unit，数值边界登记）。",
            "/// 使用 core 的 24B PlayUiSound 业务载荷；实例 EventId 由 Bevy 管理。",
        ).replace(
            "/// PlayUiSound 判别值（tag）由 country-core 承载（当前 widget_params::PlayUiSound\n/// 为 unit，数值边界登记）。",
            "/// 使用 core 的 24B PlayUiSound 业务载荷；实例 EventId 由 Bevy 管理。",
        )
    variant_calls = 0
    if relative.endswith("timelapse_replay.rs"):
        assert modified.count("PlayUiSound::variant4(0)") == 1
        modified = modified.replace("PlayUiSound::variant4(0)", "PlayUiSound::variant4()")
        variant_calls = 1
    if relative.endswith("feedback_input_out_of_border.rs"):
        modified = modified.replace(
            "//! `{kind u32 @+0=4, 载荷区 @+8..0x17, id u64 @+0x18}`，事件步长 0x20B）。disasm",
            "//! 业务载荷 24B；另加 Bevy EventId 8B，事件步长 0x20B）。disasm",
        ).replace(
            "//! `PlayUiSound::variant4(0)` 的 id@+0x18 由 bevy 记账填充（游戏逻辑无差异）。",
            "//! `PlayUiSound::variant4()` 不包含实例 id；EventId 由 Bevy 另行管理。",
        ).replace(
            "// id = 宿主 EventWriter 计数（bevy 胶水）；crate 以 0 承载。",
            "// id = 宿主 EventWriter 计数（Bevy 另行记账），不进入业务载荷。",
        )
        assert modified.count("PlayUiSound::variant4(0)") == 1
        modified = modified.replace("PlayUiSound::variant4(0)", "PlayUiSound::variant4()")
        variant_calls = 1
    assert modified != original
    # Preserve the source's newline convention. All edits are local token/line
    # changes; no formatter or old whole-file candidate is used.
    newline = "\r\n" if b"\r\n" in raw else "\n"
    modified = modified.replace("\r\n", "\n").replace("\n", newline)
    proposed = report / "proposed" / relative
    proposed.parent.mkdir(parents=True, exist_ok=True)
    encoded = modified.encode("utf-8")
    proposed.write_bytes(encoded)
    patch.extend(difflib.unified_diff(
        original.replace("\r\n", "\n").splitlines(keepends=True),
        modified.replace("\r\n", "\n").splitlines(keepends=True),
        fromfile="a/" + relative, tofile="b/" + relative,
    ))
    manifest.append({
        "source": relative,
        "source_sha256": hashlib.sha256(raw).hexdigest(),
        "proposed": str(proposed.relative_to(root)).replace("\\", "/"),
        "proposed_sha256": hashlib.sha256(encoded).hexdigest(),
        "zero_payload_literals_migrated": count,
        "variant4_calls_migrated": variant_calls,
        "production_modified": False,
    })
(report / "sound-writer-migration.patch").write_text("".join(patch), encoding="utf-8", newline="\n")
(report / "sound-writer-migration.json").write_text(json.dumps({
    "contract": "PlayUiSound { kind: u32, _payload: [u8; 20] }; variant4(); native business size 24B; Bevy EventId excluded",
    "files": manifest,
    "production_modified": False,
    "compiled": False,
    "verified_runtime": False,
    "audio_dispatch": "No production EventReader<PlayUiSound> found; remains open",
}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(json.dumps({"files": len(manifest), "literals": sum(x["zero_payload_literals_migrated"] for x in manifest), "variant4_calls": 2, "production_modified": False}))
