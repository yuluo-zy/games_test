// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/nani/mesh_atlas.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ce02aac3973656abb8615c1039eecb33b6ba3643f2e01d53afc4189431396f62
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh_atlas.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 517, 539, 544, 588, 1643, 1682, 1770
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8183
// 首条位置: {"module":539,"inlinee":33155}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":588,"record_offset":115692,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":588,"record_offset":115940,"parent_record_offset":115912,"ancestor_record_offset":115664,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":588,"record_offset":115992,"parent_record_offset":115968,"ancestor_record_offset":115664,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":588,"record_offset":115968,"parent_record_offset":115912,"ancestor_record_offset":115664,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":588,"record_offset":116016,"parent_record_offset":115992,"ancestor_record_offset":115664,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":588,"record_offset":115880,"parent_record_offset":115844,"ancestor_record_offset":115664,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":588,"record_offset":115728,"parent_record_offset":115692,"ancestor_record_offset":115664,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":588,"record_offset":115784,"parent_record_offset":115756,"ancestor_record_offset":115664,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":588,"record_offset":115912,"parent_record_offset":115844,"ancestor_record_offset":115664,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":588,"record_offset":115756,"parent_record_offset":115728,"ancestor_record_offset":115664,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":588,"record_offset":115844,"parent_record_offset":115812,"ancestor_record_offset":115664,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":588,"record_offset":115812,"parent_record_offset":115784,"ancestor_record_offset":115664,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$7::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8199
// 首条位置: {"module":539,"inlinee":33177}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$11::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7659
// 首条位置: {"module":517,"inlinee":30297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":18448,"rva":9417824,"va":5378126944}
// 编译实例 RVAs: 0x8fb460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":18584,"rva":10898720,"va":5379607840}
// 编译实例 RVAs: 0xa64d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3930815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e84105c4ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379609498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-3931434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8d602c4ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379610133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10887298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e87edf59ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379609485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d65d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8975568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8300b77ff","size":5,"soft_fail":false,"target_va":5370635728,"va":5379611291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6127106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8fe81a2ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5379607997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6129205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8cb79a2ff","size":5,"soft_fail":false,"target_va":5373481264,"va":5379610464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140977fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-974219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e87522f1ff","size":5,"soft_fail":false,"target_va":5378637744,"va":5379611958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-874307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8bda8f2ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5379610718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9fdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2337399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e877aa2300","size":5,"soft_fail":false,"target_va":5381946864,"va":5379609460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fe830
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12161905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e87193b900","size":5,"soft_fail":false,"target_va":5391771696,"va":5379609786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ac8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13922075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e81b6fd400","size":5,"soft_fail":false,"target_va":5393533152,"va":5379611072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36739826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8f29a3002","size":5,"soft_fail":false,"target_va":5416350832,"va":5379611001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36741805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8ada23002","size":5,"soft_fail":false,"target_va":5416353040,"va":5379611230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37151524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e824e33602","size":5,"soft_fail":false,"target_va":5416761616,"va":5379610087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37447073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8a1653b02","size":5,"soft_fail":false,"target_va":5417057040,"va":5379609962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37447527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e867673b02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379609524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5184798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e81e1d4f00","size":5,"soft_fail":false,"target_va":5384794864,"va":5379610061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37011192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8f8be3402","size":5,"soft_fail":false,"target_va":5416620794,"va":5379609597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37012967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8e7c53402","size":5,"soft_fail":false,"target_va":5416621536,"va":5379608564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37010842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e89abd3402","size":5,"soft_fail":false,"target_va":5416622208,"va":5379611361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37011031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e857be3402","size":5,"soft_fail":false,"target_va":5416622240,"va":5379611204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2261709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8cd822200","size":5,"soft_fail":false,"target_va":5381873152,"va":5379611438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4075393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8812f3e00","size":5,"soft_fail":false,"target_va":5383685776,"va":5379610378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4079295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379607840,"raw_hex":"e8bf3e3e00","size":5,"soft_fail":false,"target_va":5383687680,"va":5379608380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":539,"record_offset":27268,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":539,"record_offset":26320,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":21404,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":30140,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":22484,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":539,"record_offset":26664,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":13752,"source_line":17,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":539,"record_offset":27352,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":21076,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":25732,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":30384,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":21564,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":27380,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":29484,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":4359,"source_line":599,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":539,"record_offset":18868,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":31926,"source_line":58,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":539,"record_offset":18812,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":32682,"source_line":50,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$0::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":539,"record_offset":21692,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":30462,"source_line":22,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":539,"record_offset":23552,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":32361,"source_line":10,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":30356,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":25300,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":10241,"source_line":481,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":20016,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":26976,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":21914,"source_line":2037,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":29060,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":539,"record_offset":21636,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":24172,"parent_record_offset":24148,"ancestor_record_offset":18584,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":24196,"parent_record_offset":24172,"ancestor_record_offset":18584,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":25192,"parent_record_offset":25168,"ancestor_record_offset":18584,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":25216,"parent_record_offset":25192,"ancestor_record_offset":18584,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":24220,"parent_record_offset":24196,"ancestor_record_offset":18584,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":25240,"parent_record_offset":25216,"ancestor_record_offset":18584,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":24148,"parent_record_offset":24004,"ancestor_record_offset":18584,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":25168,"parent_record_offset":25144,"ancestor_record_offset":18584,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":539,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":18584,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":21472,"parent_record_offset":21440,"ancestor_record_offset":18584,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":22636,"parent_record_offset":22608,"ancestor_record_offset":18584,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":23944,"parent_record_offset":23916,"ancestor_record_offset":18584,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":22720,"parent_record_offset":22696,"ancestor_record_offset":18584,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":27480,"parent_record_offset":27452,"ancestor_record_offset":18584,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":25144,"parent_record_offset":25120,"ancestor_record_offset":18584,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":22696,"parent_record_offset":22664,"ancestor_record_offset":18584,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":21520,"parent_record_offset":21496,"ancestor_record_offset":18584,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":18584,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":22664,"parent_record_offset":22636,"ancestor_record_offset":18584,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":24004,"parent_record_offset":23972,"ancestor_record_offset":18584,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":23972,"parent_record_offset":23944,"ancestor_record_offset":18584,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":18584,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":24036,"parent_record_offset":24004,"ancestor_record_offset":18584,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":539,"record_offset":23880,"parent_record_offset":23704,"ancestor_record_offset":18584,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":539,"record_offset":26352,"parent_record_offset":26320,"ancestor_record_offset":18584,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":539,"record_offset":26828,"parent_record_offset":26800,"ancestor_record_offset":18584,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":22772,"parent_record_offset":22556,"ancestor_record_offset":18584,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":21440,"parent_record_offset":21404,"ancestor_record_offset":18584,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23740,"parent_record_offset":23704,"ancestor_record_offset":18584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":22556,"parent_record_offset":22520,"ancestor_record_offset":18584,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":18836,"parent_record_offset":18812,"ancestor_record_offset":18584,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":18584,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":22608,"parent_record_offset":22556,"ancestor_record_offset":18584,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23916,"parent_record_offset":23880,"ancestor_record_offset":18584,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23676,"parent_record_offset":23552,"ancestor_record_offset":18584,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":25760,"parent_record_offset":25732,"ancestor_record_offset":18584,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23172,"parent_record_offset":23128,"ancestor_record_offset":18584,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23704,"parent_record_offset":23676,"ancestor_record_offset":18584,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":539,"record_offset":26856,"parent_record_offset":26828,"ancestor_record_offset":18584,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":539,"record_offset":23272,"parent_record_offset":23172,"ancestor_record_offset":18584,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":539,"record_offset":23512,"parent_record_offset":22556,"ancestor_record_offset":18584,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":539,"record_offset":23452,"parent_record_offset":23424,"ancestor_record_offset":18584,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":539,"record_offset":22520,"parent_record_offset":22484,"ancestor_record_offset":18584,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":539,"record_offset":24460,"parent_record_offset":24432,"ancestor_record_offset":18584,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":24060,"parent_record_offset":24036,"ancestor_record_offset":18584,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":26688,"parent_record_offset":26664,"ancestor_record_offset":18584,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":19672,"parent_record_offset":19644,"ancestor_record_offset":18584,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":26776,"parent_record_offset":26740,"ancestor_record_offset":18584,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":539,"record_offset":22032,"parent_record_offset":22008,"ancestor_record_offset":18584,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":539,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":18584,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":539,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":18584,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":21356,"parent_record_offset":21328,"ancestor_record_offset":18584,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":21660,"parent_record_offset":21636,"ancestor_record_offset":18584,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":19840,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":24816,"parent_record_offset":24792,"ancestor_record_offset":18584,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":22936,"parent_record_offset":22900,"ancestor_record_offset":18584,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":21188,"parent_record_offset":21104,"ancestor_record_offset":18584,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":539,"record_offset":25788,"parent_record_offset":25760,"ancestor_record_offset":18584,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":22900,"parent_record_offset":22556,"ancestor_record_offset":18584,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":23128,"parent_record_offset":23048,"ancestor_record_offset":18584,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":18584,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":24108,"parent_record_offset":24084,"ancestor_record_offset":18584,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":23312,"parent_record_offset":22972,"ancestor_record_offset":18584,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":27512,"parent_record_offset":27480,"ancestor_record_offset":18584,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":539,"record_offset":19464,"parent_record_offset":19436,"ancestor_record_offset":18584,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":539,"record_offset":19436,"parent_record_offset":19408,"ancestor_record_offset":18584,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":30408,"parent_record_offset":30384,"ancestor_record_offset":18584,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":22452,"parent_record_offset":21692,"ancestor_record_offset":18584,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":27292,"parent_record_offset":27268,"ancestor_record_offset":18584,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":24344,"parent_record_offset":23552,"ancestor_record_offset":18584,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":25536,"parent_record_offset":25504,"ancestor_record_offset":18584,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":24276,"parent_record_offset":23880,"ancestor_record_offset":18584,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":18584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":23340,"parent_record_offset":22972,"ancestor_record_offset":18584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":539,"record_offset":19580,"parent_record_offset":19556,"ancestor_record_offset":18584,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":21160,"parent_record_offset":21104,"ancestor_record_offset":18584,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":23224,"parent_record_offset":23172,"ancestor_record_offset":18584,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":539,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":18584,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":539,"record_offset":26740,"parent_record_offset":26712,"ancestor_record_offset":18584,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":539,"record_offset":19784,"parent_record_offset":19756,"ancestor_record_offset":18584,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":29516,"parent_record_offset":29484,"ancestor_record_offset":18584,"inlinee":4360,"source_line":228,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":21104,"parent_record_offset":21076,"ancestor_record_offset":18584,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":539,"record_offset":26384,"parent_record_offset":26352,"ancestor_record_offset":18584,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":539,"record_offset":19700,"parent_record_offset":19672,"ancestor_record_offset":18584,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":539,"record_offset":21216,"parent_record_offset":21188,"ancestor_record_offset":18584,"inlinee":33150,"source_line":87,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":539,"record_offset":21328,"parent_record_offset":21188,"ancestor_record_offset":18584,"inlinee":33151,"source_line":88,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":539,"record_offset":21600,"parent_record_offset":21564,"ancestor_record_offset":18584,"inlinee":33152,"source_line":94,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":539,"record_offset":23080,"parent_record_offset":23048,"ancestor_record_offset":18584,"inlinee":33153,"source_line":109,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":539,"record_offset":21280,"parent_record_offset":21244,"ancestor_record_offset":18584,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":539,"record_offset":21244,"parent_record_offset":21216,"ancestor_record_offset":18584,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":19376,"parent_record_offset":19348,"ancestor_record_offset":18584,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":19408,"parent_record_offset":19376,"ancestor_record_offset":18584,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":19348,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":19208,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":22008,"parent_record_offset":21844,"ancestor_record_offset":18584,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":19868,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":25620,"parent_record_offset":25564,"ancestor_record_offset":18584,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":19292,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":539,"record_offset":19152,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":539,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":18584,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":18924,"parent_record_offset":18896,"ancestor_record_offset":18584,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":20084,"parent_record_offset":20048,"ancestor_record_offset":18584,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":22244,"parent_record_offset":22220,"ancestor_record_offset":18584,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":27024,"parent_record_offset":27000,"ancestor_record_offset":18584,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":19616,"parent_record_offset":19508,"ancestor_record_offset":18584,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":18584,"inlinee":13277,"source_line":191,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":21784,"parent_record_offset":21756,"ancestor_record_offset":18584,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":29088,"parent_record_offset":29060,"ancestor_record_offset":18584,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":19020,"parent_record_offset":18924,"ancestor_record_offset":18584,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":29392,"parent_record_offset":29088,"ancestor_record_offset":18584,"inlinee":26818,"source_line":594,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":29416,"parent_record_offset":29392,"ancestor_record_offset":18584,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19556,"parent_record_offset":19532,"ancestor_record_offset":18584,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":22064,"parent_record_offset":21844,"ancestor_record_offset":18584,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19960,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":22348,"parent_record_offset":22292,"ancestor_record_offset":18584,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":21844,"parent_record_offset":21812,"ancestor_record_offset":18584,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":18584,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":18584,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19048,"parent_record_offset":18924,"ancestor_record_offset":18584,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":18964,"parent_record_offset":18924,"ancestor_record_offset":18584,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":22268,"parent_record_offset":22244,"ancestor_record_offset":18584,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":18584,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":25476,"parent_record_offset":25412,"ancestor_record_offset":18584,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":22320,"parent_record_offset":22292,"ancestor_record_offset":18584,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":18584,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":18584,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19180,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":21812,"parent_record_offset":21784,"ancestor_record_offset":18584,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":19508,"parent_record_offset":19096,"ancestor_record_offset":18584,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":18584,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":20048,"parent_record_offset":20016,"ancestor_record_offset":18584,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":18584,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":27000,"parent_record_offset":26976,"ancestor_record_offset":18584,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":22196,"parent_record_offset":21692,"ancestor_record_offset":18584,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":21756,"parent_record_offset":21728,"ancestor_record_offset":18584,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":21728,"parent_record_offset":21692,"ancestor_record_offset":18584,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":23616,"parent_record_offset":23552,"ancestor_record_offset":18584,"inlinee":13788,"source_line":1397,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":24404,"parent_record_offset":24376,"ancestor_record_offset":18584,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":539,"record_offset":24376,"parent_record_offset":24344,"ancestor_record_offset":18584,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":539,"record_offset":24432,"parent_record_offset":24404,"ancestor_record_offset":18584,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x817E
// 首条位置: {"module":539,"inlinee":33150}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":539,"record_offset":21244,"parent_record_offset":21216,"ancestor_record_offset":21216,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":539,"record_offset":21280,"parent_record_offset":21244,"ancestor_record_offset":21216,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x817F
// 首条位置: {"module":539,"inlinee":33151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":539,"record_offset":21356,"parent_record_offset":21328,"ancestor_record_offset":21328,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8180
// 首条位置: {"module":539,"inlinee":33152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8181
// 首条位置: {"module":539,"inlinee":33153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::remove
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81CD
// 首条位置: {"module":544,"inlinee":33229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":75480,"parent_record_offset":75436,"ancestor_record_offset":75436,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":75788,"parent_record_offset":75436,"ancestor_record_offset":75436,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":75704,"parent_record_offset":75436,"ancestor_record_offset":75436,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":75580,"parent_record_offset":75556,"ancestor_record_offset":75436,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":75628,"parent_record_offset":75604,"ancestor_record_offset":75436,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":75604,"parent_record_offset":75580,"ancestor_record_offset":75436,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":75556,"parent_record_offset":75512,"ancestor_record_offset":75436,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":75512,"parent_record_offset":75480,"ancestor_record_offset":75436,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":75668,"parent_record_offset":75512,"ancestor_record_offset":75436,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":75820,"parent_record_offset":75788,"ancestor_record_offset":75436,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":75852,"parent_record_offset":75820,"ancestor_record_offset":75436,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":75728,"parent_record_offset":75704,"ancestor_record_offset":75436,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":75752,"parent_record_offset":75728,"ancestor_record_offset":75436,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::compact
// 记录形式: compiled_procedure; instances=1
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":31032,"rva":10903232,"va":5379612352}
// 编译实例 RVAs: 0xa65ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3934237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8e3f7c3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379612920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3933787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8a5f9c3ff","size":5,"soft_fail":false,"target_va":5375678736,"va":5379612518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-3934734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8f2f5c3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379613433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10890314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8b6d359ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379612501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097c510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-958693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e81b5ff1ff","size":5,"soft_fail":false,"target_va":5378655504,"va":5379614192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7e460
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t98340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e824800100","size":5,"soft_fail":false,"target_va":5379712096,"va":5379613751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141613210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12242981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e825d0ba00","size":5,"soft_fail":false,"target_va":5391856144,"va":5379613158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141614340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12247314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e812e1ba00","size":5,"soft_fail":false,"target_va":5391860544,"va":5379613225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36911750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8863a3302","size":5,"soft_fail":false,"target_va":5416524832,"va":5379613077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37007910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e826b23402","size":5,"soft_fail":false,"target_va":5416620794,"va":5379612879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37008053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8b5b23402","size":5,"soft_fail":false,"target_va":5416621600,"va":5379613542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37008191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e83fb33402","size":5,"soft_fail":false,"target_va":5416621762,"va":5379613566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37009471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e83fb83402","size":5,"soft_fail":false,"target_va":5416622320,"va":5379612844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::retain_vertices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1850622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379612352,"raw_hex":"e8fe3c1c00","size":5,"soft_fail":false,"target_va":5381463872,"va":5379613245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":34028,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33584,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":32028,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33184,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33476,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":34116,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":539,"record_offset":36052,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":539,"record_offset":33156,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":35156,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":35092,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":539,"record_offset":35912,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":34524,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":36468,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":35744,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":34496,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":35212,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":26084,"source_line":3133,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":539,"record_offset":33412,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$0::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":539,"record_offset":31252,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":30462,"source_line":22,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":32304,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":32632,"parent_record_offset":31032,"ancestor_record_offset":31032,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32196,"parent_record_offset":32172,"ancestor_record_offset":31032,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":31032,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":36748,"parent_record_offset":36724,"ancestor_record_offset":31032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":36772,"parent_record_offset":36748,"ancestor_record_offset":31032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":33968,"parent_record_offset":33944,"ancestor_record_offset":31032,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":31032,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":36796,"parent_record_offset":36772,"ancestor_record_offset":31032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":33896,"parent_record_offset":33704,"ancestor_record_offset":31032,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":32172,"parent_record_offset":32144,"ancestor_record_offset":31032,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":36724,"parent_record_offset":36636,"ancestor_record_offset":31032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":34316,"parent_record_offset":34292,"ancestor_record_offset":31032,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":33640,"parent_record_offset":33612,"ancestor_record_offset":31032,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":32084,"parent_record_offset":32056,"ancestor_record_offset":31032,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":36664,"parent_record_offset":36636,"ancestor_record_offset":31032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":36636,"parent_record_offset":36612,"ancestor_record_offset":31032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":34364,"parent_record_offset":34340,"ancestor_record_offset":31032,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":34340,"parent_record_offset":34316,"ancestor_record_offset":31032,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":32144,"parent_record_offset":32112,"ancestor_record_offset":31032,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":33668,"parent_record_offset":33640,"ancestor_record_offset":31032,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":32112,"parent_record_offset":32084,"ancestor_record_offset":31032,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":36612,"parent_record_offset":36588,"ancestor_record_offset":31032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":33740,"parent_record_offset":33704,"ancestor_record_offset":31032,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":34292,"parent_record_offset":34256,"ancestor_record_offset":31032,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":31032,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":33612,"parent_record_offset":33584,"ancestor_record_offset":31032,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":35180,"parent_record_offset":35156,"ancestor_record_offset":31032,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":35936,"parent_record_offset":35912,"ancestor_record_offset":31032,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":539,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":31032,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":539,"record_offset":36564,"parent_record_offset":36540,"ancestor_record_offset":31032,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":539,"record_offset":36124,"parent_record_offset":36084,"ancestor_record_offset":31032,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":539,"record_offset":32056,"parent_record_offset":32028,"ancestor_record_offset":31032,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":539,"record_offset":33768,"parent_record_offset":33740,"ancestor_record_offset":31032,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":35584,"parent_record_offset":35560,"ancestor_record_offset":31032,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":539,"record_offset":31580,"parent_record_offset":31556,"ancestor_record_offset":31032,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":539,"record_offset":35004,"parent_record_offset":34980,"ancestor_record_offset":31032,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":35284,"parent_record_offset":35244,"ancestor_record_offset":31032,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":539,"record_offset":33828,"parent_record_offset":33796,"ancestor_record_offset":31032,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":539,"record_offset":35120,"parent_record_offset":35092,"ancestor_record_offset":31032,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":539,"record_offset":35992,"parent_record_offset":35912,"ancestor_record_offset":31032,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":539,"record_offset":36016,"parent_record_offset":35992,"ancestor_record_offset":31032,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":539,"record_offset":36084,"parent_record_offset":36052,"ancestor_record_offset":31032,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":539,"record_offset":35876,"parent_record_offset":35852,"ancestor_record_offset":31032,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":33796,"parent_record_offset":33768,"ancestor_record_offset":31032,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":33368,"parent_record_offset":33336,"ancestor_record_offset":31032,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":33856,"parent_record_offset":33796,"ancestor_record_offset":31032,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":539,"record_offset":36692,"parent_record_offset":36664,"ancestor_record_offset":31032,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":539,"record_offset":32896,"parent_record_offset":32872,"ancestor_record_offset":31032,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":539,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":31032,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":539,"record_offset":35560,"parent_record_offset":35436,"ancestor_record_offset":31032,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":539,"record_offset":31996,"parent_record_offset":31252,"ancestor_record_offset":31032,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":539,"record_offset":32580,"parent_record_offset":32496,"ancestor_record_offset":31032,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":35524,"parent_record_offset":35500,"ancestor_record_offset":31032,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":35476,"parent_record_offset":35436,"ancestor_record_offset":31032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":539,"record_offset":34188,"parent_record_offset":34164,"ancestor_record_offset":31032,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":539,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":31032,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":36264,"parent_record_offset":36196,"ancestor_record_offset":31032,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":36292,"parent_record_offset":36196,"ancestor_record_offset":31032,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":539,"record_offset":33440,"parent_record_offset":33412,"ancestor_record_offset":31032,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":36316,"parent_record_offset":36292,"ancestor_record_offset":31032,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":34080,"parent_record_offset":34052,"ancestor_record_offset":31032,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":539,"record_offset":34140,"parent_record_offset":34116,"ancestor_record_offset":31032,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":33212,"parent_record_offset":33184,"ancestor_record_offset":31032,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":33508,"parent_record_offset":33476,"ancestor_record_offset":31032,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":33240,"parent_record_offset":33212,"ancestor_record_offset":31032,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":33540,"parent_record_offset":33508,"ancestor_record_offset":31032,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":539,"record_offset":33336,"parent_record_offset":33308,"ancestor_record_offset":31032,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":539,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":31032,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":539,"record_offset":35500,"parent_record_offset":35476,"ancestor_record_offset":31032,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":539,"record_offset":35436,"parent_record_offset":35340,"ancestor_record_offset":31032,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":539,"record_offset":35340,"parent_record_offset":35244,"ancestor_record_offset":31032,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":539,"record_offset":35244,"parent_record_offset":35212,"ancestor_record_offset":31032,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":31464,"parent_record_offset":31436,"ancestor_record_offset":31032,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":31492,"parent_record_offset":31464,"ancestor_record_offset":31032,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":539,"record_offset":31436,"parent_record_offset":31400,"ancestor_record_offset":31032,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":31556,"parent_record_offset":31400,"ancestor_record_offset":31032,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":539,"record_offset":34980,"parent_record_offset":34924,"ancestor_record_offset":31032,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":31032,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":539,"record_offset":31344,"parent_record_offset":31316,"ancestor_record_offset":31032,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31612,"parent_record_offset":31400,"ancestor_record_offset":31032,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31892,"parent_record_offset":31840,"ancestor_record_offset":31032,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31400,"parent_record_offset":31372,"ancestor_record_offset":31032,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":31032,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":34836,"parent_record_offset":34772,"ancestor_record_offset":31032,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31032,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31840,"parent_record_offset":31816,"ancestor_record_offset":31032,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":31372,"parent_record_offset":31344,"ancestor_record_offset":31032,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":31768,"parent_record_offset":31744,"ancestor_record_offset":31032,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":31744,"parent_record_offset":31252,"ancestor_record_offset":31032,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":31316,"parent_record_offset":31288,"ancestor_record_offset":31032,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":539,"record_offset":31288,"parent_record_offset":31252,"ancestor_record_offset":31032,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::compact::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DE6
// 首条位置: {"module":1643,"inlinee":81382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":727228,"parent_record_offset":727200,"ancestor_record_offset":727200,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":727260,"parent_record_offset":727228,"ancestor_record_offset":727200,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1643,"record_offset":727292,"parent_record_offset":727260,"ancestor_record_offset":727200,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::remaining_slots
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81C9
// 首条位置: {"module":1770,"inlinee":33225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81CC
// 首条位置: {"module":588,"inlinee":33228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":112960,"parent_record_offset":112360,"ancestor_record_offset":112360,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":588,"record_offset":112396,"parent_record_offset":112360,"ancestor_record_offset":112360,"inlinee":33156,"source_line":254,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":588,"record_offset":112516,"parent_record_offset":112360,"ancestor_record_offset":112360,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":112484,"parent_record_offset":112396,"ancestor_record_offset":112360,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":112592,"parent_record_offset":112564,"ancestor_record_offset":112360,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":588,"record_offset":112836,"parent_record_offset":112812,"ancestor_record_offset":112360,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":588,"record_offset":112860,"parent_record_offset":112836,"ancestor_record_offset":112360,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":588,"record_offset":112812,"parent_record_offset":112752,"ancestor_record_offset":112360,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":588,"record_offset":112452,"parent_record_offset":112428,"ancestor_record_offset":112360,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112564,"parent_record_offset":112540,"ancestor_record_offset":112360,"inlinee":12661,"source_line":872,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112664,"parent_record_offset":112640,"ancestor_record_offset":112360,"inlinee":12651,"source_line":866,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112688,"parent_record_offset":112664,"ancestor_record_offset":112360,"inlinee":5023,"source_line":3207,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112752,"parent_record_offset":112688,"ancestor_record_offset":112360,"inlinee":5024,"source_line":2669,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112724,"parent_record_offset":112688,"ancestor_record_offset":112360,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112784,"parent_record_offset":112752,"ancestor_record_offset":112360,"inlinee":5025,"source_line":2691,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112900,"parent_record_offset":112688,"ancestor_record_offset":112360,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":588,"record_offset":112640,"parent_record_offset":112616,"ancestor_record_offset":112360,"inlinee":12650,"source_line":878,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\scopeguard.rs
//     evidence={"module":588,"record_offset":112616,"parent_record_offset":112592,"ancestor_record_offset":112360,"inlinee":12554,"source_line":69,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":588,"record_offset":112428,"parent_record_offset":112396,"ancestor_record_offset":112360,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00588.jsonl.gz"}
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
// 完整结构及原始类型记录: mesh_atlas.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlas
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::mesh_atlas::MeshAtlas>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlas"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlasPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "tuple$", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Values<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "std::collections::hash::map::Values", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Values<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Values<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<std::collections::hash::map::Values<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u32", "core::iter::adapters::map::Map<std::collections::hash::map::Values<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::buffer", "arguments": ["country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh_atlas.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh_atlas.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 30; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 信息 ID: 12b63dc332d693e7a200fbe1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0xc06717 -> 0xa64d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName> @ 0xc06c97 -> 0xa64d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 信息 ID: e7ce596622cf3f4ec108d38b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas; module=588, inline_record=112360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::compact
// 信息 ID: 9c9df87eb131e56e73d76697
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap @ 0xa6d7da -> 0xa65ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::resync_clutter @ 0x1adc470 -> 0xa65ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::resync_clutter @ 0x1adc3b9 -> 0xa65ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::resync_clutter @ 0x1adc303 -> 0xa65ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::remaining_slots
// 信息 ID: 7004072c94bb1dadf3934934
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=48808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::remove
// 信息 ID: 030ace6adfae17943428415d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::reload_nani_meshes_after_mod_swap; module=544, inline_record=75436; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name<ref$<asset::prefab::PrefabName> >; module=588, inline_record=97568; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::remove_by_name<ref$<str$> >; module=588, inline_record=99520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 信息 ID: 22f3859e425e272ce1a04041
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> > > > > @ 0x1ab2ae -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::try_insert_with_key<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh,slotmap::basic::impl$5::insert::closure_env$0<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>,enum2$<slotmap::util::Never> > @ 0x1dcefa -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::try_insert_with_key<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh,slotmap::basic::impl$5::insert::closure_env$0<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>,enum2$<slotmap::util::Never> > @ 0x1dcf29 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::try_insert_with_key<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh,slotmap::basic::impl$5::insert::closure_env$0<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>,enum2$<slotmap::util::Never> > @ 0x1dcf5a -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::try_insert_with_key<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh,slotmap::basic::impl$5::insert::closure_env$0<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>,enum2$<slotmap::util::Never> > @ 0x1dcfc8 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<true> > | core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$35<false> >,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > > @ 0x30a730 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x367410 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x367b66 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37a0a7 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37a7e5 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$6,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68b73a -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::maintain_wooden_stair_drawables @ 0x7f4bdd -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x80547e -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls::closure$12 @ 0x8087d4 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x80e9f1 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities @ 0x8141f7 -> 0x8fb460 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$0::default
// 信息 ID: a899177b7d5dd49955ee15e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >; module=588, inline_record=115664; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>; module=588, inline_record=120688; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,hashbrown::map::impl$82::extend::closure_env$0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0> > > >; module=1682, inline_record=102004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$11::fmt
// 信息 ID: 84e55c4a4ff6aa8a690f7f48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>; module=517, inline_record=123412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$0
// 信息 ID: 712fa4ce083c5e2ef103946b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=21216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$1
// 信息 ID: d879298af770f8b28f773003
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=21328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$2
// 信息 ID: 7bbbf5195561006ef4c7fd90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=21600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$3
// 信息 ID: ba181a662df4e20ed23c7ec2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=23080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::compact::closure$1
// 信息 ID: 77ce1da83b2b83187fd0812b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::compact", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::render::nani::mesh_atlas::impl$3::compact::closure_env$1> >; module=1643, inline_record=727200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$7::default
// 信息 ID: 8140038c7300aac499260efd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
