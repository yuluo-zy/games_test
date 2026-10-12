// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/write_roof_color_id_to_gpu.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 43f7e87b5455811db44fe593b64570692a8ef8e2192455a07653e64bca811a4b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: write_roof_color_id_to_gpu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3758
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3758,"record_offset":1384,"rva":36275488,"va":5404984608}
// 编译实例 RVAs: 0x2298520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35717615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e811fedefd","size":5,"soft_fail":false,"target_va":5369267664,"va":5404985274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28209477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e8bb8e51fe","size":5,"soft_fail":false,"target_va":5376776208,"va":5404985680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26282771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e8edf46efe","size":5,"soft_fail":false,"target_va":5378702496,"va":5404985262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23763074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e87e6795fe","size":5,"soft_fail":false,"target_va":5381222304,"va":5404985373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425f18b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3511122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e852933500","size":5,"soft_fail":false,"target_va":5408495792,"va":5404984665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: write_roof_color_id_to_gpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6862418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e852b66800","size":5,"soft_fail":false,"target_va":5411847152,"va":5404984729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6556144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e8f0096400","size":5,"soft_fail":false,"target_va":5411541776,"va":5404985627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6496946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e8b2226300","size":5,"soft_fail":false,"target_va":5411482433,"va":5404985482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6557262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404984608,"raw_hex":"e84e0e6400","size":5,"soft_fail":false,"target_va":5411542880,"va":5404985613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5404984608,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5404984749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3758,"record_offset":1828,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3758,"record_offset":3248,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3758,"record_offset":1792,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3758,"record_offset":4384,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3758,"record_offset":1556,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write_element
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3758,"record_offset":4244,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":121483,"source_line":199,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3758,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":1384,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3758,"record_offset":2140,"parent_record_offset":1916,"ancestor_record_offset":1384,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3758,"record_offset":1892,"parent_record_offset":1852,"ancestor_record_offset":1384,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3758,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1384,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3758,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":1384,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3758,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":1384,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3758,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1384,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3758,"record_offset":1624,"parent_record_offset":1584,"ancestor_record_offset":1384,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3758,"record_offset":3280,"parent_record_offset":3248,"ancestor_record_offset":1384,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3758,"record_offset":3472,"parent_record_offset":3440,"ancestor_record_offset":1384,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3758,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":1384,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3758,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":1384,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3758,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1384,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3758,"record_offset":1956,"parent_record_offset":1916,"ancestor_record_offset":1384,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3758,"record_offset":3344,"parent_record_offset":3280,"ancestor_record_offset":1384,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3758,"record_offset":3312,"parent_record_offset":3280,"ancestor_record_offset":1384,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3758,"record_offset":3560,"parent_record_offset":3472,"ancestor_record_offset":1384,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3758,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1384,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3758,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":1384,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3758,"record_offset":4336,"parent_record_offset":4300,"ancestor_record_offset":1384,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3758,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1384,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3758,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":1384,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3758,"record_offset":2012,"parent_record_offset":1916,"ancestor_record_offset":1384,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3758,"record_offset":2380,"parent_record_offset":1916,"ancestor_record_offset":1384,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3758,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1384,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3758,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1384,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3758,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":1384,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3758,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":1384,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3758,"record_offset":3648,"parent_record_offset":3588,"ancestor_record_offset":1384,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3758,"record_offset":3676,"parent_record_offset":3648,"ancestor_record_offset":1384,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3758,"record_offset":1652,"parent_record_offset":1624,"ancestor_record_offset":1384,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3758,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":1384,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3758,"record_offset":4272,"parent_record_offset":4244,"ancestor_record_offset":1384,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/03758.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3758,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":1384,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/03758.jsonl.gz"}
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
// 完整结构及原始类型记录: write_roof_color_id_to_gpu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: write_roof_color_id_to_gpu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: write_roof_color_id_to_gpu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>

// ------------------------------------------------------------------
// 函数: system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu
// 信息 ID: 4b3ef0d5cded78f613d37fd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c6ca0e10baa94544ef54ee08: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)> > > @ 0x1c8dbae -> 0x2298520 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>)> > @ 0x1cad0a3 -> 0x2298520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
