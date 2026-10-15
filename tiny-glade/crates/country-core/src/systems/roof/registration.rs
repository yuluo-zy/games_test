//! Executable local roof CPU dependency order. This is not proof of the
//! original global scheduler or the still-unrestored render/trigger producer.
use super::{editing::*, lifecycle::*, replay::route_roof_replay};
use crate::systems::{history::HistoryCpuStage, wall::registration::WallCpuStage};
use bevy_ecs::prelude::*;

/// Call alongside add_wall_cpu_systems and add_history_cpu_systems. Commands
/// are flushed by Bevy's chained-system deferred edges before association and
/// after marking/despawn; no fabricated Entity or alternate wall map is used.
pub fn add_roof_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            route_roof_replay,
            create_roof,
            wall_roof_hashmap,
            update_roof_spatial,
            edit_roof,
            switch_roof_direction,
            set_flat_roof_style,
            set_flat_roof_rim_style,
            apply_roof_type_gameplay,
            process_invalid_roof_notifications,
            mark_roof_for_deletion,
            wall_height_dt,
            roof_animation,
            delete_roofs,
            wall_roof_hashmap,
            update_wall_gable_heights,
        )
            .chain()
            .in_set(WallCpuStage::RoofDependencies)
            .in_set(HistoryCpuStage::Apply),
    );
}
