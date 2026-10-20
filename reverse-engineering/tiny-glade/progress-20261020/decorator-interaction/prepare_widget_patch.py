from pathlib import Path
import difflib, json, hashlib
root=Path('D:/game')
out=Path(__file__).parent
ui=Path('tiny-glade/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs')
res=Path('tiny-glade/crates/systems/decorator/src/decorator_resources.rs')
old_ui=(root/ui).read_text(encoding='utf-8')
begin=old_ui.index('/// 装饰手柄名')
end=old_ui.index('/// B DecoratorAffordanceDispatcher',begin)
new_ui=old_ui[:begin]+'''// One canonical identity: the original key is a u64 and a u8, not a string.
pub use crate::decorator_resources::DecoratorHandleName;

'''+old_ui[end:]
begin=new_ui.index('/// `create_decorator_widget<&str>`')
new_ui=new_ui[:begin]+'''/// B1412823c0/187B. The fourth argument is XMM3 f32, stored at
/// volume+4. It is not a canvas layer. Preserve the constructor's occlusion
/// byte and flag bit0; add bit1 and, for type '?', bit3.
pub fn create_decorator_widget(
    ui: &UiState,
    name: &str,
    distance: f32,
    is_question_mark: bool,
    sounds: UiSounds,
) {
    ui.add(name, decorator_widget_params(distance, is_question_mark, sounds));
}

/// B141282300/187B, with the same parameter builder and the canonical typed
/// name hash used by UiState::add<DecoratorHandleName>.
pub fn create_decorator_widget_handle(
    ui: &UiState,
    name: DecoratorHandleName,
    distance: f32,
    is_question_mark: bool,
    sounds: UiSounds,
) {
    ui.add_with_id(name.to_widget_id(), decorator_widget_params(distance, is_question_mark, sounds));
}

fn decorator_widget_params(distance: f32, is_question_mark: bool, sounds: UiSounds) -> WidgetParams {
    let mut params = WidgetParams::new(WidgetVolumeShape {
        tag: 5,
        size: glam::Vec2::new(distance, 0.0),
        ..Default::default()
    }, [0; 2]).sounds(sounds);
    params.flags.0 |= 2 | (u8::from(is_question_mark) << 3);
    params
}

#[cfg(test)]
mod widget_tests {
    use super::*;

    #[test]
    fn both_real_registry_entries_keep_native_distance_flags_and_sounds() {
        for distance in [0.0f32, -0.0, 1.0, -4.5, 125.0, f32::INFINITY] {
            for question in [false, true] {
                let ui = UiState::default();
                let mut sounds = UiSounds::empty();
                for (i, byte) in sounds.slots.iter_mut().flatten().enumerate() {
                    *byte = (i as u8).wrapping_mul(7).wrapping_add(3);
                }
                let name = DecoratorHandleName(0x123456789abcdef0, 7);
                create_decorator_widget(&ui, "decorator", distance, question, sounds);
                create_decorator_widget_handle(&ui, name, distance, question, sounds);
                for id in [WidgetId::from_name("decorator"), name.to_widget_id()] {
                    let params = ui.widget(id).unwrap();
                    assert_eq!(params.volume.tag, 5);
                    assert_eq!(params.volume.size.x.to_bits(), distance.to_bits());
                    assert_eq!(params.flags.0, if question { 11 } else { 3 });
                    assert!(params.occlude_by_world);
                    assert_eq!(params.sounds, sounds);
                }
            }
        }
    }
}
'''
old_res=(root/res).read_text(encoding='utf-8')
new_res='''//! Canonical decorator handle name and its widget hash.
//! B14125d180 copies a u64 and a u8 into the name before calling B14128c8e0.
//! That hash helper writes exactly 8 bytes followed by 1 byte into zero-key
//! std SipHasher13. A string or its 0xff Hash terminator changes the key.
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use country_core::systems::ui::resources::ui_state::WidgetId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorHandleName(pub u64, pub u8);

impl DecoratorHandleName {
    pub fn to_widget_name_hash(self) -> u64 {
        let mut hash = DefaultHasher::new();
        hash.write_u64(self.0);
        hash.write_u8(self.1);
        hash.finish()
    }

    pub fn to_widget_id(self) -> WidgetId {
        WidgetId(self.to_widget_name_hash())
    }
}

pub fn to_widget_name_hash(name: DecoratorHandleName) -> u64 {
    name.to_widget_name_hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_b_handle_hash_goldens() {
        // Filled from execution of the complete original helper, including
        // its actual std SipHasher writes; no hash implementation is copied.
        for (key, kind, expected) in GOLDENS {
            let name = DecoratorHandleName(key, kind);
            assert_eq!(name.to_widget_name_hash(), expected);
            assert_eq!(name.to_widget_id().0, expected);
        }
    }

    const GOLDENS: [(u64, u8, u64); 25] = [
GOLDEN_ROWS
    ];
}
'''
goldens=json.loads((out/'handle-hash-goldens.json').read_text())
new_res=new_res.replace('GOLDEN_ROWS','\n'.join(f'        ({r["key"]}, {r["kind"]}, {r["hash"]}),' for r in goldens))
patch=[]
manifest=[]
for path,old,new in [(ui,old_ui,new_ui),(res,old_res,new_res)]:
    candidate=out/'candidate'/path
    before=out/'before'/path
    candidate.parent.mkdir(parents=True,exist_ok=True)
    before.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(new,encoding='utf-8',newline='\n')
    before.write_text(old,encoding='utf-8',newline='\n')
    patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+path.as_posix(),tofile='b/'+path.as_posix()))
    manifest.append({'path':str(path),'before':hashlib.sha256((root/path).read_bytes()).hexdigest(),'candidate':hashlib.sha256(candidate.read_bytes()).hexdigest()})
(out/'widget-and-handle.patch').write_text(''.join(patch),encoding='utf-8',newline='\n')
(out/'widget-and-handle-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps({'files':len(manifest),'patch':str(out/'widget-and-handle.patch')}))
