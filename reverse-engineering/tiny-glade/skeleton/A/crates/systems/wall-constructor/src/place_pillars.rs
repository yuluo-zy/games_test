// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/place_pillars.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e7938bb424ac7a8a8e98e76745ffcc79d6dc988a9012d82d1adbaa94c5b9d23d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_pillars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 363, 369, 1226, 1244, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1244,"record_offset":1900,"rva":13401744,"va":5382110864}
// 编译实例 RVAs: 0xcc7e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9361358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e8322871ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382111049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9362110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e8422571ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382111817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21482640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e890cc4701","size":5,"soft_fail":false,"target_va":5403593968,"va":5382111323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20865653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e875623e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382112486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10803417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e827275bff","size":5,"soft_fail":false,"target_va":5371308432,"va":5382111844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3737908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e834093900","size":5,"soft_fail":false,"target_va":5385850032,"va":5382112119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::place_pillars::place_pillars::closure$4
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e8b5f7ffff","size":5,"soft_fail":false,"target_va":5382110096,"va":5382112214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5171827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110864,"raw_hex":"e88d15b1ff","size":5,"soft_fail":false,"target_va":5376939616,"va":5382111438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":2208,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":4120,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1244,"record_offset":4444,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1244,"record_offset":2792,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1244,"record_offset":3912,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1244,"record_offset":2616,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1244,"record_offset":2700,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":5056,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":2176,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":4984,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":5156,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":4604,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1244,"record_offset":4324,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1244,"record_offset":5244,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1244,"record_offset":2468,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1244,"record_offset":2764,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1900,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":1900,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":1900,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1900,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1244,"record_offset":4052,"parent_record_offset":3960,"ancestor_record_offset":1900,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":1900,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":1900,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":3028,"parent_record_offset":2996,"ancestor_record_offset":1900,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":2996,"parent_record_offset":2964,"ancestor_record_offset":1900,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":3184,"parent_record_offset":2996,"ancestor_record_offset":1900,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1244,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":1900,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":4152,"parent_record_offset":4120,"ancestor_record_offset":1900,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":1900,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":2932,"parent_record_offset":2888,"ancestor_record_offset":1900,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":2964,"parent_record_offset":2932,"ancestor_record_offset":1900,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":1900,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1244,"record_offset":3700,"parent_record_offset":3656,"ancestor_record_offset":1900,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1244,"record_offset":3804,"parent_record_offset":3700,"ancestor_record_offset":1900,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1244,"record_offset":3336,"parent_record_offset":2888,"ancestor_record_offset":1900,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1244,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":1900,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1244,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":1900,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1244,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":1900,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1244,"record_offset":3224,"parent_record_offset":2888,"ancestor_record_offset":1900,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1244,"record_offset":3480,"parent_record_offset":3432,"ancestor_record_offset":1900,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1244,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":1900,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1244,"record_offset":3656,"parent_record_offset":3480,"ancestor_record_offset":1900,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1244,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":1900,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":3600,"parent_record_offset":3556,"ancestor_record_offset":1900,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":4692,"parent_record_offset":4660,"ancestor_record_offset":1900,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1244,"record_offset":3556,"parent_record_offset":3512,"ancestor_record_offset":1900,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":5184,"parent_record_offset":5156,"ancestor_record_offset":1900,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":5016,"parent_record_offset":4984,"ancestor_record_offset":1900,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1244,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1900,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1244,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":1900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1244,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":1900,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1244,"record_offset":3848,"parent_record_offset":3432,"ancestor_record_offset":1900,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1244,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":1900,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1244,"record_offset":2732,"parent_record_offset":2700,"ancestor_record_offset":1900,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1244,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":1900,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1244,"record_offset":3756,"parent_record_offset":3700,"ancestor_record_offset":1900,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1244,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1900,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1244,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":1900,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1244,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":1900,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1244,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":1900,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1244,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":1900,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":1900,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1244,"record_offset":4480,"parent_record_offset":4444,"ancestor_record_offset":1900,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1244,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":1900,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1244,"record_offset":5120,"parent_record_offset":5084,"ancestor_record_offset":1900,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1244,"record_offset":3512,"parent_record_offset":3480,"ancestor_record_offset":1900,"inlinee":59620,"source_line":377,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8C8
// 首条位置: {"module":1226,"inlinee":59592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8C6
// 首条位置: {"module":1226,"inlinee":59590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1226,"record_offset":11936,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":13864,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11860,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":11860,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":11860,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12304,"parent_record_offset":12204,"ancestor_record_offset":11860,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":14004,"parent_record_offset":13912,"ancestor_record_offset":11860,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":11860,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12204,"parent_record_offset":12172,"ancestor_record_offset":11860,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":11860,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12424,"parent_record_offset":12172,"ancestor_record_offset":11860,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":11860,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12600,"parent_record_offset":12560,"ancestor_record_offset":11860,"inlinee":7706,"source_line":124,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12044,"parent_record_offset":12008,"ancestor_record_offset":11860,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":11860,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":11972,"parent_record_offset":11936,"ancestor_record_offset":11860,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1226,"record_offset":12140,"parent_record_offset":12044,"ancestor_record_offset":11860,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1226,"record_offset":12240,"parent_record_offset":12204,"ancestor_record_offset":11860,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1226,"record_offset":12268,"parent_record_offset":12240,"ancestor_record_offset":11860,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1226,"record_offset":12108,"parent_record_offset":12084,"ancestor_record_offset":11860,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1226,"record_offset":13040,"parent_record_offset":13008,"ancestor_record_offset":11860,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1226,"record_offset":13008,"parent_record_offset":12600,"ancestor_record_offset":11860,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1226,"record_offset":13232,"parent_record_offset":12600,"ancestor_record_offset":11860,"inlinee":6259,"source_line":493,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1226,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":11860,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1226,"record_offset":12084,"parent_record_offset":12044,"ancestor_record_offset":11860,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":13888,"parent_record_offset":13864,"ancestor_record_offset":11860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1226,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":11860,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1226,"record_offset":13104,"parent_record_offset":13068,"ancestor_record_offset":11860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":11860,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1226,"record_offset":13132,"parent_record_offset":13068,"ancestor_record_offset":11860,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1226,"record_offset":13068,"parent_record_offset":13040,"ancestor_record_offset":11860,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1226,"record_offset":13200,"parent_record_offset":12600,"ancestor_record_offset":11860,"inlinee":59592,"source_line":43,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53A3
// 首条位置: {"module":363,"inlinee":21411}
// 编译实例 RVAs: 0x4e6770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2539157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847408,"raw_hex":"e86b41d9ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5373847584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29131158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847408,"raw_hex":"e89681bc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373847717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":363,"record_offset":16732,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":363,"record_offset":17068,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":363,"record_offset":17032,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":363,"record_offset":16884,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":363,"record_offset":16856,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":363,"record_offset":17004,"parent_record_offset":16656,"ancestor_record_offset":16656,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":363,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":16656,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":363,"record_offset":16768,"parent_record_offset":16732,"ancestor_record_offset":16656,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":363,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":16656,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":363,"record_offset":16940,"parent_record_offset":16912,"ancestor_record_offset":16656,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":363,"record_offset":16912,"parent_record_offset":16884,"ancestor_record_offset":16656,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":363,"record_offset":16824,"parent_record_offset":16732,"ancestor_record_offset":16656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00363.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":369,"record_offset":3924,"rva":5138624,"va":5373847744}
// 编译实例 RVAs: 0x4e68c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4236183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8695cbfff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373849698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4038444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8d460c2ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5373851991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1103274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8562aefff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373852981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14049d3c0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-303073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e81f60fbff","size":5,"soft_fail":false,"target_va":5373547456,"va":5373850524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3092167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8c72e2f00","size":5,"soft_fail":false,"target_va":5376943968,"va":5373851796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb310
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4081282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e882463e00","size":5,"soft_fail":false,"target_va":5377930000,"va":5373848713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8018256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e850597a00","size":5,"soft_fail":false,"target_va":5381870240,"va":5373851979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8063999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8ff0b7b00","size":5,"soft_fail":false,"target_va":5381915200,"va":5373851196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8067697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8711a7b00","size":5,"soft_fail":false,"target_va":5381919744,"va":5373852042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb1700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8169389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8ada77c00","size":5,"soft_fail":false,"target_va":5382018816,"va":5373849422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc66b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8253945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8f9f17d00","size":5,"soft_fail":false,"target_va":5382104752,"va":5373850802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8256395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e88bfb7d00","size":5,"soft_fail":false,"target_va":5382107168,"va":5373850768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8258527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8df037e00","size":5,"soft_fail":false,"target_va":5382110864,"va":5373852332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29742074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8fad3c501","size":5,"soft_fail":false,"target_va":5403593344,"va":5373851265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29070895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e82f96bb01","size":5,"soft_fail":false,"target_va":5402919776,"va":5373848876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2543994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8862ed9ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5373851093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29124881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e81169bc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373853802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29125154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8226abc01","size":5,"soft_fail":false,"target_va":5402978912,"va":5373853753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t29125483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e86b6bbc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373853392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29201251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e86393bd01","size":5,"soft_fail":false,"target_va":5403054624,"va":5373853368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29125964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e84c6dbc01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373853183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29126219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e84b6ebc01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373853120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29125275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e89b6abc01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373853664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e823e5ffff","size":5,"soft_fail":false,"target_va":5373846128,"va":5373853000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29131795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e81384bc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373848328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8cbf9ffff","size":5,"soft_fail":false,"target_va":5373846320,"va":5373847904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29132670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e87e87bc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373847981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1122211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8a31f1100","size":5,"soft_fail":false,"target_va":5374974848,"va":5373852632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1122219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8ab1f1100","size":5,"soft_fail":false,"target_va":5374974832,"va":5373852608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1121935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e88f1e1100","size":5,"soft_fail":false,"target_va":5374974624,"va":5373852684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3500938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e88a6b3500","size":5,"soft_fail":false,"target_va":5377353776,"va":5373852833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t29117626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8ba4cbc01","size":5,"soft_fail":false,"target_va":5402968992,"va":5373851361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t29118688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8e050bc01","size":5,"soft_fail":false,"target_va":5402970800,"va":5373852107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2508413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e87d462600","size":5,"soft_fail":false,"target_va":5376356912,"va":5373848494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2512299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8ab552600","size":5,"soft_fail":false,"target_va":5376360560,"va":5373848256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2508262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8e6452600","size":5,"soft_fail":false,"target_va":5376356144,"va":5373847877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8017143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8f7547a00","size":5,"soft_fail":false,"target_va":5381865424,"va":5373848276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8177798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e886c87c00","size":5,"soft_fail":false,"target_va":5382028352,"va":5373850549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8177655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8f7c77c00","size":5,"soft_fail":false,"target_va":5382028304,"va":5373850644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8177838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373847744,"raw_hex":"e8aec87c00","size":5,"soft_fail":false,"target_va":5382028336,"va":5373850493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373847744,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373851296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":10952,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":12918,"source_line":2673,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":13540,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":15104,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":14060,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":13808,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":8032,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":6836,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":369,"record_offset":19780,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":369,"record_offset":19808,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":369,"record_offset":13652,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":369,"record_offset":7436,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":369,"record_offset":7380,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":369,"record_offset":13836,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":15472,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":369,"record_offset":18328,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":369,"record_offset":11264,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":369,"record_offset":10860,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21341,"source_line":2912,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":11348,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":369,"record_offset":15232,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":369,"record_offset":15508,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":369,"record_offset":14280,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":369,"record_offset":19836,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":369,"record_offset":19696,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":5788,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":4200,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":4776,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":369,"record_offset":12492,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":369,"record_offset":11428,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::emit_pillar_at_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":369,"record_offset":16728,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21490,"source_line":277,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":369,"record_offset":14336,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21409,"source_line":1081,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":369,"record_offset":14564,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":369,"record_offset":14508,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":369,"record_offset":14592,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":369,"record_offset":14252,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":369,"record_offset":14308,"parent_record_offset":3924,"ancestor_record_offset":3924,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":369,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":3924,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":369,"record_offset":20184,"parent_record_offset":20084,"ancestor_record_offset":3924,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":369,"record_offset":20116,"parent_record_offset":20084,"ancestor_record_offset":3924,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":369,"record_offset":8680,"parent_record_offset":8648,"ancestor_record_offset":3924,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":369,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":3924,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":369,"record_offset":20084,"parent_record_offset":20056,"ancestor_record_offset":3924,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":8648,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":3924,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":3924,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":9136,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":8592,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":8712,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":3924,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":3924,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":369,"record_offset":11032,"parent_record_offset":10976,"ancestor_record_offset":3924,"inlinee":8528,"source_line":2674,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":369,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":3924,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":3924,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":6900,"parent_record_offset":6860,"ancestor_record_offset":3924,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":3924,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::size_hint
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":3924,"inlinee":21339,"source_line":774,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":3924,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::size_hint
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":369,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":3924,"inlinee":21340,"source_line":858,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":369,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":3924,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":3924,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":3924,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":3924,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6128,"parent_record_offset":6100,"ancestor_record_offset":3924,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6160,"parent_record_offset":6128,"ancestor_record_offset":3924,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6332,"parent_record_offset":6160,"ancestor_record_offset":3924,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":369,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":3924,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":369,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":3924,"inlinee":8530,"source_line":1709,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":369,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":3924,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":369,"record_offset":16860,"parent_record_offset":16728,"ancestor_record_offset":3924,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":369,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":3924,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":369,"record_offset":11816,"parent_record_offset":11792,"ancestor_record_offset":3924,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":3924,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":3924,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17084,"parent_record_offset":17028,"ancestor_record_offset":3924,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":369,"record_offset":14476,"parent_record_offset":14336,"ancestor_record_offset":3924,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":369,"record_offset":11200,"parent_record_offset":10976,"ancestor_record_offset":3924,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":369,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":3924,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":369,"record_offset":8816,"parent_record_offset":8788,"ancestor_record_offset":3924,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":369,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":3924,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":369,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":3924,"inlinee":8623,"source_line":151,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":369,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":3924,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":369,"record_offset":8740,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":369,"record_offset":8128,"parent_record_offset":8080,"ancestor_record_offset":3924,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":369,"record_offset":13996,"parent_record_offset":13972,"ancestor_record_offset":3924,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":369,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":3924,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":369,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":3924,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":14384,"parent_record_offset":14336,"ancestor_record_offset":3924,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":3924,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":3924,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":8512,"parent_record_offset":8212,"ancestor_record_offset":3924,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":6972,"parent_record_offset":6924,"ancestor_record_offset":3924,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":369,"record_offset":20148,"parent_record_offset":20116,"ancestor_record_offset":3924,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":369,"record_offset":14688,"parent_record_offset":14640,"ancestor_record_offset":3924,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":369,"record_offset":4412,"parent_record_offset":4316,"ancestor_record_offset":3924,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":369,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":3924,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":369,"record_offset":14948,"parent_record_offset":14916,"ancestor_record_offset":3924,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":369,"record_offset":15040,"parent_record_offset":15008,"ancestor_record_offset":3924,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":369,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":3924,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":369,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":3924,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":3924,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":3924,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":15788,"parent_record_offset":15508,"ancestor_record_offset":3924,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":5372,"parent_record_offset":4912,"ancestor_record_offset":3924,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":6072,"parent_record_offset":5896,"ancestor_record_offset":3924,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":8896,"parent_record_offset":8816,"ancestor_record_offset":3924,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":14440,"parent_record_offset":14336,"ancestor_record_offset":3924,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":369,"record_offset":15820,"parent_record_offset":15508,"ancestor_record_offset":3924,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":369,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":3924,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":369,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":3924,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":369,"record_offset":6396,"parent_record_offset":6100,"ancestor_record_offset":3924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":369,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":3924,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":369,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":3924,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":369,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":3924,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":369,"record_offset":9020,"parent_record_offset":8996,"ancestor_record_offset":3924,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":369,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":3924,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":369,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3924,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":369,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":3924,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":369,"record_offset":5332,"parent_record_offset":5300,"ancestor_record_offset":3924,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":369,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":3924,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":369,"record_offset":6796,"parent_record_offset":5860,"ancestor_record_offset":3924,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":369,"record_offset":15700,"parent_record_offset":15616,"ancestor_record_offset":3924,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":369,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":3924,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":369,"record_offset":15384,"parent_record_offset":15356,"ancestor_record_offset":3924,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":369,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":3924,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":369,"record_offset":4348,"parent_record_offset":4316,"ancestor_record_offset":3924,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":369,"record_offset":7224,"parent_record_offset":6924,"ancestor_record_offset":3924,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":369,"record_offset":15760,"parent_record_offset":15508,"ancestor_record_offset":3924,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":369,"record_offset":14088,"parent_record_offset":14060,"ancestor_record_offset":3924,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":369,"record_offset":4376,"parent_record_offset":4348,"ancestor_record_offset":3924,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":369,"record_offset":14116,"parent_record_offset":14088,"ancestor_record_offset":3924,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":369,"record_offset":7096,"parent_record_offset":7072,"ancestor_record_offset":3924,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":369,"record_offset":7072,"parent_record_offset":7036,"ancestor_record_offset":3924,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":369,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":3924,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":8996,"parent_record_offset":8972,"ancestor_record_offset":3924,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3924,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":3924,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":8972,"parent_record_offset":8920,"ancestor_record_offset":3924,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":8260,"parent_record_offset":8212,"ancestor_record_offset":3924,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":369,"record_offset":5076,"parent_record_offset":4968,"ancestor_record_offset":3924,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":369,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":3924,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":369,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":3924,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":369,"record_offset":4808,"parent_record_offset":4776,"ancestor_record_offset":3924,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":369,"record_offset":4940,"parent_record_offset":4912,"ancestor_record_offset":3924,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":369,"record_offset":4912,"parent_record_offset":4880,"ancestor_record_offset":3924,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":369,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":3924,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":369,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":3924,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":369,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":3924,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":369,"record_offset":5404,"parent_record_offset":4880,"ancestor_record_offset":3924,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4512,"parent_record_offset":4480,"ancestor_record_offset":3924,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4672,"parent_record_offset":4512,"ancestor_record_offset":3924,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3924,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":15328,"parent_record_offset":15300,"ancestor_record_offset":3924,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4232,"parent_record_offset":4200,"ancestor_record_offset":3924,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3924,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4288,"parent_record_offset":4256,"ancestor_record_offset":3924,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":369,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":3924,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":369,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":3924,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":369,"record_offset":5216,"parent_record_offset":4968,"ancestor_record_offset":3924,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":369,"record_offset":5188,"parent_record_offset":4968,"ancestor_record_offset":3924,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":369,"record_offset":5020,"parent_record_offset":4968,"ancestor_record_offset":3924,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":18408,"parent_record_offset":18376,"ancestor_record_offset":3924,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":15644,"parent_record_offset":15616,"ancestor_record_offset":3924,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":19720,"parent_record_offset":19696,"ancestor_record_offset":3924,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":3924,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":3924,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":369,"record_offset":19744,"parent_record_offset":19720,"ancestor_record_offset":3924,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":369,"record_offset":15260,"parent_record_offset":15232,"ancestor_record_offset":3924,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":369,"record_offset":15848,"parent_record_offset":15820,"ancestor_record_offset":3924,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":369,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":3924,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":369,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":3924,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":369,"record_offset":17116,"parent_record_offset":16728,"ancestor_record_offset":3924,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":369,"record_offset":17228,"parent_record_offset":16728,"ancestor_record_offset":3924,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":3924,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17284,"parent_record_offset":17260,"ancestor_record_offset":3924,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17340,"parent_record_offset":17316,"ancestor_record_offset":3924,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":3924,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17260,"parent_record_offset":17228,"ancestor_record_offset":3924,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17316,"parent_record_offset":17284,"ancestor_record_offset":3924,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17028,"parent_record_offset":16728,"ancestor_record_offset":3924,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":3924,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17424,"parent_record_offset":17340,"ancestor_record_offset":3924,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":3924,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":3924,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":369,"record_offset":15008,"parent_record_offset":14592,"ancestor_record_offset":3924,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":369,"record_offset":14828,"parent_record_offset":14592,"ancestor_record_offset":3924,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":369,"record_offset":14916,"parent_record_offset":14592,"ancestor_record_offset":3924,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":369,"record_offset":17684,"parent_record_offset":17600,"ancestor_record_offset":3924,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":369,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":3924,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":369,"record_offset":11496,"parent_record_offset":11460,"ancestor_record_offset":3924,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":369,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":3924,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":369,"record_offset":12224,"parent_record_offset":12000,"ancestor_record_offset":3924,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":369,"record_offset":11552,"parent_record_offset":11520,"ancestor_record_offset":3924,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":3924,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12112,"parent_record_offset":12000,"ancestor_record_offset":3924,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":3924,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12432,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12028,"parent_record_offset":12000,"ancestor_record_offset":3924,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":3924,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":11580,"parent_record_offset":11520,"ancestor_record_offset":3924,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":3924,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":369,"record_offset":11880,"parent_record_offset":11848,"ancestor_record_offset":3924,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":369,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":3924,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":369,"record_offset":11968,"parent_record_offset":11848,"ancestor_record_offset":3924,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":369,"record_offset":11848,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":11680,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":369,"record_offset":12000,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":369,"record_offset":11708,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":369,"record_offset":11792,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":369,"record_offset":12340,"parent_record_offset":11628,"ancestor_record_offset":3924,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":369,"record_offset":15616,"parent_record_offset":15508,"ancestor_record_offset":3924,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":369,"record_offset":18376,"parent_record_offset":18352,"ancestor_record_offset":3924,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":5824,"parent_record_offset":5788,"ancestor_record_offset":3924,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":369,"record_offset":5936,"parent_record_offset":5896,"ancestor_record_offset":3924,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":6100,"parent_record_offset":5896,"ancestor_record_offset":3924,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":369,"record_offset":6368,"parent_record_offset":6100,"ancestor_record_offset":3924,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":369,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":3924,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":369,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":3924,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":369,"record_offset":5996,"parent_record_offset":5964,"ancestor_record_offset":3924,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":369,"record_offset":5896,"parent_record_offset":5860,"ancestor_record_offset":3924,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":369,"record_offset":5860,"parent_record_offset":5824,"ancestor_record_offset":3924,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::place_pillars::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":369,"record_offset":18296,"parent_record_offset":16728,"ancestor_record_offset":3924,"inlinee":21511,"source_line":182,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":369,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":3924,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":369,"record_offset":14640,"parent_record_offset":14592,"ancestor_record_offset":3924,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5407
// 首条位置: {"module":369,"inlinee":21511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1244,"record_offset":260,"rva":13400976,"va":5382110096}
// 编译实例 RVAs: 0xcc7b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12297105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110096,"raw_hex":"e86f5c44ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382110652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5166253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110096,"raw_hex":"e8532bb1ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382110216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-191774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110096,"raw_hex":"e8e212fdff","size":5,"soft_fail":false,"target_va":5381918976,"va":5382110745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-240536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382110096,"raw_hex":"e86854fcff","size":5,"soft_fail":false,"target_va":5381870112,"va":5382110643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":468,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59852,"source_line":146,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":496,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":45962,"source_line":650,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":424,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":1616,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1244,"record_offset":1868,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":752,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":876,"parent_record_offset":800,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":1536,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":36060,"source_line":82,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":536,"parent_record_offset":496,"ancestor_record_offset":260,"inlinee":45963,"source_line":515,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":648,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":45966,"source_line":498,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1244,"record_offset":600,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":45965,"source_line":665,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":1728,"parent_record_offset":1688,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":1688,"parent_record_offset":1648,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":1648,"parent_record_offset":1616,"ancestor_record_offset":260,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":1116,"parent_record_offset":1080,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1244,"record_offset":712,"parent_record_offset":648,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1244,"record_offset":1768,"parent_record_offset":1728,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::emit_pillar_at_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53F2
// 首条位置: {"module":369,"inlinee":21490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":369,"record_offset":16860,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":369,"record_offset":17116,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":21492,"source_line":127,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":369,"record_offset":17228,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":21497,"source_line":538,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17028,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":369,"record_offset":16792,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::place_pillars::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":369,"record_offset":18296,"parent_record_offset":16728,"ancestor_record_offset":16728,"inlinee":21511,"source_line":182,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":16728,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":16728,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":369,"record_offset":17084,"parent_record_offset":17028,"ancestor_record_offset":16728,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":369,"record_offset":16828,"parent_record_offset":16792,"ancestor_record_offset":16728,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":16728,"inlinee":21494,"source_line":148,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17284,"parent_record_offset":17260,"ancestor_record_offset":16728,"inlinee":21500,"source_line":526,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17340,"parent_record_offset":17316,"ancestor_record_offset":16728,"inlinee":21503,"source_line":516,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":16728,"inlinee":21495,"source_line":66,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17260,"parent_record_offset":17228,"ancestor_record_offset":16728,"inlinee":21499,"source_line":661,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":369,"record_offset":17316,"parent_record_offset":17284,"ancestor_record_offset":16728,"inlinee":21502,"source_line":676,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":16728,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$13::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17424,"parent_record_offset":17340,"ancestor_record_offset":16728,"inlinee":21507,"source_line":1077,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":16728,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$8::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":369,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":16728,"inlinee":21505,"source_line":1039,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":369,"record_offset":17684,"parent_record_offset":17600,"ancestor_record_offset":16728,"inlinee":21509,"source_line":178,"archive":"_pdb/A/modules/00369.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E4
// 首条位置: {"module":1226,"inlinee":59620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1244,"record_offset":3556,"parent_record_offset":3512,"ancestor_record_offset":3512,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/01244.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1244,"record_offset":3600,"parent_record_offset":3556,"ancestor_record_offset":3512,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01244.jsonl.gz"}
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
// 完整结构及原始类型记录: place_pillars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarPlacementProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_wall_constructor::place_pillars::PillarPlacementProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarProposals
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_pillars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_pillars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 信息 ID: cc36d5cf1496250d5430d352
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8d3f -> 0x4e6770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8f42 -> 0x4e6770 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=363, inline_record=16656; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=363, inline_record=18124; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 信息 ID: 37512e4c4ecb25cc11336587
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::place_pillars::assemble_stone_pillar", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; module=1244, inline_record=3512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 信息 ID: a5b503a01cad4d55dc6bd497
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::place_pillars::assemble_stone_pillar -> ["system_wall_constructor::place_pillars::place_pillars::closure_env$4"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e7aac -> 0xcc7e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::emit_pillar_at_point
// 信息 ID: f6cd578a7b7f05eaf762bc06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=16728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$1::clone
// 信息 ID: 3bb901f7e04ea3f93c0613d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1226, inline_record=13200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$2::clone
// 信息 ID: 5b17abbee716fb2841abbe33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1226, inline_record=11860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars
// 信息 ID: fc007278a4cbfa071ec66470
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] caefaf91b7f06c253b77db09: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> @ 0x19e5b -> 0x4e68c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> > @ 0xb3d70 -> 0x4e68c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$3
// 信息 ID: 26066c1f208fbee4bef567c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=18296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$4
// 信息 ID: 10c31dd8fba5da3d6d4b6076
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::place_pillars::place_pillars::closure_env$4 <- system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0xcc83d6 -> 0xcc7b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0xcc8462 -> 0xcc7b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
