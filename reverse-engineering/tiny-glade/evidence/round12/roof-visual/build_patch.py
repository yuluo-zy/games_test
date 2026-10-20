"""Prepare a reviewable patch only; never write production files."""
from pathlib import Path
import difflib, hashlib, json
root = Path(__file__).resolve().parent
repo = root.parents[4]
changes = {}
def change(rel, text):
    old = (repo / rel).read_text(encoding='utf8')
    if old == text: return
    changes[rel] = (old, text)
def replace(rel, before, after):
    text = changes.get(rel, ((repo/rel).read_text(encoding='utf8'), None))[0]
    if rel in changes: text = changes[rel][1]
    assert before in text, (rel,before)
    change(rel, text.replace(before, after))

base='tiny-glade/crates/'
for name in ['assemble_roof_tiles','generate_roof_stone_floor_and_roof_bottom']:
    change(base+f'systems/roof/src/visual/{name}.rs',(root/f'{name}.rs').read_text(encoding='utf8'))
replace(base+'country-core/src/resources/roofs/commands.rs',
    '#[derive(Event, Clone, Copy, Debug)]\npub struct RecalculateRoofCmd(pub WallId);',
    'pub use RoofGeometryChanged as RecalculateRoofCmd;')
rel=base+'systems/roof/src/create_roof.rs'
replace(rel,'use country_core::utils::components::transform::Transform;',
    '''use country_core::utils::components::transform::Transform;
use country_core::components::drawable::NaniBundle;
use country_core::render::nani::{
    gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
    instance_data::StoneFloorInstanceData,
    systems::{NaniInstanceDesc, NaniMeshSelector},
};
use country_core::resources::roofs::RoofTiles;
use crate::utils::components::{RoofSpire, RoofSupportBeams};''')
replace(rel,'''                // 1421ab2bf..2f2: zero translation, identity rotation, unit scale.
                Transform::identity(),''', '''                RoofTiles::default(),
                RoofSpire,
                RoofSupportBeams,
                NaniBundle::<StoneFloorInstanceData> {
                    desc: NaniInstanceDesc {
                        mesh: NaniMeshSelector::AtlasPart(0),
                        subset_kind: GpuWorldSubsetKind::RoofFloor,
                    },
                    // Original bundle stores the identity transform here.
                    transform: Transform::identity(),
                    marker: InstanceDataMarker::default(),
                },''')
replace(base+'systems/clutter/src/inter_roof_merlons/detect_inter_roof_merlons.rs',
    'set.extend(ev_roof_cmd.read().map(|e| e.0));',
    'set.extend(ev_roof_cmd.read().map(|e| e.wall));')
replace(base+'systems/critters/src/wren.rs',
    'for event in roof_changed.read() {\n        dirty.push(event.0);',
    'for event in roof_changed.read() {\n        dirty.push(event.wall);')
# Keep the explicitly unfinished triangle producer compilable after unification.
# It is not installed/accepted by this change; its original algorithm is pending.
replace(base+'systems/roof/src/roof_trigger_volume.rs',
    'walls.roof_entity(event.0)', 'walls.roof_entity(event.wall)')

diff=''
for p,(old,new) in changes.items():
    old_raw=(repo/p).read_bytes().decode('utf8')
    new_raw=new.replace('\r\n','\n')
    if '\r\n' in old_raw: new_raw=new_raw.replace('\n','\r\n')
    diff+=''.join(difflib.unified_diff(old_raw.splitlines(True),new_raw.splitlines(True),fromfile='a/'+p,tofile='b/'+p))
(root/'roof-visual.patch').write_bytes(diff.encode('utf8'))
manifest={}
for p,(old,new) in changes.items():
    before=(repo/p).read_bytes()
    after=new.replace('\r\n','\n')
    if b'\r\n' in before: after=after.replace('\n','\r\n')
    after=after.encode('utf8')
    manifest[p]={'base_raw_sha256':hashlib.sha256(before).hexdigest(),'base_normalized_lf_sha256':hashlib.sha256(old.encode()).hexdigest(),'proposed_raw_sha256':hashlib.sha256(after).hexdigest()}
    out=root/'proposed'/p
    out.parent.mkdir(parents=True,exist_ok=True)
    out.write_bytes(after)
(root/'patch-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(f'Prepared {len(changes)} files; production sources untouched.')
