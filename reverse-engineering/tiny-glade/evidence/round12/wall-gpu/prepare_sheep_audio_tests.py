from pathlib import Path
r=Path('D:/game');b=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep';p=Path('crates/systems/critters/src/sheep/sheep_behavior.rs');s=(r/'tiny-glade'/p).read_text(encoding='utf-8');a=s.index('/// update_sheep_audio（');z=s.index('\n#[cfg(test)]',a);s=s[:a]+'''/// B update_sheep_audio142316490: set position, then audio+0x34 = dt + old.
pub fn update_sheep_audio(time: Res<GameTime>, mut query: Query<(&Transform, &mut SheepAudio)>) {
    for (transform, mut audio) in &mut query {
        audio.handle.set_position(transform.translation);
        audio.elapsed = time.delta + audio.elapsed;
    }
}

/// Original 56B audio record's semantic fields; the host owns AudioInstance.
#[derive(Component, Debug, Default)]
pub struct SheepAudio {
    pub handle: AudioHandle,
    /// B +0x30: same pitch used for startup's Pitch parameter.
    pub startup_pitch: f32,
    /// B +0x34, initialized zero and advanced by update_sheep_audio.
    pub elapsed: f32,
}
'''+s[z:];q=b/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_text(s,encoding='utf-8')
# Existing integration tests use actual unique control component birth.
p=Path('crates/country-core/tests/sheep_snapshot_native.rs');s=(r/'tiny-glade'/p).read_text(encoding='utf-8');s=s.replace('world.insert_resource(SheepControl {','let control_owner = world.spawn(SheepControl {').replace('            sheep: vec![saved.clone()],\n        }),\n    });','            sheep: vec![saved.clone()],\n        }),\n        ..SheepControl::new(Entity::from_raw(1),Entity::from_raw(2))\n    }).id();').replace('snapshot: Some(SheepControlSnapshot { sheep: vec![] }),\n    });','snapshot: Some(SheepControlSnapshot { sheep: vec![] }),\n        ..SheepControl::new(Entity::from_raw(1),Entity::from_raw(2))\n    }).id();').replace('.resource::<SheepControl>()','.get::<SheepControl>(control_owner).unwrap()');q=b/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_text(s,encoding='utf-8')
# The missing-prev regression must use the real sheep and animation components.
p=Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_tests.rs');q=b/p;s=q.read_text(encoding='utf-8');s=s.replace('let e = w.spawn(SheepView::default()).id();','''let e = w.spawn((
        crate::resources::critters::sheep::Sheep { behavior_state:0,goal_timer:0.0,pet_phase:0,
        pet_progress:0.0,pet_target:0,move_state:crate::resources::critters::sheep::SheepMoveState::Idle{timeout_s:0.0},
        amplitude:0.0,control_flag:0 },crate::resources::critters::sheep::SheepWalkRecord::default()
    )).id();''');q.write_text(s,encoding='utf-8')
print('audio clock and actual owner integration tests ready')
