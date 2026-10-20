//! B parser compatibility pass 141471d3c..1414732b3. This is a game policy
//! pass over the canonical owners; it does not duplicate their wire codecs.
use super::{HistorySectionJson, ParsedLoadMetadata};
use crate::resources::{
    history::{
        EditType, color_edit::ColorEdit, terrain_edit::TerrainEdit, wall_edit::WallHistoryEdit,
    },
    save_file::upgrade_window_color_id,
};
use anyhow::Context as _;
use serde_json::Value;
use std::sync::Arc;

/// MarkEnclosure is a removed native tag15. Decode its three required fields
/// with their actual types before replacing it with tag4. True places the
/// shape on the after side, false on the before side (141471fc8..141472072).
pub(super) fn migrate_legacy_enclosure(history: &mut Value) -> anyhow::Result<bool> {
    #[derive(serde::Deserialize)]
    struct MarkEnclosure {
        wall_index: u64,
        enclosure_params: crate::systems::wall::valid_enclosure::ShapeParameters,
        is_valid: bool,
    }
    let mut changed = false;
    if let Some(entries) = history.get_mut("edit_chain").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(edits) = entry.get_mut("edits").and_then(Value::as_array_mut) {
                for edit in edits {
                    let wall = match edit.as_object_mut() {
                        Some(map) if map.contains_key("Wall") => map.get_mut("Wall"),
                        Some(map) => map.get_mut("WallNewSystem"),
                        None => None,
                    };
                    if let Some(groups) = wall.and_then(Value::as_array_mut) {
                        for group in groups {
                            if let Some(edits) = group.as_array_mut() {
                                for edit in edits {
                                    if let Some(payload) = edit.get("MarkEnclosure") {
                                        let old: MarkEnclosure =
                                            serde_json::from_value(payload.clone())
                                                .context("parsing legacy MarkEnclosure")?;
                                        use crate::systems::wall::valid_enclosure::IsEnclosed;
                                        let shape = IsEnclosed::PremadeShape(old.enclosure_params);
                                        let (before, after) = if old.is_valid {
                                            (IsEnclosed::No, shape)
                                        } else {
                                            (shape, IsEnclosed::No)
                                        };
                                        *edit = serde_json::json!({"ChangeIsEnclosedState": {
                                            "wall_index": old.wall_index, "before": before, "after": after
                                        }});
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(changed)
}

/// Original MAXSS/MINSS select the second operand for unordered comparison.
/// Preserve that policy: negative legacy inputs produce NaN, rather than
/// quietly turning into zero through Rust's f32::max/min NaN policy.
fn old_terrain_height(value: f32) -> f32 {
    let value = value.sqrt() * 10.0;
    let lower = if 0.0 > value { 0.0 } else { value };
    if 20.0 < lower { 20.0 } else { lower }
}

/// Returns the wrapper metadata and the native potential-mutation decision.
/// Only that decision changes Identity and invalidates parser snapshot refs;
/// color compatibility flags are independent, even if a mapped ID is equal.
pub(super) fn migrate_history(
    history: &mut HistorySectionJson,
    version: i32,
    legacy_enclosure_changed: bool,
) -> (ParsedLoadMetadata, bool) {
    if version == 27 {
        return (ParsedLoadMetadata::VERSION_27, false);
    }
    let mut changed = version < 16 || legacy_enclosure_changed;
    let mut window_colors = false;
    let mut arrow_colors = false;
    for saved in &mut history.edit_chain {
        let entry = &mut saved.0;
        for edit in &mut entry.edits {
            match edit {
                EditType::Wall(compound) => {
                    for group in &mut Arc::make_mut(compound).groups {
                        for edit in group {
                            if let WallHistoryEdit::StateChange {
                                observed_operation, ..
                            } = edit
                            {
                                let tag =
                                    u32::from_le_bytes(observed_operation[..4].try_into().unwrap());
                                let start = f32::from_le_bytes(
                                    observed_operation[4..8].try_into().unwrap(),
                                );
                                let end =
                                    f32::from_le_bytes(observed_operation[8..].try_into().unwrap());
                                if tag < 2 && start == end {
                                    let start = start - 0.01;
                                    assert!(
                                        start.is_finite(),
                                        "original finite wall operation coordinate"
                                    );
                                    let start = start.clamp(0.0, 1.0);
                                    let end = start + 0.01;
                                    assert!(
                                        end < f32::INFINITY,
                                        "original wall operation upper bound"
                                    );
                                    observed_operation[4..8].copy_from_slice(&start.to_le_bytes());
                                    observed_operation[8..]
                                        .copy_from_slice(&end.min(1.0).to_le_bytes());
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                EditType::Terrain(terrain) => {
                    if let TerrainEdit::NewTerrainStroke { stroke_pts, .. } = Arc::make_mut(terrain)
                    {
                        if stroke_pts.len() == 2 {
                            let a = stroke_pts[0];
                            let b = stroke_pts[1];
                            let dx = a[0] - b[0];
                            let dy = a[1] - b[1];
                            let dz = a[2] - b[2];
                            let distance = (dz * dz + (dy * dy + dx * dx)).sqrt();
                            if distance == 0.0 {
                                stroke_pts[1][0] += 0.001;
                                // Native adds +0 in these lanes as well (signed zero).
                                stroke_pts[1][1] += 0.0;
                                stroke_pts[1][2] += 0.0;
                                changed = true;
                            }
                        }
                    }
                }
                EditType::Color(color) if version < 20 => {
                    if let ColorEdit::Gate { from, to, .. }
                    | ColorEdit::Decorator { from, to, .. } = Arc::make_mut(color)
                    {
                        from.0 = upgrade_window_color_id(from.0);
                        to.0 = upgrade_window_color_id(to.0);
                    }
                }
                _ => {}
            }
        }
        // Original clears the map, retaining its owning Arc, for pre16.
        if version < 16 {
            if let Some(diff) = &mut entry.auxiliary.decorator_diff {
                Arc::make_mut(diff).clear();
            }
            entry.observed_auxiliary_counts[0] = 0;
        }
        // Vec::retain 1412f50f0: Decorator(tag11) always removed, with V5
        // (tag10) and Color::Decorator(target7) removed only before16.
        entry.edits.retain(|edit| {
            let remove = matches!(edit, EditType::Decorator(_))
                || (version < 16 && (matches!(edit, EditType::DecoratorV5(_))
                    || matches!(edit, EditType::Color(color) if matches!(&**color, ColorEdit::Decorator { .. }))));
            changed |= remove;
            !remove
        });
        if let Some(diff) = &mut entry.auxiliary.decorator_diff {
            for bucket in Arc::make_mut(diff).values_mut() {
                for side in [&mut bucket.before, &mut bucket.after] {
                    for state in side.states.values_mut() {
                        // Record key occupies first8B. Native +30 color_id is
                        // the low word of canonical state.u28, not u34 light.
                        let color = state.u28 as u32;
                        if version < 20 && state.ty.wrapping_sub(0x38) < 3 {
                            state.u28 = (state.u28 & !0xffff_ffff)
                                | u64::from(upgrade_window_color_id(color));
                            window_colors = true;
                        }
                        if version < 21 && state.ty == 0x3a {
                            state.u28 = (state.u28 & !0xffff_ffff) | 999;
                            arrow_colors = true;
                        }
                    }
                }
            }
        }
    }
    if version < 17 {
        for saved in &mut history.edit_chain {
            for edit in &mut saved.0.edits {
                if let EditType::Terrain(terrain) = edit {
                    match Arc::make_mut(terrain) {
                        TerrainEdit::NewTerrainStroke { height_ws, .. } => {
                            *height_ws = old_terrain_height(*height_ws);
                            changed = true;
                        }
                        TerrainEdit::EditTerrainStroke {
                            prev_height_ws,
                            new_height_ws,
                            ..
                        } => {
                            *prev_height_ws = old_terrain_height(*prev_height_ws);
                            *new_height_ws = old_terrain_height(*new_height_ws);
                            changed = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if changed {
        history.snapshots.clear();
    }
    (
        ParsedLoadMetadata {
            requires_upgrade_backup: version != 27 || changed,
            upgrade_window_colors: window_colors,
            upgrade_arrow_slit_colors: arrow_colors,
            legacy_version_notice: version < 19,
        },
        changed,
    )
}

/// Native load_game 14146eb4e..14146ee0a, after opening a valid snapshot and
/// before applying any domain state. Work on the loaded owned document, with
/// the canonical typed validators, retaining every other field verbatim.
pub(super) fn migrate_snapshot(
    loaded: &mut crate::systems::save::snapshot::LoadedSnapshotData,
    flags: ParsedLoadMetadata,
) -> anyhow::Result<()> {
    if flags.upgrade_arrow_slit_colors {
        log::info!("Detected old version of arrow slits color IDs. Upgrading...");
    }
    if flags.upgrade_window_colors {
        log::info!("Detected old version of window color IDs. Upgrading...");
    }
    for (name, section) in &mut loaded.sections {
        if *name == "GateColors" && flags.upgrade_window_colors {
            let mut colors: Vec<(usize, crate::resources::color_id::ColorId)> =
                serde_json::from_value(section.clone()).context("upgrading snapshot GateColors")?;
            for (_, color) in &mut colors {
                color.0 = upgrade_window_color_id(color.0);
            }
            *section = serde_json::to_value(colors)?;
        }
        if *name == "Decorators" && (flags.upgrade_window_colors || flags.upgrade_arrow_slit_colors)
        {
            use crate::systems::save::snapshot::{
                DecoratorInfoJson, DecoratorTypeJson, DecoratorsSectionJson,
            };
            // Validate full structure before making any field write, using
            // the same original schema that the snapshot consumer accepts.
            serde_json::from_value::<DecoratorsSectionJson>(section.clone())
                .context("upgrading snapshot Decorators")?;
            for bucket in section["states"]
                .as_array_mut()
                .context("snapshot decorator states")?
            {
                for row in bucket[1]["state"]
                    .as_array_mut()
                    .context("snapshot decorator bucket")?
                {
                    let info: DecoratorInfoJson = serde_json::from_value(row[1].clone())?;
                    let name = match info.ty {
                        DecoratorTypeJson::Name(name) => name,
                        _ => continue,
                    };
                    let mut color = info.color_id;
                    // Original applies bit48 first, then bit40. Mapping999
                    // preserves999, so both flags leave ArrowSlit at999.
                    if flags.upgrade_arrow_slit_colors && name == "ArrowSlit" {
                        color = 999;
                    }
                    if flags.upgrade_window_colors
                        && matches!(
                            name.as_str(),
                            "CottageWindow" | "GothicWindow" | "ArrowSlit"
                        )
                    {
                        color = upgrade_window_color_id(color);
                    }
                    if color != info.color_id {
                        row[1]["color_id"] = serde_json::json!(color);
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        history::{decorator_edit::*, serialized::SavedMultiEdit, transactions::HistoryEntry},
        walls::{decorator::DecoratorId, decorator_storage::DecoratorStorageAddr},
    };
    fn history(edits: Vec<EditType>) -> HistorySectionJson {
        HistorySectionJson {
            chain_index: 3,
            history_start: -1,
            snapshots: vec![serde_json::json!({"chain_index": 0, "snapshot_id": 7})],
            edit_chain: vec![SavedMultiEdit(HistoryEntry::from_edits(edits))],
        }
    }
    fn windows() -> Arc<DecoratorHistoryDiffMap> {
        let state = |ty, color, light| DecoratorHistoryState {
            ty,
            u28: (0x1234_5678u64 << 32) | color,
            u34: light,
            ..Default::default()
        };
        let mut diff = DecoratorStorageHistoryDiff::default();
        diff.before.states.insert(DecoratorId(1), state(56, 1, 6));
        diff.after.states.insert(DecoratorId(1), state(56, 2, 7));
        diff.before.states.insert(DecoratorId(2), state(58, 4, 8));
        diff.after.states.insert(DecoratorId(2), state(58, 5, 9));
        Arc::new(
            [(DecoratorStorageAddr::Terrain, diff)]
                .into_iter()
                .collect(),
        )
    }
    #[test]
    fn compatibility_colors_preserve_seed_and_light_on_both_sides() {
        let mut saved = history(Vec::new());
        saved.edit_chain[0].0.auxiliary.decorator_diff = Some(windows());
        let (flags, changed) = migrate_history(&mut saved, 19, false);
        assert!(flags.upgrade_window_colors && flags.upgrade_arrow_slit_colors);
        assert!(
            !changed,
            "color flags do not set the potential-mutation flag"
        );
        assert_eq!(saved.snapshots.len(), 1);
        let diff = &saved.edit_chain[0]
            .0
            .auxiliary
            .decorator_diff
            .as_ref()
            .unwrap()[&DecoratorStorageAddr::Terrain];
        for (side, color, light) in [(&diff.before, 2u64, 6), (&diff.after, 5u64, 7)] {
            assert_eq!(
                side.states[&DecoratorId(1)].u28,
                (0x1234_5678u64 << 32) | color
            );
            assert_eq!(side.states[&DecoratorId(1)].u34, light);
            assert_eq!(side.states[&DecoratorId(2)].u28 as u32, 999);
        }
        let mut saved = history(Vec::new());
        saved.edit_chain[0].0.auxiliary.decorator_diff = Some(windows());
        let (flags, changed) = migrate_history(&mut saved, 20, false);
        assert!(!flags.upgrade_window_colors && flags.upgrade_arrow_slit_colors);
        assert!(!changed);
        let diff = &saved.edit_chain[0]
            .0
            .auxiliary
            .decorator_diff
            .as_ref()
            .unwrap()[&DecoratorStorageAddr::Terrain];
        assert_eq!(diff.before.states[&DecoratorId(1)].u28 as u32, 1);
    }
    #[test]
    fn legacy_enclosure_keeps_shape_and_direction_for_place_and_remove() {
        use crate::systems::wall::valid_enclosure::{IsEnclosed, Shape, ShapeParameters};
        let shape = ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: glam::Vec2::new(3.0, -2.0),
            radius: 4.0,
            rotation: 0.5,
        }));
        for is_valid in [false, true] {
            let mut value = serde_json::json!({"chain_index": 0, "edit_chain": [{"edits": [{"Wall": [[{
                "MarkEnclosure": {"wall_index": 11, "enclosure_params": shape, "is_valid": is_valid}
            }]]}]}]});
            assert!(migrate_legacy_enclosure(&mut value).unwrap());
            let history: HistorySectionJson = serde_json::from_value(value).unwrap();
            let EditType::Wall(compound) = &history.edit_chain[0].0.edits[0] else {
                panic!()
            };
            let WallHistoryEdit::EnclosureChange {
                wall,
                previous,
                next,
            } = compound.groups[0][0]
            else {
                panic!()
            };
            assert_eq!(wall.0, 11);
            assert_eq!(
                previous,
                if is_valid {
                    IsEnclosed::No
                } else {
                    IsEnclosed::PremadeShape(shape)
                }
            );
            assert_eq!(
                next,
                if is_valid {
                    IsEnclosed::PremadeShape(shape)
                } else {
                    IsEnclosed::No
                }
            );
        }
    }
    #[test]
    fn old_terrain_height_and_degenerate_stroke_do_not_change_falloff() {
        use crate::resources::terrain_editing::StrokeId;
        let mut saved = history(vec![
            EditType::Terrain(Arc::new(TerrainEdit::NewTerrainStroke {
                index: StrokeId(7),
                height_ws: 0.25,
                stroke_pts: vec![[1.0, -0.0, 2.0]; 2],
            })),
            EditType::Terrain(Arc::new(TerrainEdit::EditTerrainStroke {
                index: StrokeId(7),
                prev_height_ws: 0.25,
                new_height_ws: 9.0,
                prev_falloff: 0.3,
                new_falloff: 0.7,
            })),
        ]);
        let (_, changed) = migrate_history(&mut saved, 16, false);
        assert!(changed && saved.snapshots.is_empty());
        let EditType::Terrain(edit) = &saved.edit_chain[0].0.edits[0] else {
            panic!()
        };
        let TerrainEdit::NewTerrainStroke {
            height_ws,
            stroke_pts,
            ..
        } = &**edit
        else {
            panic!()
        };
        assert_eq!(*height_ws, 5.0);
        assert_eq!(stroke_pts[1], [1.001, 0.0, 2.0]);
        let EditType::Terrain(edit) = &saved.edit_chain[0].0.edits[1] else {
            panic!()
        };
        let TerrainEdit::EditTerrainStroke {
            prev_height_ws,
            new_height_ws,
            prev_falloff,
            new_falloff,
            ..
        } = &**edit
        else {
            panic!()
        };
        assert_eq!(
            (*prev_height_ws, *new_height_ws, *prev_falloff, *new_falloff),
            (5.0, 20.0, 0.3, 0.7)
        );
        assert!(old_terrain_height(-1.0).is_nan());
        assert_eq!(old_terrain_height(f32::INFINITY), 20.0);
        assert_eq!(old_terrain_height(-0.0).to_bits(), (-0.0f32).to_bits());
    }
    #[test]
    fn current_version_keeps_deprecated_markers_and_snapshot_references_untouched() {
        let mut saved = history(vec![EditType::Decorator(Arc::new(
            DecoratorHistoryEdit::default(),
        ))]);
        let (flags, changed) = migrate_history(&mut saved, 27, false);
        assert_eq!(flags, ParsedLoadMetadata::VERSION_27);
        assert!(!changed);
        assert_eq!(
            (saved.edit_chain[0].0.edits.len(), saved.snapshots.len()),
            (1, 1)
        );
        let (flags, changed) = migrate_history(&mut saved, 26, false);
        assert!(flags.requires_upgrade_backup && changed);
        assert!(!flags.legacy_version_notice);
        assert!(saved.edit_chain[0].0.edits.is_empty() && saved.snapshots.is_empty());
    }
    #[test]
    fn snapshot_upgrade_applies_gate_table_and_decorators_with_distinct_flags() {
        use crate::systems::save::snapshot::{LoadedSnapshotData, decorator_history_state_json};
        let state = |ty| {
            decorator_history_state_json(&DecoratorHistoryState {
                ty,
                u28: (123u64 << 32) | 1,
                u34: 8,
                ..Default::default()
            })
            .unwrap()
        };
        let original = LoadedSnapshotData {
            sections: vec![
                ("GateColors", serde_json::json!([[11, 1], [12, 999]])),
                (
                    "Decorators",
                    serde_json::json!({"states": [["Terrain", {"state": [[1, state(56)], [2, state(58)]]}]]}),
                ),
            ],
            ..Default::default()
        };
        let mut flags = ParsedLoadMetadata::VERSION_27;
        flags.upgrade_arrow_slit_colors = true;
        let mut loaded = original.clone();
        migrate_snapshot(&mut loaded, flags).unwrap();
        assert_eq!(loaded.sections[0].1, original.sections[0].1);
        assert_eq!(
            loaded.sections[1].1["states"][0][1]["state"][1][1]["color_id"],
            999
        );
        flags.upgrade_window_colors = true;
        migrate_snapshot(&mut loaded, flags).unwrap();
        assert_eq!(
            loaded.sections[0].1,
            serde_json::json!([[11, 2], [12, 999]])
        );
        assert_eq!(
            loaded.sections[1].1["states"][0][1]["state"][0][1]["color_id"],
            2
        );
        assert_eq!(
            loaded.sections[1].1["states"][0][1]["state"][0][1]["seed"],
            123
        );
        assert_eq!(
            loaded.sections[1].1["states"][0][1]["state"][0][1]["window_light_color"],
            8
        );
    }
}
