from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');here=Path(__file__).parent;out=here/'candidate';rows={};patch=[]
def save(rel,new):
 p=root/rel;old=p.read_bytes() if p.exists() else b'';dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()}
 patch.append(''.join(line if line.endswith('\n') else line+'\n\\ No newline at end of file\n' for line in difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)))
rel='crates/systems/critters/src/shared.rs';old=(root/rel).read_bytes();nl=b'\r\n' if b'\r\n' in old else b'\n';save(rel,(here/'shared.rs').read_bytes().replace(b'\r\n',b'\n').replace(b'\n',nl))
rel='crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs';old=(root/rel).read_bytes();needle=b'    pub fn segments_in_cell(';pos=old.index(needle);nl=b'\r\n' if old[old.index(b'\n',pos)-1:old.index(b'\n',pos)]==b'\r' else b'\n'
method='''    /// B query140aa5730/273B: only the point's current cell, no clamping,
    /// radius, sorting, wall deduplication or whole-wall expansion. Float to
    /// i32 uses native Rust saturation (NaN ->0); -65 is included, +65 outside.
    pub fn segments_at_world_xz(&self, position: [f32; 2]) -> impl Iterator<Item = &WallSpatialSegment> {
        let cell = crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped(position);
        let valid = cell[0] >= 0 && cell[1] >= 0
            && cell[0] < GRID_SIDE as i32 && cell[1] < GRID_SIDE as i32;
        valid.then_some(cell[1].wrapping_mul(GRID_SIDE as i32).wrapping_add(cell[0]) as usize)
            .into_iter().flat_map(move |index| self.segments_in_cell(index))
    }

'''.encode().replace(b'\n',nl)
save(rel,old[:pos]+method+old[pos:])
for local,rel in [('movement.tests.rs','crates/systems/critters/tests/original_shared_movement.rs'),('point-query.tests.rs','crates/country-core/tests/original_wall_point_query.rs')]:
 if (here/local).exists():save(rel,(here/local).read_bytes())
(here/'candidate.patch').write_bytes(''.join(patch).encode())
(here/'manifest.json').write_text(json.dumps({'created':datetime.datetime.now().isoformat(),'scope':'evidence-only shared game algorithms, canonical world owners; no startup/plugin production changes','files':rows,'verification':'original per-entity movement71 cases:70return/1missing-public panic; Rust candidate unexecuted, original source ranges/hash in original.json'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
