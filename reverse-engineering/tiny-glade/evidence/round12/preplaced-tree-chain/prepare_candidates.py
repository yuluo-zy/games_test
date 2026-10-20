from pathlib import Path
import re,json,hashlib,difflib,subprocess
ROOT=Path('D:/game')
OUT=Path(__file__).resolve().parent
changes=[]
def change(path, transform):
    before=(ROOT/path).read_text(encoding='utf8')
    after=transform(before)
    base=OUT/'base'/path; base.parent.mkdir(parents=True,exist_ok=True)
    if base.exists():
        assert base.read_text(encoding='utf8')==before, 'shared baseline changed: '+path
    else: base.write_text(before,encoding='utf8',newline='\n')
    target=OUT/'candidate'/path; target.parent.mkdir(parents=True,exist_ok=True)
    target.write_text(after,encoding='utf8',newline='\n')
    changes.append((path,before,target))
def swap(s):
    s=re.sub(r'\bPrefabInstanceData\b','TEMP_NATIVE_CONTENTS',s)
    s=re.sub(r'\bPrefabInstance\b','PrefabInstanceData',s)
    s=s.replace('TEMP_NATIVE_CONTENTS','PrefabInstance')
    s=s.replace('resources::prefab::PrefabInstanceData','systems::prefab::PrefabInstanceData')
    s=s.replace('systems::prefab::PrefabInstance;', 'resources::prefab::PrefabInstance;')
    s=s.replace('systems::prefab::PrefabInstance,', 'resources::prefab::PrefabInstance,')
    return s

core='tiny-glade/crates/country-core/src/'
source=(ROOT/core/'systems/prefab.rs').read_text(encoding='utf8')
begin=source.index('/// 预制体实例数据组件')
end=source.index('/// 生成预制体命令',begin)
contents=source[begin:end].replace('PrefabInstanceData','PrefabInstance')
# The aggregate keeps its existing owned GPU/RT/audio contents. Only its
# canonical defining module/type changes; no prefab algorithm is rewritten.
contents=contents.replace('Vec<GpuInstanceRecord>', 'Vec<crate::systems::prefab::GpuInstanceRecord>')
contents=contents.replace('Vec<RtMeshHandle>', 'Vec<crate::resources::rt::RtMeshHandle>')
contents=contents.replace('Vec<AudioTriggerRecord>', 'Vec<crate::systems::prefab::AudioTriggerRecord>')
contents=contents.replace('/// 预制体实例数据组件（O：`systems::prefab::PrefabInstance`，**原生 0x58 B**，见模块文档）。',
    '/// Canonical B resources::prefab::PrefabInstance, 88B owned contents.\n/// Original register_component141552040 writes size0x58 at1415522dc.')

def resources(s):
    a=s.index('/// Mutable placement component.'); b=s.index('/// 预制体库值',a)
    return s[:a]+contents+s[b:]
change(core+'resources/prefab.rs',resources)

def systems(s):
    a=s.index('/// 预制体实例数据组件'); b=s.index('/// 生成预制体命令',a)
    s=s[:a]+'''/// Canonical B systems::prefab::PrefabInstanceData: mutable48B placement.
/// Original register_component140b7e440 writes size0x30 at140b7e6ed.
#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrefabInstanceData(pub PlacementDataSsbo);

'''+s[b:]
    # Swap all old aggregate algorithm references, while retaining the newly
    # restored declaration's canonical name.
    s=s.replace('pub struct PrefabInstanceData(pub PlacementDataSsbo);','pub struct RESTORED_PLACEMENT(pub PlacementDataSsbo);')
    s=swap(s)
    s=s.replace('pub struct RESTORED_PLACEMENT(pub PlacementDataSsbo);','pub struct PrefabInstanceData(pub PlacementDataSsbo);')
    s=s.replace('/// Canonical B systems::prefab::PrefabInstance: mutable48B placement.',
        '/// Canonical B systems::prefab::PrefabInstanceData: mutable48B placement.')
    # swap() changes the old re-export into the placement module itself;
    # the aggregate lives in resources::prefab and remains exported here too.
    s=s.replace('pub use crate::systems::prefab::PrefabInstanceData;',
        'pub use crate::resources::prefab::PrefabInstance;')
    # Correct historical layout summary without altering the actual algorithms.
    s=s.replace('`PrefabInstanceData` = **0x58 B**','`resources::prefab::PrefabInstance` = **0x58 B**')
    s=s.replace('components::trees::PrefabInstance` 已改为 re-export 本文件的 0x58 语义类型',
        'components::trees::PrefabInstanceData` re-exports the canonical48B placement')
    return s
change(core+'systems/prefab.rs',systems)
change(core+'systems/prefab/runtime.rs',swap)

def mapped_rock(s):
    at=s.index('/// 岩石 mask 栅格边长')
    return s[:at]+'''impl TerrainRockMaskSsbo {
    /// Native typed mapped-readback view. Reuse the same bit-sampling policy
    /// without a second CPU shadow or a caller-specific copied mask.
    pub fn sample_mapped_at_world_xz(mask: &[Self], x: f32, z: f32) -> u32 {
        sample_at_world_xz(bytemuck::cast_slice(mask), x, z)
    }
}

'''+s[at:]
change(core+'resources/terrain_rock_mask.rs',mapped_rock)

for path in [
    'tiny-glade/crates/systems/tree-tool/src/process_tree_cmds.rs',
    'tiny-glade/crates/systems/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs',
    'tiny-glade/crates/systems/tree-tool/src/ui_delete_tree.rs',
    'tiny-glade/crates/systems/tree-tool/src/ui_move_tree.rs',
    core+'systems/save/save_game.rs',
]: change(path,swap)
def placement_native_docs(s):
    s=swap(s)
    # Existing binary-derived headers already used native Data correctly;
    # unlike their old implementation imports, those anchors must not swap.
    marker='//! Placement owner' if '//! Placement owner' in s else '//! ## \u26a0'
    prefix, separator, rest=s.partition(marker)
    prefix=re.sub(r'\bPrefabInstance\b','PrefabInstanceData',prefix)
    return prefix+separator+rest
change('tiny-glade/crates/systems/tree-tool/src/tree_move_bend.rs',placement_native_docs)
change('tiny-glade/crates/systems/forest/src/tree_animation.rs',placement_native_docs)
def hide(s):
    s=swap(s)
    s=s.replace('components::{PrefabInstance, TreeAnimation, TreeComponent}',
        'components::{TreeAnimation, TreeComponent}')
    s='use country_core::resources::prefab::PrefabInstance;\n'+s if not 'use country_core::resources::prefab::PrefabInstance;' in s else s
    # Keep the module's inner documentation before any use declaration.
    if s.startswith('use country_core::resources::prefab::PrefabInstance;\n//!'):
        s=s.removeprefix('use country_core::resources::prefab::PrefabInstance;\n')
        at=s.index('\nuse ');s=s[:at]+'\nuse country_core::resources::prefab::PrefabInstance;'+s[at:]
    return s
change('tiny-glade/crates/systems/tree-tool/src/hide_trees.rs',hide)

# The complete replacement producers/consumer and new occupied-cell policy are
# prepared as reviewed snippets in this same evidence directory.
change(core+'startup/spawn_preplaced_trees.rs',lambda _: (OUT/'spawn_preplaced_trees.rs').read_text(encoding='utf8'))
change('tiny-glade/crates/systems/tree-tool/src/convert_preplaced_trees.rs',lambda _: (OUT/'convert_preplaced_trees.rs').read_text(encoding='utf8'))
def wall(s):
    at=s.index('    /// B 0x140aa5850')
    return s[:at]+(OUT/'occupied_cell_method.rs').read_text(encoding='utf8')+'\n'+s[at:]+(OUT/'occupied_cell_tests.rs').read_text(encoding='utf8')
change(core+'systems/wall/wall_state/wall_spatial_hash.rs',wall)

tool='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe'
subprocess.run([tool,'--edition','2024',*[str(p) for _,_,p in changes]],check=True)
patch=''; rows=[]
for path,before,target in changes:
    after=target.read_text(encoding='utf8')
    patch+=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+path,tofile='b/'+path))
    rows.append({'path':path,'base_sha256':hashlib.sha256((ROOT/path).read_bytes()).hexdigest(),
        'proposed_sha256':hashlib.sha256(target.read_bytes()).hexdigest()})
(OUT/'production-review.patch').write_text(patch,encoding='utf8',newline='\n')
(OUT/'candidate-checkpoint.json').write_text(json.dumps({'status':'candidate only; no production writes; no Cargo', 'files':rows},indent=2),encoding='utf8')
print(json.dumps({'candidates':len(rows),'patch':str(OUT/'production-review.patch')}))
