// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/util_cross_brick_pillar.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d5c2f3e5109929d67dde1d9a5c4b0bb1ef37718b5b46f1cc66b88421c4f48ce4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: util_cross_brick_pillar.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1226, 1264, 3821
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3821,"record_offset":260,"rva":29562704,"va":5398271824}
// 编译实例 RVAs: 0x1c31750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28460448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e860ba4dfe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398273995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25522489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8c78e7afe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398272180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25522805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e88b8d7afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398272512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21328774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e87a8cbafe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398272769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16403752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8d8b205ff","size":5,"soft_fail":false,"target_va":5381870240,"va":5398273987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16355185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e88f7006ff","size":5,"soft_fail":false,"target_va":5381918976,"va":5398274156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: util_cross_brick_pillar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5321938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8d2345100","size":5,"soft_fail":false,"target_va":5403593968,"va":5398272025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4703807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e83fc64700","size":5,"soft_fail":false,"target_va":5402978144,"va":5398274332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12423535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8916e42ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5398273562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20314183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8b907cafe","size":5,"soft_fail":false,"target_va":5377959456,"va":5398273634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20317788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8a4f9c9fe","size":5,"soft_fail":false,"target_va":5377955632,"va":5398273415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20320071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8b9f0c9fe","size":5,"soft_fail":false,"target_va":5377953168,"va":5398273234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21332524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8d47dbafe","size":5,"soft_fail":false,"target_va":5376939616,"va":5398272135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21330709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398271824,"raw_hex":"e8eb84bafe","size":5,"soft_fail":false,"target_va":5376942944,"va":5398273648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3821,"record_offset":3848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3821,"record_offset":2360,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3821,"record_offset":2064,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3821,"record_offset":2452,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3821,"record_offset":688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3821,"record_offset":3876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3821,"record_offset":1856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":3004,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2592,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2892,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2536,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2836,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3821,"record_offset":3152,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3821,"record_offset":660,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3821,"record_offset":3264,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3821,"record_offset":2120,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3821,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3821,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3821,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3821,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3821,"record_offset":1996,"parent_record_offset":1904,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3821,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3821,"record_offset":1032,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3821,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3821,"record_offset":1184,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3821,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3821,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":260,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3821,"record_offset":936,"parent_record_offset":784,"ancestor_record_offset":260,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3821,"record_offset":968,"parent_record_offset":936,"ancestor_record_offset":260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3821,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3821,"record_offset":1688,"parent_record_offset":1652,"ancestor_record_offset":260,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3821,"record_offset":1776,"parent_record_offset":1688,"ancestor_record_offset":260,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3821,"record_offset":1224,"parent_record_offset":784,"ancestor_record_offset":260,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":3821,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":3821,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":260,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3821,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3821,"record_offset":3756,"parent_record_offset":3292,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3821,"record_offset":3556,"parent_record_offset":3504,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3821,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3821,"record_offset":2148,"parent_record_offset":2120,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3821,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3821,"record_offset":824,"parent_record_offset":784,"ancestor_record_offset":260,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3821,"record_offset":1368,"parent_record_offset":1320,"ancestor_record_offset":260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3821,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3821,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":260,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3821,"record_offset":1652,"parent_record_offset":1368,"ancestor_record_offset":260,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3821,"record_offset":1960,"parent_record_offset":1932,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1516,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1600,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":260,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1476,"parent_record_offset":1440,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3821,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3821,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3821,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":260,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3821,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3821,"record_offset":1736,"parent_record_offset":1688,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3821,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3821,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":260,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":3821,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":260,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":3821,"record_offset":3336,"parent_record_offset":3292,"ancestor_record_offset":260,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3821,"record_offset":3408,"parent_record_offset":3336,"ancestor_record_offset":260,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3821,"record_offset":3504,"parent_record_offset":3292,"ancestor_record_offset":260,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3821,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":260,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2676,"parent_record_offset":2644,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3821,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3821,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3821,"record_offset":1440,"parent_record_offset":1404,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3821,"record_offset":2708,"parent_record_offset":2676,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/util_cross_brick_pillar.rs
//     evidence={"module":3821,"record_offset":1404,"parent_record_offset":1368,"ancestor_record_offset":260,"inlinee":59613,"source_line":32,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs
//     evidence={"module":3821,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":260,"inlinee":111655,"source_line":132,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8DD
// 首条位置: {"module":1226,"inlinee":59613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3821,"record_offset":1440,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1516,"parent_record_offset":1440,"ancestor_record_offset":1404,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1600,"parent_record_offset":1440,"ancestor_record_offset":1404,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3821,"record_offset":1476,"parent_record_offset":1440,"ancestor_record_offset":1404,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03821.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: util_cross_brick_pillar.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: util_cross_brick_pillar.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: util_cross_brick_pillar.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar::closure$0
// 信息 ID: 3b1d09a86250a6fb85694d0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> >; module=3821, inline_record=1404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> >
// 信息 ID: 41ee2bd32334dcec21af0297
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar -> ["ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base @ 0x177473c -> 0x1c31750 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns @ 0x1774e48 -> 0x1c31750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
