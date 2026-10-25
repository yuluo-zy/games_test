import difflib, hashlib, json
from pathlib import Path
w=Path(__file__).parent
source=Path('tiny-glade/crates/systems/tree-tool/src/process_tree_cmds.rs')
before=(w/'before.rs').read_text()
s='''//! 树命令的权威消费链，B 142267430 / 1422686d0。
//!
//! 依次处理 Move、Insert、Delete。插入的主题来自 ActiveSession；
//! is_flowery/is_olden/is_winter 的 ICF 同址符号不能解释成 UI 或存档状态。
//! flags16 全零为原写面，mode 与声音仍保留主题、变体和插入布尔分支。

'''+before[before.index('use bevy_ecs'):]
s=s.replace('use country_core::systems::save::load_completion_checker::LoadingCompletionState;\n','')
a=s.index('/// `process_tree_cmds` @');b=s.index('#[allow(clippy::too_many_arguments)]',a)
s=s[:a]+'''/// B 原码按 Move→Insert→Delete 顺序消费；存储、Prefab placement 和声音
/// 分别沿现有权威所有者提交。原 Query 的缺失组件失败不得用替代状态掩盖。
'''+s[b:]
s=s.replace('    readiness: Option<Res<LoadingCompletionState>>,\n','')
a=s.index('    // ---- Insert 分支');b=s.index('    for cmd in inserts.read() {',a)
s=s[:a]+'''    // 原事件同时保留唯一存储索引、放置变体、旋转和布尔标志。
'''+s[b:]
a=s.index('        // O：`SessionLoadReadiness');b=s.index('        // O：动画 α',a)
s=s[:a]+'''        let is_olden = session.is_olden();
        let is_winter = session.is_winter();
        let flowery_extra = session.is_flowery() && (cmd.variant & 3) != 0;
        // B MAXSS(scale,.7) 只作用于 olden 裸树；NaN 时也选 .7。
        let scale = if naked && is_olden {
            cmd.scale.max(0.7)
        } else {
            cmd.scale
        };
'''+s[b:]
a=s.index('        // O：prefab = 事件名');b=s.index('        let prefab_name =',a)
s=s[:a]+'''        // 缺名回放与实时放置使用同一会话主题，不能读取加载 readiness。
'''+s[b:]
s=s.replace('''                deny_load,
                session.meta.is_halloween_any(),
                is_corrupted,
                false,
''','''                is_olden,
                session.meta.is_halloween_any(),
                is_winter,
                session.is_autumn(),
''')
a=s.index('        // O：`SpawnPrefabCmd`');b=s.index('        spawns.write(SpawnPrefabCmd',a)
s=s[:a]+'''        // B 142267dcd..ded：olden 裸树优先，其次插入标志，最后 flowery 变体。
        let mode = if naked && is_olden {
            3
        } else if cmd.is_preplaced {
            0
        } else if flowery_extra {
            2
        } else {
            1
        };
'''+s[b:]
a=s.index('        if let Some(previous) = previous {');b=s.index('        // O：水面采样',a)
replacement=s[a:b]
s=s[:a]+s[b:]
s=s.replace('selector=0.0<水样、extra=naked && (deny||corrupted)','selector=0.0<水样、extra=naked && (olden||winter)')
s=s.replace('extra: (naked && (deny_load || is_corrupted)) as u8,','extra: (naked && (is_olden || is_winter)) as u8,')
a=s.index('    // ---- Delete 分支');s=s[:a].rstrip()[:-1]+'''        // B 142267f72..142268141：先提交声音，随后才回收被替换的旧树。
'''+replacement+'''    }
'''+s[a:]
# 追加到既有 placement_consumer_tests 内，复用真实 world() 初始化。
assert s.rstrip().endswith('}')
s=s.rstrip()[:-1]+'''
    #[test]
    fn actual_insert_mode_uses_theme_variant_and_boolean_without_loading_readiness() {
        use country_core::resources::session::GladeType;
        for (theme, name, variant, is_preplaced, expected_scale, expected_mode, sound_extra) in [
            (GladeType::Summer, "Oak", 1, false, 0.6, 1, 0),
            (GladeType::Summer, "Oak", 1, true, 0.6, 0, 0),
            (GladeType::Flowery, "Oak", 1, false, 0.6, 2, 0),
            (GladeType::Flowery, "Oak", 4, false, 0.6, 1, 0),
            (GladeType::Flowery, "Oak", -17, false, 0.6, 2, 0),
            (GladeType::Flowery, "Oak", 1, true, 0.6, 0, 0),
            (GladeType::Olden, "clearing/naked_tree/naked1", 1, false, 0.7, 3, 1),
            (GladeType::Olden, "clearing/naked_tree/naked1", 1, true, 0.7, 3, 1),
            (GladeType::Winter, "clearing/naked_tree/naked1", 1, false, 0.6, 1, 1),
            (GladeType::Autumn, "clearing/naked_tree/naked1", 1, false, 0.6, 1, 0),
        ] {
            let mut world = world();
            world.resource_mut::<ActiveSession>().meta.glade = theme;
            world.send_event(InsertTreeCmd {
                id: 73,
                position: glam::Vec3::new(2.0, 3.0, 4.0),
                scale: 0.6,
                rotation_degrees: 45.0,
                variant,
                is_preplaced,
                prefab_name: name.into(),
            });
            let mut schedule = Schedule::default();
            schedule.add_systems(process_tree_cmds::<()>);
            schedule.run(&mut world);
            let events = world.resource::<Events<SpawnPrefabCmd>>();
            let mut cursor = events.get_cursor();
            let spawn = cursor.read(events).next().unwrap();
            assert_eq!(spawn.scale, expected_scale);
            assert_eq!(spawn.mode, expected_mode);
            assert_eq!(spawn.flags, [0; 16]);
            assert_eq!(spawn.rotation, 45.0);
            assert_eq!(spawn.variant, variant as u32);
            let entity = spawn.entity.unwrap();
            assert_eq!(world.resource::<TreeStorage>().get(TreeStorageIndex(73)).unwrap().entity, entity);
            assert_eq!(world.get::<TreeComponent>(entity).unwrap().is_preplaced, is_preplaced);
            assert_eq!(world.get::<TreeMoveBend>(entity).unwrap().position, glam::Vec2::new(2.0, 4.0));
            assert_eq!(world.get::<TreeAmbience>(entity).unwrap().position, glam::Vec3::new(2.0, 3.0 + expected_scale * 10.0, 4.0));
            let events = world.resource::<Events<PlayTreeSoundCmd>>();
            let mut cursor = events.get_cursor();
            let sound = cursor.read(events).next().unwrap();
            assert_eq!(sound.extra, sound_extra);
            assert_eq!(sound.value, inverse_lerp(0.5, 0.9, 0.6));
            assert_eq!(sound.position, glam::Vec3::new(2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn missing_prefab_fallback_uses_session_and_keeps_original_name_audio_gate() {
        use country_core::resources::session::GladeType;
        for (theme, id, expected) in [
            (GladeType::Summer, 8, "clearing/tree/branchy2"),
            (GladeType::Olden, 8, "clearing/naked_tree/naked1"),
            (GladeType::Autumn, 9, "clearing/tree/autumn/branchy1"),
            (GladeType::Winter, 8, "clearing/naked_tree/naked1_no_mushrooms"),
            (GladeType::Winter, 9, "clearing/tree/branchy1"),
        ] {
            let mut world = world();
            world.resource_mut::<ActiveSession>().meta.glade = theme;
            world.send_event(InsertTreeCmd {
                id, position: glam::Vec3::ZERO, scale: 0.6,
                rotation_degrees: 90.0, variant: 2, is_preplaced: false,
                prefab_name: country_core::resources::history::tree_edit::missing_prefab().into(),
            });
            let mut schedule = Schedule::default();
            schedule.add_systems(process_tree_cmds::<()>);
            schedule.run(&mut world);
            let events = world.resource::<Events<SpawnPrefabCmd>>();
            let mut cursor = events.get_cursor();
            let spawn = cursor.read(events).next().unwrap();
            assert_eq!(spawn.prefab_name.as_ref().unwrap().as_name_ref(), expected);
            // 原版按事件旧名判断裸树，不按 fallback 后的 prefab 重算。
            assert_eq!(spawn.mode, 1);
            assert_eq!(spawn.scale, 0.6);
            let events = world.resource::<Events<PlayTreeSoundCmd>>();
            let mut cursor = events.get_cursor();
            assert_eq!(cursor.read(events).next().unwrap().extra, 0);
        }
    }

    #[test]
    fn olden_naked_scale_preserves_original_maxss_nan_result() {
        use country_core::resources::session::GladeType;
        let mut world = world();
        world.resource_mut::<ActiveSession>().meta.glade = GladeType::Olden;
        world.send_event(InsertTreeCmd {
            id: 1, position: glam::Vec3::ZERO, scale: f32::NAN,
            rotation_degrees: 0.0, variant: 0, is_preplaced: false,
            prefab_name: "clearing/naked_tree/naked1".into(),
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_tree_cmds::<()>);
        schedule.run(&mut world);
        let events = world.resource::<Events<SpawnPrefabCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(cursor.read(events).next().unwrap().scale, 0.7);
        let events = world.resource::<Events<PlayTreeSoundCmd>>();
        let mut cursor = events.get_cursor();
        assert!(cursor.read(events).next().unwrap().value.is_nan());
    }
}
'''
(w/'candidate.rs').write_text(s)
(w/'candidate.patch').write_text(''.join(difflib.unified_diff(before.splitlines(True),s.splitlines(True),fromfile='a/'+str(source),tofile='b/'+str(source))))
(w/'hashes.json').write_text(json.dumps({'source':str(source),'before_sha256':hashlib.sha256(before.encode()).hexdigest(),'candidate_sha256':hashlib.sha256(s.encode()).hexdigest(),'tests_executed':False},indent=2))
print((w/'hashes.json').read_text())
