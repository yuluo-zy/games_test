//! Coherent topology fades, shader readiness and value-continuous root motion.
use super::*;

/// The old opaque entities have not been mutated yet. Roll back resource
/// residency and discard only unpublished buffers/materials/zero-alpha ghosts.
pub(super) fn abort_waiting(
    root: Entity,
    fade: Fade,
    commands: &mut Commands,
    assets: &mut RenderAssets,
) {
    debug_assert!(!fade.handoff.started && !fade.handoff.switched);
    if let Some(stage) = assets.staging.0.remove(&root) {
        stage.upload.discard(&mut assets.meshes, &assets.bridge);
    }
    if let Some(next) = assets.residency.0.insert(root, fade.baseline.clone()) {
        for part in next.parts.into_values() {
            if !fade
                .baseline
                .parts
                .values()
                .any(|p| p.handle == part.handle)
            {
                release_mesh(&mut assets.meshes, &assets.bridge, &part.handle);
            }
        }
    }
    for child in fade
        .restore
        .iter()
        .map(|(child, _)| *child)
        .chain(fade.warmups.iter().copied())
    {
        commands.entity(child).despawn();
    }
    // Retired entities belong to the restored baseline: keep their GPU assets.
    dispose_fade(root, fade, assets);
}

// Zero-alpha instances queue the actual material/vertex-layout variant without
// hiding the old opaque picture. There is no duplicate geometry upload.
pub(super) fn warm_old_part(
    part: &ResidentBatch,
    root: Entity,
    commands: &mut Commands,
    materials: &mut BTreeMap<MaterialKey, Handle<StandardMaterial>>,
    warmups: &mut Vec<Entity>,
    assets: &mut RenderAssets,
) {
    if materials.contains_key(&part.material) {
        return;
    }
    let material = fading_material(part.material, 0., materials, assets);
    let child = commands
        .spawn((
            Mesh3d(part.handle.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::from_array(part.offset)),
            ChildOf(root),
        ))
        .id();
    assets.bridge.track_draw(child);
    warmups.push(child);
}
pub(super) fn fading_material(
    key: MaterialKey,
    alpha: f32,
    cache: &mut BTreeMap<MaterialKey, Handle<StandardMaterial>>,
    assets: &mut RenderAssets,
) -> Handle<StandardMaterial> {
    cache
        .entry(key)
        .or_insert_with(|| {
            let mut material = assets
                .materials
                .get(&assets.palette.0[&key])
                .unwrap()
                .clone();
            material.alpha_mode = AlphaMode::Blend;
            material.base_color = material.base_color.with_alpha(alpha);
            assets.materials.add(material)
        })
        .clone()
}
pub(super) fn dispose_fade(entity: Entity, fade: Fade, assets: &mut RenderAssets) {
    for child in fade
        .warmups
        .iter()
        .copied()
        .chain(fade.restore.iter().map(|(e, _)| *e))
    {
        assets.bridge.forget_draw(child);
    }
    for part in fade.retired {
        // Transform-only changes share the active GPU buffer.
        if !assets
            .residency
            .0
            .get(&entity)
            .is_some_and(|r| r.parts.values().any(|p| p.handle == part.handle))
        {
            release_mesh(&mut assets.meshes, &assets.bridge, &part.handle);
        }
    }
    for handle in fade
        .old_materials
        .into_values()
        .chain(fade.next_materials.into_values())
    {
        assets.materials.remove(handle.id());
    }
}
pub(super) fn advance_transitions(
    mut commands: Commands,
    time: Option<Res<Time>>,
    settings: Res<TransitionSettings>,
    mut poses: Query<(Entity, &mut Transform, &RootMotion)>,
    mut assets: RenderAssets,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs().min(0.1);
    for (entity, mut current, goal) in &mut poses {
        let a = if settings.motion_half_life.is_finite() && settings.motion_half_life > 0. {
            1. - (-dt / settings.motion_half_life).exp2()
        } else {
            1.
        };
        current.translation = current.translation.lerp(goal.0.translation, a);
        current.rotation = current.rotation.slerp(goal.0.rotation, a);
        if current.translation.distance_squared(goal.0.translation) < 1e-8
            && current.rotation.angle_between(goal.0.rotation) < 1e-4
        {
            *current = goal.0;
            commands.entity(entity).remove::<RootMotion>();
        }
    }
    let entities = assets.fades.0.keys().copied().collect::<Vec<_>>();
    for entity in entities {
        let mut fade = assets.fades.0.remove(&entity).unwrap();
        let ready = fade.handoff.pipeline_ready(
            dt,
            assets.bridge.enabled,
            assets.bridge.draws_ready(
                fade.restore
                    .iter()
                    .map(|(e, _)| *e)
                    .chain(fade.warmups.iter().copied()),
            ),
        );
        if ready && let Some(groups) = &mut assets.groups {
            groups.building_ready(fade.next.ticket.object_revision, fade.next.ticket.building);
        }
        let group_ready = assets
            .groups
            .as_ref()
            .is_none_or(|g| g.ready(fade.next.ticket.object_revision));
        let elapsed = assets
            .groups
            .as_mut()
            .and_then(|g| g.elapsed(fade.next.ticket.object_revision, time.elapsed_secs_f64()));
        let Some(step) = fade
            .handoff
            .advance(dt, settings.duration, ready && group_ready, elapsed)
        else {
            assets.fades.0.insert(entity, fade);
            continue;
        };
        if step.started_now {
            for part in &fade.retired {
                commands
                    .entity(part.child)
                    .insert(MeshMaterial3d(fade.old_materials[&part.material].clone()));
            }
            for child in fade.warmups.drain(..) {
                commands.entity(child).despawn();
                assets.bridge.forget_draw(child);
            }
        }
        let alpha = step.alpha;
        for handle in fade.old_materials.values() {
            if let Some(mut m) = assets.materials.get_mut(handle) {
                m.base_color = m.base_color.with_alpha(1. - alpha);
            }
        }
        for handle in fade.next_materials.values() {
            if let Some(mut m) = assets.materials.get_mut(handle) {
                m.base_color = m.base_color.with_alpha(alpha);
            }
        }
        // During cross-fade pick the dominant coherent pose, never combine old
        // wall triangles with new window triangles into a fictitious collider.
        if step.switched_now {
            commands.entity(entity).insert(fade.next.clone());
        }
        if step.complete {
            for (child, key) in &fade.restore {
                commands
                    .entity(*child)
                    .insert(MeshMaterial3d(assets.palette.0[key].clone()));
            }
            for part in &fade.retired {
                commands.entity(part.child).despawn();
            }
            assets.pending.displayed(fade.next.ticket);
            dispose_fade(entity, fade, &mut assets);
        } else {
            assets.fades.0.insert(entity, fade);
        }
    }
    assets.stats.transition_batches = assets.fades.0.values().map(|f| f.retired.len()).sum();
}
