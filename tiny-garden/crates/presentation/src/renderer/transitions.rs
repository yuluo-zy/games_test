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
    debug_assert!(!fade.started && !fade.switched);
    if let Some(stage) = assets.staging.0.remove(&root) {
        for h in stage.owned {
            assets.bridge.forget(&h);
            assets.meshes.remove(h.id());
        }
    }
    if let Some(next) = assets.residency.0.insert(root, fade.baseline.clone()) {
        for part in next.parts.into_values() {
            if !fade
                .baseline
                .parts
                .values()
                .any(|p| p.handle == part.handle)
            {
                assets.bridge.forget(&part.handle);
                assets.meshes.remove(part.handle.id());
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
            assets.bridge.forget(&part.handle);
            assets.meshes.remove(part.handle.id());
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
        fade.age += dt;
        let unseen = fade.age > 0.05
            && fade
                .restore
                .iter()
                .all(|(child, _)| !assets.bridge.draw_visible(*child));
        if !fade.started {
            let ready = (!assets.bridge.enabled || fade.age > 0.05)
                && fade
                    .restore
                    .iter()
                    .map(|(child, _)| *child)
                    .chain(fade.warmups.iter().copied())
                    .filter(|e| assets.bridge.draw_visible(*e))
                    .all(|e| assets.bridge.draw_ready(e));
            if !ready && !unseen {
                assets.fades.0.insert(entity, fade);
                continue;
            }
            for part in &fade.retired {
                commands
                    .entity(part.child)
                    .insert(MeshMaterial3d(fade.old_materials[&part.material].clone()));
            }
            for child in fade.warmups.drain(..) {
                commands.entity(child).despawn();
                assets.bridge.forget_draw(child);
            }
            fade.started = true;
        }
        fade.elapsed += dt;
        let t = if unseen && assets.bridge.enabled {
            1.
        } else {
            (fade.elapsed / settings.duration.max(0.001)).clamp(0., 1.)
        };
        let alpha = t * t * (3. - 2. * t);
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
        if !fade.switched && alpha >= 0.5 {
            commands.entity(entity).insert(fade.next.clone());
            fade.switched = true;
        }
        if t >= 1. {
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
