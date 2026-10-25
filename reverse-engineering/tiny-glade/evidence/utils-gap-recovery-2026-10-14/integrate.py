"""Install consumer bridges and fixtures from the retained native outputs."""
from pathlib import Path
import json
OUT=Path(__file__).parent
ROOT=OUT.parents[3]
SRC=ROOT/'tiny-glade/crates'
wrapper=SRC/'country-core/src/utils/index_condenser.rs'
old=wrapper.read_text(encoding='utf8')
tests=old[old.index('#[cfg(test)]'):]
wrapper.write_text('''//! Consumer adapter for the shared recovered index contract. B insert inline
//! IPI0x74ee rejects occupied IDs; get/removal repair stale hints. The original
//! typed marker remains local, while ID/state policy lives in the utils crate.
use core::{cell::Cell, marker::PhantomData};
use ::utils::index_condenser::{Condex, IndexCondenser as SharedCondenser};

pub struct IndexCondenser<T> {
    shared: SharedCondenser<u32>,
    _marker: PhantomData<fn() -> T>,
}
impl<T> Default for IndexCondenser<T> {
    fn default() -> Self { Self::new() }
}
impl<T> IndexCondenser<T> {
    pub fn new() -> Self { Self { shared: SharedCondenser::default(), _marker: PhantomData } }
    pub fn len(&self) -> usize { self.shared.len() }
    pub fn is_empty(&self) -> bool { self.shared.is_empty() }
    pub fn contains_id(&self, id: u32) -> bool { self.shared.contains_key(&id) }
    pub fn insert(&mut self, id: u32) -> Option<usize> {
        self.shared.insert(id).map(|key| key.last_known_index() as usize)
    }
    /// Compatibility for callers that already established an unoccupied ID.
    pub fn push(&mut self, id: u32) -> usize {
        self.insert(id).expect("ID already occupied")
    }
    pub fn id_at(&self, dense: usize) -> Option<u32> { self.shared.keys().get(dense).copied() }
    pub fn resolve(&self, id: u32, hint: &mut u32) -> Option<usize> {
        let key = Condex { key: id, cached_index: Cell::new(*hint) };
        let resolved = self.shared.get(&key);
        *hint = key.last_known_index();
        resolved.map(|i| i as usize)
    }
    pub fn remove(&mut self, id: u32, hint: &mut u32) -> Option<usize> {
        let key = Condex { key: id, cached_index: Cell::new(*hint) };
        let removed = self.shared.remove_index(&key);
        *hint = key.last_known_index();
        removed.map(|i| i as usize)
    }
}

'''+tests,encoding='utf8')

wrapper=SRC/'country-core/src/utils/float_grid.rs'
old=wrapper.read_text(encoding='utf8');tests=old[old.index('#[cfg(test)]'):]
wrapper.write_text('''//! RasterBufferRef<512> consumer of shared B bilinear sampling policy.
//! 0x140a14df0: byte values /255, original lerp order and edge extrapolation.
//! Branch byte 0x14362c308 is caller state; its producer remains unresolved.
pub use ::utils::float_grid::{
    CENTERED_PATH, DEFAULT_WORLD_EXTENT, FloatGridCells, GRID_SIDE,
    bilinear_sample_at_world_xz, bilinear_sample_at_world_xz_130, lerp,
};

'''+tests,encoding='utf8')

# Preserve the actual producer's retry-on-insert-failure contract.
p=SRC/'country-core/src/render/nani/gpu_world.rs'
s=p.read_text(encoding='utf8')
start=s.index('        let id = loop {',s.index('    fn allocate('))
end=s.index('    /// 写剔除数据',start)
old=s[start:end]
new='''        loop {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
            if let Some(dense) = self.condenser.insert(id) {
                return Some(InstanceKey {
                    id,
                    index: dense as u32,
                    kind,
                });
            }
        }
    }

'''
s=s[:start]+new+s[end:];p.write_text(s,encoding='utf8')

# Encode original outputs, never calculate expected outputs with candidate code.
r=json.loads((OUT/'index-native-fixtures.json').read_text())
test='use super::*;\n#[test]\nfn original_insert_rejects_duplicate_and_appends_new_key() {\n'
for row in r['insert']:
    result='None' if row['index'] is None else f"Some({row['index']})"
    test+='    {\n        let mut c = IndexCondenser::default();\n'
    for k in row['keys']:test+=f'        c.insert({k}u32).unwrap();\n'
    test+=f"        let h = c.insert({row['key']}u32);\n        assert_eq!(h.map(|v| v.last_known_index()), {result});\n        assert_eq!(c.keys(), &{row['after']['keys']!r});\n"
    for k,v in row['after']['mapping'].items():test+=f'        assert_eq!(c.indices.get(&{k}u32), Some(&{v}));\n'
    test+='    }\n'
test+='}\n#[test]\nfn original_complete_removal_returns_index_and_repairs_mapping() {\n'
for row in r['remove']:
    result='None' if row['index'] is None else f"Some({row['index']})"
    test+='    {\n        let mut c = IndexCondenser::default();\n'
    for k in row['keys']:test+=f'        c.insert({k}u32).unwrap();\n'
    test+=f"        let h = Condex {{ key: {row['key']}u32, cached_index: Cell::new({row['cached']}) }};\n        assert_eq!(c.remove_index(&h), {result});\n        assert_eq!(h.last_known_index(), {row['updated_cached']});\n        assert_eq!(c.keys(), &{row['after']['keys']!r});\n"
    for k,v in row['after']['mapping'].items():test+=f'        assert_eq!(c.indices.get(&{k}u32), Some(&{v}));\n'
    test+='    }\n'
test+='}\n'
(SRC/'utils/src/oracle/index_producer_tests.rs').write_text(test,encoding='utf8')
p=SRC/'utils/src/index_condenser.rs';s=p.read_text();s+='\n#[cfg(test)]\n#[path = "oracle/index_producer_tests.rs"]\nmod producer_oracle;\n';p.write_text(s)

r=json.loads((OUT/'grid-native-fixtures.json').read_text())
test='''use super::*;
#[test]
fn original_byte_grid_both_modes_and_numeric_boundaries() {
    let values: Vec<u8> = (0..512).flat_map(|r| (0..512).map(move |c| ((r*17+c*31+(r*c)%251)%256) as u8)).collect();
    let grid = ByteFloatGrid { values: &values, width: 512, height: 512 };
    let cases: &[(bool, [u32; 4], u32)] = &[
'''
for row in r['rows']:test+=f"        ({'true' if row['mode'] else 'false'}, {row['input']!r}, {row['result']}),\n"
test+='''    ];
    for &(mode, input, expected) in cases {
        let [x,z,ex,ez] = input.map(f32::from_bits);
        let actual = grid.bilinear_sample_at_world_xz(Vec2::new(x,z), Vec2::new(ex,ez), mode);
        assert_eq!(actual.to_bits(), expected, "mode={mode}, input={input:?}");
    }
}
'''
(SRC/'utils/src/oracle/grid_tests.rs').write_text(test,encoding='utf8')
p=SRC/'utils/src/float_grid.rs';s=p.read_text();s+='\n#[cfg(test)]\n#[path = "oracle/grid_tests.rs"]\nmod native_oracle;\n';p.write_text(s)
print('integrated index/raster consumer bridges and native fixture tests')
