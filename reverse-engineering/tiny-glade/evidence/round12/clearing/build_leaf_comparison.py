from pathlib import Path
import json,struct
root=Path(__file__).resolve().parent
data=json.loads((root/'original-placement-fixtures.json').read_text())
lines=['extern crate glam;','mod native_placement_policy;','use glam::Vec3;','fn v(b:[u32;3])->Vec3{Vec3::from_array(b.map(f32::from_bits))}','fn main(){']
for row in data['particle_rows']:
    args=[f'v({row["position_bits"]})',f'f32::from_bits({row["scale_bits"]})',f'f32::from_bits({row["rotation_bits"]})',f'v({row["vertex_bits"]})']
    expected='None' if not row['result_tag'] else 'Some('+str(list(struct.unpack('<4I',bytes.fromhex(row['result_payload_hex']))))+')'
    lines.append('assert_eq!(native_placement_policy::particle_emitter_for_vertex('+','.join(args)+'),'+expected+');')
# Verify the complete input domain, not only the54 accepted outputs.
accepted={r['input_index'] for r in data['forest_rows']}
import math
points=[(x,2,z) for x in [-math.inf,-66,-65,-64.999,0,64.999,65,66,math.inf,math.nan] for z in [-66,-65,0,64.999,65,math.nan]]
for i,p in enumerate(points):
    bits=list(struct.unpack('<3I',struct.pack('<3f',*p)))
    lines.append(f'assert_eq!(native_placement_policy::is_interactable_forest_placement(v({bits})),{str(i in accepted).lower()});')
lines+=['println!("60 original forest filter + 8 original particle transform fixtures matched");','}']
(root/'leaf_comparison.rs').write_text('\n'.join(lines),encoding='utf8')
