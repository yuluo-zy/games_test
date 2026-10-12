// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4102643cdb4dcdeb1dca2ef035db3731361c3a3ff6b7769c10f66fb3f9697d9b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wooden_corners.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1521, 1525, 3142, 3143, 3144
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5>
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3143,"record_offset":280,"rva":38656944,"va":5407366064}
// 编译实例 RVAs: 0x24ddbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1424dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e80ff3ffff","size":5,"soft_fail":false,"target_va":5407362960,"va":5407366268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9254506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e86a368d00","size":5,"soft_fail":false,"target_va":5416621600,"va":5407367089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9255337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e8a9398d00","size":5,"soft_fail":false,"target_va":5416622320,"va":5407366978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e850d7ffff","size":5,"soft_fail":false,"target_va":5407356528,"va":5407366939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e8d8f7ffff","size":5,"soft_fail":false,"target_va":5407364352,"va":5407366435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24053455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e831f990fe","size":5,"soft_fail":false,"target_va":5383313312,"va":5407366762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24043532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407366064,"raw_hex":"e8f41f91fe","size":5,"soft_fail":false,"target_va":5383323216,"va":5407366743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3143,"record_offset":552,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3143,"record_offset":800,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3143,"record_offset":712,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden_corners::select_corner_source
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs
//     evidence={"module":3143,"record_offset":884,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":117763,"source_line":93,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3143,"record_offset":1088,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3143,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3143,"record_offset":1060,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3143,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3143,"record_offset":672,"parent_record_offset":608,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3143,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3143,"record_offset":952,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3143,"record_offset":924,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3143,"record_offset":644,"parent_record_offset":608,"ancestor_record_offset":280,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3143,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":280,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3143,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::select_corner_source
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC03
// 首条位置: {"module":3143,"inlinee":117763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3143,"record_offset":1088,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3143,"record_offset":1060,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3143,"record_offset":952,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3143,"record_offset":924,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03143.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::emit_corner_band
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3142,"record_offset":280,"rva":38654432,"va":5407363552}
// 编译实例 RVAs: 0x24dd1e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wooden_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24038773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407363552,"raw_hex":"e88b3291fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407363824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9016394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407363552,"raw_hex":"e84a948900","size":5,"soft_fail":false,"target_va":5416380336,"va":5407363937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight_with_depth
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27455039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407363552,"raw_hex":"e8c1115dfe","size":5,"soft_fail":false,"target_va":5379908848,"va":5407363882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_flags
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27456069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407363552,"raw_hex":"e8bb0d5dfe","size":5,"soft_fail":false,"target_va":5379907856,"va":5407363920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27454917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407363552,"raw_hex":"e83b125dfe","size":5,"soft_fail":false,"target_va":5379908992,"va":5407363904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":976,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3142,"record_offset":512,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3142,"record_offset":456,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":1116,"parent_record_offset":1064,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":1064,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":1008,"parent_record_offset":976,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3142,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3142,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3142,"record_offset":1216,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3142,"record_offset":1260,"rva":38655232,"va":5407364352}
// 编译实例 RVAs: 0x24dd500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wooden_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24039553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407364352,"raw_hex":"e87f2f91fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5407364604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wooden_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8987120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407364352,"raw_hex":"e8f0218900","size":5,"soft_fail":false,"target_va":5416353040,"va":5407365915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wooden_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9691440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407364352,"raw_hex":"e830e19300","size":5,"soft_fail":false,"target_va":5417057056,"va":5407365611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9255724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407364352,"raw_hex":"e82c3b8d00","size":5,"soft_fail":false,"target_va":5416621600,"va":5407365871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::skewed_box_trigger_vol
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26853717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407364352,"raw_hex":"e8ab3e66fe","size":5,"soft_fail":false,"target_va":5380511776,"va":5407365488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":2352,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3142,"record_offset":2068,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3142,"record_offset":1592,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3142,"record_offset":1492,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3142,"record_offset":1464,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3142,"record_offset":1528,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3142,"record_offset":1620,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3142,"record_offset":1676,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3142,"record_offset":2324,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":1260,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":1260,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1260,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2536,"parent_record_offset":2504,"ancestor_record_offset":1260,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1260,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1260,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3142,"record_offset":2504,"parent_record_offset":2476,"ancestor_record_offset":1260,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":2612,"parent_record_offset":2408,"ancestor_record_offset":1260,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":1260,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":2448,"parent_record_offset":2408,"ancestor_record_offset":1260,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3142,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":1260,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3142,"record_offset":3184,"parent_record_offset":2408,"ancestor_record_offset":1260,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3142,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":1260,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3142,"record_offset":2380,"parent_record_offset":2352,"ancestor_record_offset":1260,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":3142,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1260,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3142,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1260,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":3142,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1260,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3142,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":1260,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3142,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":1260,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3142,"record_offset":2856,"parent_record_offset":2820,"ancestor_record_offset":1260,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3142,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1260,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3142,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1260,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3142,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1260,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3142,"record_offset":2724,"parent_record_offset":2408,"ancestor_record_offset":1260,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3142,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":1260,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3142,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1260,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3142,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3142,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":1260,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3142,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1260,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3142,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":1260,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3142,"record_offset":2284,"parent_record_offset":2116,"ancestor_record_offset":1260,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3142,"record_offset":2996,"parent_record_offset":2820,"ancestor_record_offset":1260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3142,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1260,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs
//     evidence={"module":3142,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1260,"inlinee":80047,"source_line":177,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$1
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs
//     evidence={"module":3142,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":1260,"inlinee":80031,"source_line":188,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x138AF
// 首条位置: {"module":1525,"inlinee":80047}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3142,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03142.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1389F
// 首条位置: {"module":1521,"inlinee":80031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: wooden_corners.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wooden_corners.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wooden_corners.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision
// 信息 ID: 66d19557fc974b7040f16b9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141cc1b -> 0x24dd500 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24ddd23 -> 0x24dd500 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24ddd7e -> 0x24dd500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$0
// 信息 ID: e2c2b0b456a136cf4ca5798f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=2216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision::closure$1
// 信息 ID: 780ece7395edc2f44d499339
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=3368; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=3452; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=3536; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=3620; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision; module=3142, inline_record=3704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5>
// 信息 ID: d040b1fc9b418c30b4ec8424
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners -> ["system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141cf73 -> 0x24ddbb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::emit_corner_band
// 信息 ID: 507f9a63d9bc32c82f16c8e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::add_wooden_riser @ 0x24db0a2 -> 0x24dd1e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dbb12 -> 0x24dd1e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dbbac -> 0x24dd1e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden_corners::select_corner_source
// 信息 ID: ab9cd0f79e8c5fe6a073a474
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5>; module=3143, inline_record=884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
