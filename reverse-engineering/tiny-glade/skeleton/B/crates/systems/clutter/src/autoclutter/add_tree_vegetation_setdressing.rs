// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/autoclutter/add_tree_vegetation_setdressing.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 78ca186dcd80159b54c33feefa081cd41d202f701ec1b5f4dc4c9166b5be55e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_tree_vegetation_setdressing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3808
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3808,"record_offset":260,"rva":36517888,"va":5405227008}
// 编译实例 RVAs: 0x22d3800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29711113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e8f7a43afe","size":5,"soft_fail":false,"target_va":5375516032,"va":5405227140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26891686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e85aaa65fe","size":5,"soft_fail":false,"target_va":5378336416,"va":5405228097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26891731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e82daa65fe","size":5,"soft_fail":false,"target_va":5378336448,"va":5405228174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25948427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e8f50e74fe","size":5,"soft_fail":false,"target_va":5379280368,"va":5405228790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ae8320
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25082305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e83f4681fe","size":5,"soft_fail":false,"target_va":5380145952,"va":5405228252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23294363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e8658e9cfe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405228886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf7600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6146466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e85e36a2ff","size":5,"soft_fail":false,"target_va":5399082496,"va":5405228957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: add_tree_vegetation_setdressing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6619353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e8d9006500","size":5,"soft_fail":false,"target_va":5411847952,"va":5405228594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28604563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e86d874bfe","size":5,"soft_fail":false,"target_va":5376624768,"va":5405229326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6312596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e894526000","size":5,"soft_fail":false,"target_va":5411542064,"va":5405229463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6312327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e887516000","size":5,"soft_fail":false,"target_va":5411541904,"va":5405229572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::placement_data::PlacementDataSsbo::position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25160379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e8451580fe","size":5,"soft_fail":false,"target_va":5380067744,"va":5405228118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23218844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e864b59dfe","size":5,"soft_fail":false,"target_va":5382009728,"va":5405228567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23316698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e826379cfe","size":5,"soft_fail":false,"target_va":5381911968,"va":5405228661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23316448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e820389cfe","size":5,"soft_fail":false,"target_va":5381911872,"va":5405228315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23321324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405227008,"raw_hex":"e814259cfe","size":5,"soft_fail":false,"target_va":5381907008,"va":5405228327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":4604,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3808,"record_offset":5760,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3808,"record_offset":924,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3808,"record_offset":512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3808,"record_offset":5012,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3808,"record_offset":4800,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3808,"record_offset":4544,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":28175,"source_line":279,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3808,"record_offset":4516,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":31632,"source_line":289,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3808,"record_offset":4576,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3808,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5464,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5352,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3808,"record_offset":5816,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3808,"record_offset":4884,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3808,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3808,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3808,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":4648,"parent_record_offset":4604,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":260,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":708,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3808,"record_offset":676,"parent_record_offset":648,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3808,"record_offset":3960,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3808,"record_offset":2588,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3808,"record_offset":2672,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3808,"record_offset":4156,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3808,"record_offset":2504,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3808,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":260,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":260,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":260,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":260,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":260,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3808,"record_offset":3820,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3808,"record_offset":624,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3808,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3808,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3808,"record_offset":2764,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3808,"record_offset":1356,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3808,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3808,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":260,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3808,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":260,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3808,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":260,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":260,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":260,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":1272,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":260,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":1188,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3808,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":260,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3808,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":260,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3808,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3808,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":260,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3808,"record_offset":568,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3808,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":260,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3808,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":260,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3808,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":260,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3808,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3808,"record_offset":4920,"parent_record_offset":4884,"ancestor_record_offset":260,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3808,"record_offset":5256,"parent_record_offset":5076,"ancestor_record_offset":260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3808,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3808,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3808,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":260,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3808,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3808,"record_offset":1576,"parent_record_offset":1512,"ancestor_record_offset":260,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3808,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":260,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3808,"record_offset":5116,"parent_record_offset":5076,"ancestor_record_offset":260,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3808,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3808,"record_offset":1640,"parent_record_offset":1460,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3808,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3808,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3808,"record_offset":1072,"parent_record_offset":1036,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3808,"record_offset":4768,"parent_record_offset":4604,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3808,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3808,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3808,"record_offset":848,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3808,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3808,"record_offset":1036,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3808,"record_offset":1100,"parent_record_offset":1036,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5564,"parent_record_offset":5528,"ancestor_record_offset":260,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5528,"parent_record_offset":5488,"ancestor_record_offset":260,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3808,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":260,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3808,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":260,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3808,"record_offset":5600,"parent_record_offset":5564,"ancestor_record_offset":260,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3808,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":260,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3808,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":260,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3808,"record_offset":4948,"parent_record_offset":4884,"ancestor_record_offset":260,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/03808.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":3808,"record_offset":5044,"parent_record_offset":5012,"ancestor_record_offset":260,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/03808.jsonl.gz"}
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
// 完整结构及原始类型记录: add_tree_vegetation_setdressing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_tree_vegetation_setdressing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_tree_vegetation_setdressing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing
// 信息 ID: 001e19e103b1fdcf6d79ad8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 383a2ee4060b25921b5ddef0: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > > @ 0x1ce1304 -> 0x22d3800 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TreeComponent>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x1cee96d -> 0x22d3800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
