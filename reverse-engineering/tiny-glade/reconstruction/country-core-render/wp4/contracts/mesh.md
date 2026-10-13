# WP4 Contract: `country_core::render::mesh`

PDB bundle: `skeleton/B/crates/country-core/src/render/mesh.rs` (source_sha256 `e8e64685…`, 30 procedure_instances, modules 476/486/…/537/544/…).
Decompiles: `_analysis/decomp/B_0x140a07390.c …` plus `evidence/building-gates/B-mesh-extra/140a34c90.c`, `evidence/building-gates/B-mesh-helpers/*.c`.

Marking: **O**=Observed (machine code / PDB), **I**=Inference, **U**=Unknown.

## 1. `Mesh` (struct, 0x78 = 120 bytes — O: `AssetMeshLibrary::add` memcpy of 15 u64s)

| offset | field | evidence |
|---|---|---|
| +0x00 | `indices: Vec<u32>` (ptr/cap/len) | O: `set_indices` copies 3 u64s; `retain_vertices` iterates `+0x08`/`+0x10` as len |
| +0x18 | u64 (unbound; init `0x8000000000000000` sentinel in `new`) | O `Mesh::new` |
| +0x20 | u64 (unbound) | I |
| +0x28 | `attributes: HashMap<String, VertexAttributeValues>` (RandomState at +0x28/+0x30, buckets +0x38, mask +0x40, growth +0x48, items +0x50) | O: `set_attribute`/`positions`/`aabb_bounds` read these |
| +0x58 | u64 (RandomState key tail) | I |
| +0x60 | u64 | I |
| +0x68 | u64 | I |
| +0x70 | u64 | I |

Attribute key names observed: `"Vertex_Position"` (len 15, bytes `Vertex_P`+`osition` — O `positions`/`aabb_bounds`/`as_triangles`); `"Vertex_Color"` inferred (`add_color`).

### Functions

- `new(source_file?, …) -> Mesh` (0x140a07390, 166B): builds empty Mesh: empty indices, empty attributes (RandomState), `+0x18 = 0x8000…`, args copied to +0x68/+0x70. O.
- `set_source_file` (0x140a07440, 69B): assigns source-file path string. O (trivial assignment).
- `set_indices(&mut self, &Vec<u32>)` (0x140a07490, 60B): frees old indices Vec, moves {ptr,cap,len}. O.
- `arrayed_indices(&self) -> Vec<[u32;3]>` (0x140a074d0, 104B): `from_iter<[u32;3], Map<ChunksExact<u32>,…>>` — triples the index list. O.
- `add_color(&mut self, color: [f32;3])` (0x140a07540, 316B): look up `Vertex_Position`; `vec::from_elem(color, pos_count)`; `set_attribute<Vec<[f32;3]>>(name, colors)`. O.
- `add_color_self(&mut self)` (0x140a07680, 423B): per-vertex color variant (color = f(position)). O (decompile shows per-vertex computation).
- `aabb_bounds(&self) -> Aabb3` (0x140a07870, 505B): look up `Vertex_Position` (disc 3); empty → `Aabb3 { min:+FLT_MAX, max:-FLT_MAX }`; else seed first point, `Aabb3::expand_to_contain_point` for each. Return = 24B `{min: Vec3, max: Vec3}`. O.
- `collision_bbx(&self) -> …` (0x140a07a70, 181B; decompile `evidence/building-integrate/B-core/140a07a70.c`): collision box (xz-extended) from positions. O.
- `positions(&self) -> (&[u32], &[[f32;3]])` (0x140a07b30, 361B): look up `Vertex_Position` (disc 3, else panic); returns `{indices.ptr, indices.ptr+len*4, pos.ptr, pos.len}`. O.
- `positions_without_indices(&self) -> &[[f32;3]]` (0x140a07ca0): positions only. O.
- `normals_without_indices(&self) -> &[[f32;3]]` (0x140a07e00): `Vertex_Normal` attribute (disc 3). O/I (name inferred from pattern).
- `uvs_without_indices(&self) -> &[[f32;2]]` (0x140a07f50): `Vertex_UV` attribute (disc 2). O/I.
- `retain_vertices(&mut self, &mut [i32])` (0x140a08090, 318B): (1) `indices.retain(keep)`, (2) per attribute `VertexAttributeValues::retain_by_index`, (3) rewrite flags in place: kept → new index, dropped → -1; (4) rewrite every index via remap, panic if remapped -1. O.
- `collision_radius_xz(&self) -> f32` (0x140a081e0, 149B): max over vertices of `sqrt(x²+z²)` (iterates indices → positions). O.
- `as_triangles(&self) -> Vec<utils::geometry::intersect::Triangle>` (0x140a08280, 438B): chunks indices by 3 → `Triangle` (positions), collect. O.
- `clear`, `is_valid`, `has_attribute`, `vertex_count`, `translate_hack_slow_dont_use`, `collision_radius_xz` inline hosts. I/U.
- `set_attribute<Vec<[f32;4]>>` (0x140a34170), `<Vec<[f32;3]>>` (0x140a34c90 — decompiled), `<Vec<[f32;2]>>`, `<Vec<f32>>` (0x140a35b80), `<Vec<i32>>`, `<VertexAttributeValues>` (0x140a35240, 687B — stores a prebuilt enum): if name exists in map → replace value in place (dealloc old, set new discrim); else insert new (name cloned). O. Discriminants: 0/1/2/3 map to f32/f32x2/f32x3/f32x4/i32 variants (I; disc 2 = `[f32;2]`, disc 3 = `[f32;3]` confirmed by uvs/positions).

## 2. `VertexAttributeValues` (enum, O)

Variants (I, by discriminant from set_attribute/positions/uvs): `Float(Vec<f32>)`, `Float2(Vec<[f32;2]>)`, `Float3(Vec<[f32;3]>)`, `Float4(Vec<[f32;4]>)`, `Int(Vec<i32>)`, … (unowned type records absent; field layout per Vec = ptr/cap/len + discriminant).

- `array_length(&self) -> usize` (0x140949170, 5B; PDB also aliases `sync_point_count`): returns Vec len. O.
- `append(&mut self, &VertexAttributeValues)` (0x140a08440, 170B): same-variant dispatch via jump table, panic otherwise (different-variant error fmt). O.
- `retain_by_index<closure>` (0x140a36110): retains elements by closure (used by retain_vertices). O.
- `as_f32/as_f32x2/as_f32x3/as_f32x4/as_i32/as_ptr/component_count/size_in_bytes/stride` — trivial accessors (O/I names, no standalone procs).
- `From<Vec<…>>` impls (0x140a0db00/0x140a0db90, `impl$6/7::from`). O.

## 3. Typed gaps
- Exact names of all attribute keys (only `Vertex_Position` confirmed; `Vertex_Normal`/`Vertex_UV`/`Vertex_Color` inferred).
- `+0x18/+0x20/+0x58/+0x60/+0x68/+0x70` field semantics.
- `VertexAttributeValues` variant count/order beyond discriminants 0..3.
- `translate_hack_slow_dont_use` body (inline).
