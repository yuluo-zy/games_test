from pathlib import Path
import json,hashlib,difflib
ROOT=Path('/Users/yuluo/Desktop/game')
CORE='games_test/tiny-glade/crates/country-core/src/'
WORK=Path(__file__).resolve().parent
files={}
def read(rel):
 p=ROOT/CORE/rel; s=p.read_text(); files[rel]={'base':s,'candidate':s};return s
def write(rel,s): files[rel]['candidate']=s
rel='resources/history/wall_edit.rs';s=read(rel)
s=s.replace('''        source: WallAuthoritySnapshot,
        color_id: u32,
    },
    /// 原生变体 3''','''        source: WallAuthoritySnapshot,
        color_id: u32,
        /// Native T+0x80, distinct from the captured source height. Delete's
        /// Some-source undo preserves that source; only native None fallback
        /// uses this height to construct an empty source (140ac1322).
        restored_height: f32,
    },
    /// 原生变体 3''',1)
needle='''    WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::Delete {
            wall,
            source: source.clone(),
            color_id,
        }]],
    }
}'''
replacement='''    record_deleted_wall_with_height(wall, source, color_id, source.source.extra)
}

/// B finish_wall(false) retains both the previous snapshot and the independent
/// target height (T+0x80). Do not overwrite the captured snapshot to store it.
pub fn record_deleted_wall_with_height(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
    restored_height: f32,
) -> WallCompoundHistoryEdit {
    WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::Delete {
            wall,
            source: source.clone(),
            color_id,
            restored_height,
        }]],
    }
}'''
assert needle in s;s=s.replace(needle,replacement,1)
s=s.replace('''        WallHistoryEdit::Delete {
            wall,
            source,
            color_id,
        } =>''','''        WallHistoryEdit::Delete {
            wall,
            source,
            color_id,
            ..
        } =>''',1)
s=s.replace('''        let mut restored = source.clone();
        if let WallHistoryEdit::Create''','''        let mut restored = source.clone();
        // Delete Some-source restores the original height (140ac12be/14c0).
        // Its distinct restored_height is retained for native None fallback,
        // which the current mandatory-source model does not represent.
        if let WallHistoryEdit::Create''',1)
insert='''
    #[test]
    fn delete_retains_independent_height_but_some_source_undo_restores_snapshot() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let state = crate::resources::walls::WallState::new(curve, 3.9, 3, 0, false);
        let source = state.snapshot();
        let terrain = crate::resources::walls::publication_source::TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        };
        for height in [7.0, -0.0, f32::from_bits(0x7fc12345)] {
            let compound = record_deleted_wall_with_height(WallId(9), source, 17, height);
            let edit = &compound.groups[0][0];
            let WallHistoryEdit::Delete { source: captured, restored_height, color_id, .. } = edit else {
                panic!("delete record");
            };
            assert_eq!(restored_height.to_bits(), height.to_bits());
            assert_eq!(captured.source.extra.to_bits(), 3.9_f32.to_bits());
            assert_eq!(*color_id, 17);
            let mut sources = std::collections::HashMap::new();
            let out = apply_instant_wall_edit(&mut sources, edit, true, &terrain).unwrap();
            assert_eq!(out.new_source_notification, Some(WallId(9)));
            assert_eq!(sources[&WallId(9)].source.extra.to_bits(), 3.9_f32.to_bits());
            apply_instant_wall_edit(&mut sources, edit, false, &terrain).unwrap();
            assert!(sources.is_empty());
        }
        let compound = record_deleted_wall(WallId(9), source, 17);
        let WallHistoryEdit::Delete { restored_height, .. } = &compound.groups[0][0] else {
            panic!("delete record");
        };
        assert_eq!(restored_height.to_bits(), source.source.extra.to_bits());
    }
'''
pos=s.rfind('\n}');s=s[:pos]+insert+s[pos:];write(rel,s)
rel='systems/wall/history_recording.rs';s=read(rel)
s=s.replace('use crate::resources::history::wall_edit::record_deleted_wall;','use crate::resources::history::wall_edit::record_deleted_wall;\nuse crate::resources::history::wall_edit::record_deleted_wall_with_height;',1)
needle='''    pub fn deleted_wall(wall: WallId, source: &WallState, color_id: u32) -> Self {
        Self(Arc::new(record_deleted_wall(
            RecoveredWallId(wall.0),
            source.snapshot(),
            color_id,
        )))
    }
'''
assert needle in s;s=s.replace(needle,needle+'''
    /// Native continuation-failure Delete retains its independent target
    /// height while the previous source remains an unchanged undo snapshot.
    pub fn deleted_wall_with_height(
        wall: WallId,
        source: &WallState,
        color_id: u32,
        restored_height: f32,
    ) -> Self {
        Self(Arc::new(record_deleted_wall_with_height(
            RecoveredWallId(wall.0),
            source.snapshot(),
            color_id,
            restored_height,
        )))
    }
''',1);write(rel,s)
rel='resources/history/serialized.rs';s=read(rel)
s=s.replace('''            Self::Delete {
                wall: id,
                source,
                color_id,
            } => SavedWallEdit::DeleteWall {
                wall_index: id.0,
                wall_height: source.source.extra,''','''            Self::Delete {
                wall: id,
                source,
                color_id,
                restored_height,
            } => SavedWallEdit::DeleteWall {
                wall_index: id.0,
                wall_height: *restored_height,''',1)
start=s.index('''            } => {
                // The current live Delete payload stores the source height.''')
end=s.index('''            SavedWallEdit::AdjustHeight''',start)
s=s[:start]+'''            } => Self::Delete {
                wall: WallId(wall_index),
                source: before.snapshot().clone(),
                color_id,
                restored_height: wall_height,
            },
'''+s[end:]
insert='''
    #[test]
    fn delete_wall_distinct_height_round_trips_without_overwriting_before() {
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[0.0, 0.0], [4.0, 0.0]], false,
        ).unwrap();
        let state = WallState::new(curve, 3.9, 3, 0, false);
        let edit = WallHistoryEdit::Delete {
            wall: WallId(9), source: state.snapshot().clone(), color_id: 17,
            restored_height: 7.0,
        };
        let saved = serde_json::to_value(&edit).unwrap();
        assert_eq!(saved["DeleteWall"]["wall_height"], 7.0);
        assert_eq!(saved["DeleteWall"]["before"]["height"], serde_json::json!(3.9_f32));
        let back: WallHistoryEdit = serde_json::from_value(saved.clone()).unwrap();
        assert_eq!(serde_json::to_value(&back).unwrap(), saved);
        let WallHistoryEdit::Delete { source, restored_height, .. } = back else {
            panic!("delete record");
        };
        assert_eq!(source.source.extra.to_bits(), 3.9_f32.to_bits());
        assert_eq!(restored_height.to_bits(), 7.0_f32.to_bits());
        let mut alias = saved.clone();
        let fields = alias["DeleteWall"].as_object_mut().unwrap();
        let before = fields.remove("before").unwrap();
        fields.insert("undo_instant_state".into(), before);
        let back: WallHistoryEdit = serde_json::from_value(alias).unwrap();
        assert_eq!(serde_json::to_value(back).unwrap(), saved);
    }
'''
pos=s.rfind('\n}');s=s[:pos]+insert+s[pos:];write(rel,s)
rel='systems/history/replay_wall.rs';s=read(rel)
needle='''                    source: state.snapshot().clone(),
                    color_id: 0,
                }],'''
assert needle in s;s=s.replace(needle,'''                    source: state.snapshot().clone(),
                    color_id: 0,
                    restored_height: state.snapshot().source.extra,
                }],''',1)
insert='''
    #[test]
    fn delete_undo_uses_before_height_even_when_saved_fallback_height_differs() {
        let (mut world, mut schedule, state) = fixture();
        let edit = crate::resources::history::wall_edit::record_deleted_wall_with_height(
            RecoveredWallId(9), state.snapshot(), 7, 7.0,
        );
        push(&mut world, 0, &edit, true, true);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<InnerWalls>().get_unwrapped(WallId(9)).height().to_bits(),
            state.height().to_bits(),
        );
        assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
        assert_eq!(world.resource::<Events<WallHistoryRestoreColor>>().len(), 1);
        push(&mut world, 1, &edit, false, true);
        schedule.run(&mut world);
        assert!(!world.resource::<InnerWalls>().contains(WallId(9)));
        assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
    }
'''
pos=s.rfind('\n}');s=s[:pos]+insert+s[pos:];write(rel,s)
rel='systems/wall/eraser/archivist.rs';s=read(rel)
needle='''            source: child_walls.original.clone(),
            color_id: 0,
        });'''
assert s.count(needle)==2;s=s.replace(needle,'''            source: child_walls.original.clone(),
            color_id: 0,
            restored_height: child_walls.original.source.extra,
        });''');write(rel,s)
rel='systems/wall/eraser/bulldoze.rs';s=read(rel)
needle='''                vec![WallHistoryEdit::Delete {
                    wall,
                    source: next,
                    color_id,
                }],'''
assert needle in s;s=s.replace(needle,'''                vec![WallHistoryEdit::Delete {
                    wall,
                    restored_height: next.source.extra,
                    source: next,
                    color_id,
                }],''',1);write(rel,s)
manifest=[]
for rel,item in files.items():
 for kind in ['base','candidate']:
  path=WORK/kind/rel;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(item[kind])
 manifest.append({'path':CORE+rel,'sha256':hashlib.sha256(item['base'].encode()).hexdigest(),'candidate':str(Path('candidate')/rel),'role':'owned wall_history adapter' if rel=='resources/history/wall_edit.rs' else 'shared necessary API/serde integration hunk'})
(WORK/'source-map.json').write_text(json.dumps(manifest,indent=2)+'\n')
