// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/history/replay_plant_paint.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 197faabc9e88a9d65b1607de201cff32892f08b80b00a277e07f8010d8ad1fc8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_plant_paint.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 536, 1665, 1674, 1682
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::replay_plant_paint
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1665,"record_offset":17328,"rva":24919856,"va":5393628976}
// 编译实例 RVAs: 0x17c3f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t22989657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e859cb5e01","size":5,"soft_fail":false,"target_va":5416621000,"va":5393631338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20500090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e88631c7fe","size":5,"soft_fail":false,"target_va":5373130976,"va":5393631061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17341234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8ce64f7fe","size":5,"soft_fail":false,"target_va":5376289952,"va":5393631181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14831500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e874b01dff","size":5,"soft_fail":false,"target_va":5378799536,"va":5393631031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12052408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8481848ff","size":5,"soft_fail":false,"target_va":5381577744,"va":5393630147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11875736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e868ca4aff","size":5,"soft_fail":false,"target_va":5381754944,"va":5393630675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11747108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8dcc04cff","size":5,"soft_fail":false,"target_va":5381884032,"va":5393631135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11743625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e877ce4cff","size":5,"soft_fail":false,"target_va":5381887584,"va":5393631204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11743411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e84dcf4cff","size":5,"soft_fail":false,"target_va":5381887680,"va":5393631086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c918c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11743035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8c5d04cff","size":5,"soft_fail":false,"target_va":5381888192,"va":5393631222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11741085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e863d84cff","size":5,"soft_fail":false,"target_va":5381890032,"va":5393631112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11740386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e81edb4cff","size":5,"soft_fail":false,"target_va":5381890640,"va":5393631021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22719646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e89eac5a01","size":5,"soft_fail":false,"target_va":5416350384,"va":5393630733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23427345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e811796501","size":5,"soft_fail":false,"target_va":5417057056,"va":5393629706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23426163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e873746501","size":5,"soft_fail":false,"target_va":5417057072,"va":5393630904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23427687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8677a6501","size":5,"soft_fail":false,"target_va":5417057088,"va":5393629396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23427770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8ba7a6501","size":5,"soft_fail":false,"target_va":5417057664,"va":5393629889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22990275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8c3cd5e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5393631320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22990403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e843ce5e01","size":5,"soft_fail":false,"target_va":5416621762,"va":5393631354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22991469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e86dd25e01","size":5,"soft_fail":false,"target_va":5416622768,"va":5393631294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22991007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e89fd05e01","size":5,"soft_fail":false,"target_va":5416622320,"va":5393631308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::accumulate_stroke
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13630093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8730530ff","size":5,"soft_fail":false,"target_va":5379999584,"va":5393629672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::begin_stroke
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13632015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8f1fd2fff","size":5,"soft_fail":false,"target_va":5379997248,"va":5393629258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13632602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8a6fb2fff","size":5,"soft_fail":false,"target_va":5379997744,"va":5393630341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10332743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393628976,"raw_hex":"e8b95562ff","size":5,"soft_fail":false,"target_va":5383297104,"va":5393629842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5393628976,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5393630181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1665,"record_offset":19360,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":7951,"source_line":2143,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21708,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":19840,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1665,"record_offset":17560,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1665,"record_offset":21344,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1665,"record_offset":19664,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1665,"record_offset":17992,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1665,"record_offset":21188,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1665,"record_offset":21316,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":22024,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":21160,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1665,"record_offset":19212,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1665,"record_offset":19272,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1665,"record_offset":19184,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1665,"record_offset":17624,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":18356,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":32987,"source_line":88,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::commit_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":20820,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":29019,"source_line":137,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging_ssbo
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":20292,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":29020,"source_line":307,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_plant_paint::apply_full_edit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_plant_paint.rs
//     evidence={"module":1665,"record_offset":19472,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":82376,"source_line":52,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_plant_paint::stroke_prefix_len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_plant_paint.rs
//     evidence={"module":1665,"record_offset":19696,"parent_record_offset":17328,"ancestor_record_offset":17328,"inlinee":82377,"source_line":61,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17760,"parent_record_offset":17732,"ancestor_record_offset":17328,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17648,"parent_record_offset":17624,"ancestor_record_offset":17328,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17672,"parent_record_offset":17648,"ancestor_record_offset":17328,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17860,"parent_record_offset":17672,"ancestor_record_offset":17328,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17704,"parent_record_offset":17672,"ancestor_record_offset":17328,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1665,"record_offset":17732,"parent_record_offset":17704,"ancestor_record_offset":17328,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1665,"record_offset":17792,"parent_record_offset":17760,"ancestor_record_offset":17328,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1665,"record_offset":17892,"parent_record_offset":17860,"ancestor_record_offset":17328,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1665,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":17328,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1665,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":17328,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1665,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":17328,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1665,"record_offset":22136,"parent_record_offset":22100,"ancestor_record_offset":17328,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1665,"record_offset":22100,"parent_record_offset":22072,"ancestor_record_offset":17328,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21540,"parent_record_offset":21500,"ancestor_record_offset":17328,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21788,"parent_record_offset":21744,"ancestor_record_offset":17328,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21468,"parent_record_offset":21432,"ancestor_record_offset":17328,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21500,"parent_record_offset":21468,"ancestor_record_offset":17328,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21988,"parent_record_offset":21744,"ancestor_record_offset":17328,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21744,"parent_record_offset":21708,"ancestor_record_offset":17328,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1665,"record_offset":17588,"parent_record_offset":17560,"ancestor_record_offset":17328,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1665,"record_offset":21368,"parent_record_offset":21344,"ancestor_record_offset":17328,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1665,"record_offset":21432,"parent_record_offset":21368,"ancestor_record_offset":17328,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1665,"record_offset":19780,"parent_record_offset":19696,"ancestor_record_offset":17328,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1665,"record_offset":19808,"parent_record_offset":19696,"ancestor_record_offset":17328,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1665,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":17328,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1665,"record_offset":18424,"parent_record_offset":18400,"ancestor_record_offset":17328,"inlinee":7947,"source_line":99,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1665,"record_offset":18016,"parent_record_offset":17992,"ancestor_record_offset":17328,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1665,"record_offset":18124,"parent_record_offset":18040,"ancestor_record_offset":17328,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1665,"record_offset":18400,"parent_record_offset":18356,"ancestor_record_offset":17328,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1665,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":17328,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1665,"record_offset":18152,"parent_record_offset":18124,"ancestor_record_offset":17328,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1665,"record_offset":18040,"parent_record_offset":18016,"ancestor_record_offset":17328,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1665,"record_offset":18536,"parent_record_offset":18452,"ancestor_record_offset":17328,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1665,"record_offset":20260,"parent_record_offset":20088,"ancestor_record_offset":17328,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":19384,"parent_record_offset":19360,"ancestor_record_offset":17328,"inlinee":7952,"source_line":2443,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":21960,"parent_record_offset":21744,"ancestor_record_offset":17328,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":19620,"parent_record_offset":19588,"ancestor_record_offset":17328,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":22048,"parent_record_offset":22024,"ancestor_record_offset":17328,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1665,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":17328,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1665,"record_offset":22168,"parent_record_offset":22136,"ancestor_record_offset":17328,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1665,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":17328,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":21928,"parent_record_offset":21744,"ancestor_record_offset":17328,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":21660,"parent_record_offset":21500,"ancestor_record_offset":17328,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":19588,"parent_record_offset":19560,"ancestor_record_offset":17328,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":17328,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":18864,"parent_record_offset":18356,"ancestor_record_offset":17328,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":19088,"parent_record_offset":19004,"ancestor_record_offset":17328,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":19868,"parent_record_offset":19840,"ancestor_record_offset":17328,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":21276,"parent_record_offset":21248,"ancestor_record_offset":17328,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":19924,"parent_record_offset":19896,"ancestor_record_offset":17328,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":19896,"parent_record_offset":19868,"ancestor_record_offset":17328,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1665,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":17328,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1665,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":17328,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1665,"record_offset":18480,"parent_record_offset":18452,"ancestor_record_offset":17328,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1665,"record_offset":18068,"parent_record_offset":18040,"ancestor_record_offset":17328,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1665,"record_offset":18888,"parent_record_offset":18864,"ancestor_record_offset":17328,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1665,"record_offset":22196,"parent_record_offset":22136,"ancestor_record_offset":17328,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1665,"record_offset":22228,"parent_record_offset":22100,"ancestor_record_offset":17328,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1665,"record_offset":20188,"parent_record_offset":20088,"ancestor_record_offset":17328,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1665,"record_offset":20220,"parent_record_offset":20188,"ancestor_record_offset":17328,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1665,"record_offset":20416,"parent_record_offset":20292,"ancestor_record_offset":17328,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::plant_paint_edit::PlantPaintEdit::brush
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/plant_paint_edit.rs
//     evidence={"module":1665,"record_offset":19504,"parent_record_offset":19472,"ancestor_record_offset":17328,"inlinee":37879,"source_line":22,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::plant_sample_raster::PlantSampleRaster::commit_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":19532,"parent_record_offset":19472,"ancestor_record_offset":17328,"inlinee":29019,"source_line":137,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::plant_sample_raster::PlantSampleRaster::mark_all_dirty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":18832,"parent_record_offset":18356,"ancestor_record_offset":17328,"inlinee":32990,"source_line":327,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::plant_sample_raster::impl$0::clear::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":17328,"inlinee":32989,"source_line":93,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_plant_paint::replay_plant_paint::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_plant_paint.rs
//     evidence={"module":1665,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":17328,"inlinee":82374,"source_line":90,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1665,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":17328,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::PlantPaintReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141E0
// 首条位置: {"module":1674,"inlinee":82400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":94640,"parent_record_offset":94460,"ancestor_record_offset":94460,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1674,"record_offset":94668,"parent_record_offset":94460,"ancestor_record_offset":94460,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_plant_paint::polyline_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_plant_paint.rs
//     evidence={"module":1674,"record_offset":94500,"parent_record_offset":94460,"ancestor_record_offset":94460,"inlinee":82378,"source_line":46,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1674,"record_offset":94692,"parent_record_offset":94668,"ancestor_record_offset":94460,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1674,"record_offset":94548,"parent_record_offset":94524,"ancestor_record_offset":94460,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1674,"record_offset":94604,"parent_record_offset":94580,"ancestor_record_offset":94460,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1674,"record_offset":94524,"parent_record_offset":94500,"ancestor_record_offset":94460,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1674,"record_offset":94580,"parent_record_offset":94500,"ancestor_record_offset":94460,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":94728,"parent_record_offset":94692,"ancestor_record_offset":94460,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":94752,"parent_record_offset":94728,"ancestor_record_offset":94460,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":94784,"parent_record_offset":94692,"ancestor_record_offset":94460,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::polyline_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141CA
// 首条位置: {"module":1665,"inlinee":82378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1674,"record_offset":94524,"parent_record_offset":94500,"ancestor_record_offset":94500,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1674,"record_offset":94580,"parent_record_offset":94500,"ancestor_record_offset":94500,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1674,"record_offset":94548,"parent_record_offset":94524,"ancestor_record_offset":94500,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1674,"record_offset":94604,"parent_record_offset":94580,"ancestor_record_offset":94500,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::polyline_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FF1
// 首条位置: {"module":536,"inlinee":32753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1682,"record_offset":35296,"parent_record_offset":35268,"ancestor_record_offset":35268,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":35428,"parent_record_offset":35404,"ancestor_record_offset":35268,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":35348,"parent_record_offset":35320,"ancestor_record_offset":35268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1682,"record_offset":35372,"parent_record_offset":35320,"ancestor_record_offset":35268,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1682,"record_offset":35320,"parent_record_offset":35296,"ancestor_record_offset":35268,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1682,"record_offset":35404,"parent_record_offset":35372,"ancestor_record_offset":35268,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::apply_full_edit
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141C8
// 首条位置: {"module":1665,"inlinee":82376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::history::plant_paint_edit::PlantPaintEdit::brush
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/plant_paint_edit.rs
//     evidence={"module":1665,"record_offset":19504,"parent_record_offset":19472,"ancestor_record_offset":19472,"inlinee":37879,"source_line":22,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::commit_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":1665,"record_offset":19532,"parent_record_offset":19472,"ancestor_record_offset":19472,"inlinee":29019,"source_line":137,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":19620,"parent_record_offset":19588,"ancestor_record_offset":19472,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":19588,"parent_record_offset":19560,"ancestor_record_offset":19472,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1665,"record_offset":19560,"parent_record_offset":19532,"ancestor_record_offset":19472,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::stroke_prefix_len
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141C9
// 首条位置: {"module":1665,"inlinee":82377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1665,"record_offset":19780,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1665,"record_offset":19808,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1665,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1665,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":19696,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_plant_paint::replay_plant_paint::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141C6
// 首条位置: {"module":1665,"inlinee":82374}
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
// 完整结构及原始类型记录: replay_plant_paint.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_plant_paint::PlantPaintReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_plant_paint.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_plant_paint.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 31; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::PlantPaintReplayCmd::new
// 信息 ID: bbaab272e2d29f557fb49597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=94460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::apply_full_edit
// 信息 ID: ab40aa763231ddb633f0114a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_plant_paint::replay_plant_paint; module=1665, inline_record=19472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::polyline_length
// 信息 ID: ce74dcff6016c7bab6704056
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=94500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::polyline_length::closure$0
// 信息 ID: 476ca5a05905eb6b67c5e0d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_plant_paint::polyline_length", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Windows<glam::f32::vec2::Vec2>,country_core::systems::history::replay_plant_paint::polyline_length::closure_env$0> > >; module=1682, inline_record=35268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::replay_plant_paint
// 信息 ID: a0d56a875687c22d3270a152
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a8b1f70e3f58bc2b719cd548: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)> > @ 0x19558a4 -> 0x17c3f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)> @ 0x19de8f5 -> 0x17c3f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::replay_plant_paint::closure$0
// 信息 ID: 1fe3b74b7cff7a9e8720b9eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_plant_paint::replay_plant_paint", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_plant_paint::replay_plant_paint; module=1665, inline_record=18176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_plant_paint::stroke_prefix_len
// 信息 ID: 890eba6d27dc166000cf59bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_plant_paint::replay_plant_paint; module=1665, inline_record=19696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
