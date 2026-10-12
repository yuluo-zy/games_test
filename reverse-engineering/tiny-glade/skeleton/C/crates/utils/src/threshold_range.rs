// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/threshold_range.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 44d08e679eabfba5736badba798a7aae36dbd52626f7a715b422892cc2a34e67
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: threshold_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 399, 1471
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
// 函数符号: utils::threshold_range::threshold_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x665E
// 首条位置: {"module":399,"inlinee":26206}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":399,"record_offset":13172,"parent_record_offset":13124,"ancestor_record_offset":13124,"inlinee":26207,"source_line":109,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":399,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":13124,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":399,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":13124,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":399,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":13124,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":399,"record_offset":13292,"parent_record_offset":13256,"ancestor_record_offset":13124,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":399,"record_offset":13232,"parent_record_offset":13172,"ancestor_record_offset":13124,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":399,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":13124,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":399,"record_offset":13204,"parent_record_offset":13172,"ancestor_record_offset":13124,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":399,"record_offset":13404,"parent_record_offset":13256,"ancestor_record_offset":13124,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00399.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1471,"record_offset":280,"rva":21077680,"va":5389786800}
// 编译实例 RVAs: 0x1419eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20245380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389786800,"raw_hex":"e87c14cbfe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389787375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141474360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t369177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389786800,"raw_hex":"e819a20500","size":5,"soft_fail":false,"target_va":5390156640,"va":5389787458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26563544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389786800,"raw_hex":"e8d8539501","size":5,"soft_fail":false,"target_va":5416351056,"va":5389787507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26834682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389786800,"raw_hex":"e8fa769901","size":5,"soft_fail":false,"target_va":5416622320,"va":5389787633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1471,"record_offset":732,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79522,"source_line":35,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1420,"parent_record_offset":1388,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1388,"parent_record_offset":1352,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1328,"parent_record_offset":732,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1471,"record_offset":1084,"parent_record_offset":1044,"ancestor_record_offset":280,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1471,"record_offset":1044,"parent_record_offset":992,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1471,"record_offset":852,"parent_record_offset":816,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1471,"record_offset":1232,"parent_record_offset":1084,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1471,"record_offset":916,"parent_record_offset":732,"ancestor_record_offset":280,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1200,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1168,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1471,"record_offset":1508,"parent_record_offset":1352,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":880,"parent_record_offset":816,"ancestor_record_offset":280,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":788,"parent_record_offset":732,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":280,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1471,"record_offset":1124,"parent_record_offset":1084,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1471,"record_offset":1276,"parent_record_offset":992,"ancestor_record_offset":280,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1471,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":280,"inlinee":79524,"source_line":40,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1471,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":280,"inlinee":36143,"source_line":1082,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1471,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":35826,"source_line":92,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1471,"record_offset":2488,"rva":21079376,"va":5389788496}
// 编译实例 RVAs: 0x141a550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14140b2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e85c0affff","size":5,"soft_fail":false,"target_va":5389726432,"va":5389789311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140b680
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e8060effff","size":5,"soft_fail":false,"target_va":5389727360,"va":5389789301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26563648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e840549501","size":5,"soft_fail":false,"target_va":5416353040,"va":5389789387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27267776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e8c012a001","size":5,"soft_fail":false,"target_va":5417057056,"va":5389789275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8554402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e85e787dff","size":5,"soft_fail":false,"target_va":5381234640,"va":5389789037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12713217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e8ff023eff","size":5,"soft_fail":false,"target_va":5377075712,"va":5389788924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-74222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e812defeff","size":5,"soft_fail":false,"target_va":5389714944,"va":5389789161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-86650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389788496,"raw_hex":"e886adfeff","size":5,"soft_fail":false,"target_va":5389702160,"va":5389788805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1471,"record_offset":2832,"parent_record_offset":2488,"ancestor_record_offset":2488,"inlinee":79527,"source_line":551,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":2488,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":2488,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":3388,"parent_record_offset":3352,"ancestor_record_offset":2488,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":3352,"parent_record_offset":3320,"ancestor_record_offset":2488,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":3248,"parent_record_offset":3204,"ancestor_record_offset":2488,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":3508,"parent_record_offset":3248,"ancestor_record_offset":2488,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":3292,"parent_record_offset":3248,"ancestor_record_offset":2488,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":3084,"parent_record_offset":3048,"ancestor_record_offset":2488,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":3048,"parent_record_offset":2832,"ancestor_record_offset":2488,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::forget_remaining_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1471,"record_offset":3540,"parent_record_offset":3204,"ancestor_record_offset":2488,"inlinee":6152,"source_line":162,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1471,"record_offset":3204,"parent_record_offset":3048,"ancestor_record_offset":2488,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1471,"record_offset":3116,"parent_record_offset":3084,"ancestor_record_offset":2488,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1471,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2488,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1471,"record_offset":3172,"parent_record_offset":3084,"ancestor_record_offset":2488,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1471,"record_offset":3012,"parent_record_offset":2960,"ancestor_record_offset":2488,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":1471,"record_offset":2960,"parent_record_offset":2832,"ancestor_record_offset":2488,"inlinee":26093,"source_line":23,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1471,"record_offset":2904,"parent_record_offset":2832,"ancestor_record_offset":2488,"inlinee":26122,"source_line":175,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1471,"record_offset":1552,"rva":21078528,"va":5389787648}
// 编译实例 RVAs: 0x141a200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14140b2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e8bb0dffff","size":5,"soft_fail":false,"target_va":5389726432,"va":5389788448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26562053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e8054e9501","size":5,"soft_fail":false,"target_va":5416350384,"va":5389788326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27268788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e8b416a001","size":5,"soft_fail":false,"target_va":5417057056,"va":5389788263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26593218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e8c2c79501","size":5,"soft_fail":false,"target_va":5416381456,"va":5389788233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12712509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e8c3053eff","size":5,"soft_fail":false,"target_va":5377075712,"va":5389788216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-85944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389787648,"raw_hex":"e848b0feff","size":5,"soft_fail":false,"target_va":5389702160,"va":5389788099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1471,"record_offset":2240,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":79526,"source_line":113,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1471,"record_offset":1972,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":79525,"source_line":131,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":2112,"parent_record_offset":2080,"ancestor_record_offset":1552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":1552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":1552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":2080,"parent_record_offset":2040,"ancestor_record_offset":1552,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":2012,"parent_record_offset":1972,"ancestor_record_offset":1552,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1552,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1471,"record_offset":2416,"parent_record_offset":2372,"ancestor_record_offset":1552,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1471,"record_offset":2200,"parent_record_offset":2040,"ancestor_record_offset":1552,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1471,"record_offset":2288,"parent_record_offset":2240,"ancestor_record_offset":1552,"inlinee":26122,"source_line":175,"archive":"_pdb/C/modules/01471.jsonl.gz"}
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
// 完整结构及原始类型记录: threshold_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: threshold_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: threshold_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range
// 信息 ID: b6bcee06b3181e1466095e67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=399, inline_record=13124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 信息 ID: 7dfa96ad05b52982472775fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>", "system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x1474153 -> 0x1419eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>
// 信息 ID: e3fc0465b2a2a8f3d1565023
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1", "system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f1bf6 -> 0x141a550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 信息 ID: bde4cc0919be59178504c510
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7", "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs @ 0x7fdeb7 -> 0x141a200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
