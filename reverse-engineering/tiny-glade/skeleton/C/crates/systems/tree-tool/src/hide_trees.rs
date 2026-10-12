// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/tree-tool/src/hide_trees.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2b5e848dbeea509fbc094cde2aa4682f1864803e28605dafea48a50d3eaf49a7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hide_trees.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3899, 4746
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_tree; // namespace candidate
//   package identities: [{"name":"system-tree","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3899,"record_offset":268,"rva":42340384,"va":5411049504}
// 编译实例 RVAs: 0x2861020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-35372442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e86642e4fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5411051125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-35374375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8d93ae4fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411053074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42328913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8af1c7afd","size":5,"soft_fail":false,"target_va":5368722192,"va":5411051100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31574918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e87a341efe","size":5,"soft_fail":false,"target_va":5379475248,"va":5411050161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7ede0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31340374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8aac821fe","size":5,"soft_fail":false,"target_va":5379714528,"va":5411054897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14181c960
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17059030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e82ab3fbfe","size":5,"soft_fail":false,"target_va":5393992032,"va":5411051057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142aff130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2743277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8eddb2900","size":5,"soft_fail":false,"target_va":5413794096,"va":5411050814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142aff540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2742801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e811da2900","size":5,"soft_fail":false,"target_va":5413795136,"va":5411052330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5298716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e81cda5000","size":5,"soft_fail":false,"target_va":5416350720,"va":5411051999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5300114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e892df5000","size":5,"soft_fail":false,"target_va":5416351056,"va":5411050937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6005892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e884a45b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5411051159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5565964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e80cee5400","size":5,"soft_fail":false,"target_va":5416620769,"va":5411054800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5348585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8e99c5100","size":5,"soft_fail":false,"target_va":5416400480,"va":5411051890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t5567773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e81df55400","size":5,"soft_fail":false,"target_va":5416620794,"va":5411053016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::query::Query::get_mut<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8520e0000","size":5,"soft_fail":false,"target_va":5411055792,"va":5411052121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30499116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8d49e2efe","size":5,"soft_fail":false,"target_va":5380552224,"va":5411051335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::placement_data::PlacementDataSsbo::position
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30526127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e851352efe","size":5,"soft_fail":false,"target_va":5380524592,"va":5411050714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27748490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8769758fe","size":5,"soft_fail":false,"target_va":5383304336,"va":5411052821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27744838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e8baa558fe","size":5,"soft_fail":false,"target_va":5383305920,"va":5411050753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::with_expanded_diameter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27747030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e82a9d58fe","size":5,"soft_fail":false,"target_va":5383305776,"va":5411052801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27739551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e861ba58fe","size":5,"soft_fail":false,"target_va":5383313216,"va":5411052762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::with_expanded_size
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27736827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411049504,"raw_hex":"e805c558fe","size":5,"soft_fail":false,"target_va":5383315920,"va":5411052742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3899,"record_offset":7124,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":19980,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":11076,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":9160,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":10052,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":9216,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3899,"record_offset":6292,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3899,"record_offset":5820,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3899,"record_offset":1236,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3899,"record_offset":2200,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3899,"record_offset":472,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3899,"record_offset":5932,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3899,"record_offset":3008,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3899,"record_offset":6348,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3899,"record_offset":2484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":3899,"record_offset":10024,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":3899,"record_offset":18048,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":3899,"record_offset":18104,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3899,"record_offset":13064,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3899,"record_offset":9436,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":26084,"source_line":3133,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3899,"record_offset":12308,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3899,"record_offset":9328,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":6912,"parent_record_offset":6756,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3899,"record_offset":7584,"parent_record_offset":7528,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3899,"record_offset":7208,"parent_record_offset":7160,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3899,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":6672,"parent_record_offset":6632,"ancestor_record_offset":268,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":7556,"parent_record_offset":7528,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":6756,"parent_record_offset":6712,"ancestor_record_offset":268,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":6712,"parent_record_offset":6672,"ancestor_record_offset":268,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3899,"record_offset":6800,"parent_record_offset":6756,"ancestor_record_offset":268,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3899,"record_offset":6508,"parent_record_offset":6468,"ancestor_record_offset":268,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3899,"record_offset":6548,"parent_record_offset":6508,"ancestor_record_offset":268,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3899,"record_offset":6588,"parent_record_offset":6548,"ancestor_record_offset":268,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3899,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":268,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3899,"record_offset":6428,"parent_record_offset":6388,"ancestor_record_offset":268,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3899,"record_offset":6388,"parent_record_offset":6348,"ancestor_record_offset":268,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":6124,"parent_record_offset":6080,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":268,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":20108,"parent_record_offset":20004,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":6052,"parent_record_offset":6016,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":20488,"parent_record_offset":20004,"ancestor_record_offset":268,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":6632,"parent_record_offset":6588,"ancestor_record_offset":268,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":20248,"parent_record_offset":20004,"ancestor_record_offset":268,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3899,"record_offset":964,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3899,"record_offset":3304,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3899,"record_offset":5988,"parent_record_offset":5956,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3899,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3899,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":268,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3899,"record_offset":3076,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3899,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3899,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":3899,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3899,"record_offset":596,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3899,"record_offset":1276,"parent_record_offset":1236,"ancestor_record_offset":268,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3899,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":268,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3899,"record_offset":560,"parent_record_offset":508,"ancestor_record_offset":268,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3899,"record_offset":6016,"parent_record_offset":5956,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":3860,"parent_record_offset":3724,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5668,"parent_record_offset":5468,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":268,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":268,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":268,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":268,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3899,"record_offset":5468,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3899,"record_offset":3724,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3899,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3899,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3899,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3899,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3899,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3899,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3899,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3899,"record_offset":3492,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3899,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3899,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3899,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3899,"record_offset":508,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3899,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":268,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3899,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":268,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3899,"record_offset":3340,"parent_record_offset":3304,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3899,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":268,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3899,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":268,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3899,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":268,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3899,"record_offset":9508,"parent_record_offset":9468,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3899,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":268,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3899,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":268,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3899,"record_offset":1316,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3899,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":268,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3899,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":268,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3899,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":268,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3899,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":268,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3899,"record_offset":652,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3899,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3899,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3899,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3899,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":268,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3899,"record_offset":18072,"parent_record_offset":18048,"ancestor_record_offset":268,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3899,"record_offset":9784,"parent_record_offset":9660,"ancestor_record_offset":268,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3899,"record_offset":4068,"parent_record_offset":4008,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3899,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":268,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":1352,"parent_record_offset":1316,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":268,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":3184,"parent_record_offset":3100,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":4132,"parent_record_offset":3960,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":3276,"parent_record_offset":3032,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":1412,"parent_record_offset":1316,"ancestor_record_offset":268,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":2332,"parent_record_offset":2252,"ancestor_record_offset":268,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3899,"record_offset":1444,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3899,"record_offset":2904,"parent_record_offset":2600,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3899,"record_offset":7040,"parent_record_offset":6588,"ancestor_record_offset":268,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3899,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3899,"record_offset":7360,"parent_record_offset":7160,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3899,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3899,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3899,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3899,"record_offset":9368,"parent_record_offset":9328,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3899,"record_offset":6244,"parent_record_offset":6080,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3899,"record_offset":708,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3899,"record_offset":1020,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3899,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3899,"record_offset":4212,"parent_record_offset":4180,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3899,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":268,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3899,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":268,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3899,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3899,"record_offset":2408,"parent_record_offset":2356,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":3899,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":268,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":3899,"record_offset":9660,"parent_record_offset":9564,"ancestor_record_offset":268,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":3899,"record_offset":9564,"parent_record_offset":9468,"ancestor_record_offset":268,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":3899,"record_offset":9468,"parent_record_offset":9436,"ancestor_record_offset":268,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3899,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3899,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3899,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3899,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3899,"record_offset":2668,"parent_record_offset":2640,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3899,"record_offset":2700,"parent_record_offset":2668,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3899,"record_offset":2640,"parent_record_offset":2600,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3899,"record_offset":2792,"parent_record_offset":2600,"ancestor_record_offset":268,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3899,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":268,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3899,"record_offset":2848,"parent_record_offset":2600,"ancestor_record_offset":268,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3899,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":268,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3899,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":268,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3899,"record_offset":5596,"parent_record_offset":5564,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3899,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3899,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":268,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   [pdb_inline_transitive] system_tree::hide_trees::hide_trees::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-tree","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/tree-tool/src/hide_trees.rs
//     evidence={"module":3899,"record_offset":12404,"parent_record_offset":12308,"ancestor_record_offset":268,"inlinee":134399,"source_line":88,"archive":"_pdb/C/modules/03899.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x23769
// 首条位置: {"module":4746,"inlinee":145257}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::binary_search
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":4044,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":29203,"source_line":2919,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":3988,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4212,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":36147,"source_line":169,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impl$13::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4746,"record_offset":4148,"parent_record_offset":4068,"ancestor_record_offset":3948,"inlinee":9452,"source_line":397,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":4556,"parent_record_offset":4240,"ancestor_record_offset":3948,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":4524,"parent_record_offset":4268,"ancestor_record_offset":3948,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":3948,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::select_unpredictable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":4746,"record_offset":4176,"parent_record_offset":4068,"ancestor_record_offset":3948,"inlinee":9469,"source_line":829,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":3948,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":3948,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3948,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3948,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":4120,"parent_record_offset":4068,"ancestor_record_offset":3948,"inlinee":29205,"source_line":2923,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3948,"inlinee":13154,"source_line":2970,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3948,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":3948,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":3948,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3948,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4720,"parent_record_offset":4612,"ancestor_record_offset":3948,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":4300,"parent_record_offset":4268,"ancestor_record_offset":3948,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4746,"record_offset":4924,"parent_record_offset":4892,"ancestor_record_offset":3948,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4412,"parent_record_offset":4268,"ancestor_record_offset":3948,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4844,"parent_record_offset":4212,"ancestor_record_offset":3948,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4588,"parent_record_offset":4240,"ancestor_record_offset":3948,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4240,"parent_record_offset":4212,"ancestor_record_offset":3948,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":4268,"parent_record_offset":4240,"ancestor_record_offset":3948,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20CFF
// 首条位置: {"module":3899,"inlinee":134399}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x23768
// 首条位置: {"module":4746,"inlinee":145256}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4746,"record_offset":1740,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":1628,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4746,"record_offset":1796,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":1824,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":36147,"source_line":169,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":2168,"parent_record_offset":1852,"ancestor_record_offset":1592,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":2136,"parent_record_offset":1880,"ancestor_record_offset":1592,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4746,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1592,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1592,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1592,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4746,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1592,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4746,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1592,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4746,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1592,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4746,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1592,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4746,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":1592,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1592,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1592,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":1592,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1592,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":2332,"parent_record_offset":2224,"ancestor_record_offset":1592,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4746,"record_offset":1912,"parent_record_offset":1880,"ancestor_record_offset":1592,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4746,"record_offset":2536,"parent_record_offset":2504,"ancestor_record_offset":1592,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":2024,"parent_record_offset":1880,"ancestor_record_offset":1592,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":2456,"parent_record_offset":1824,"ancestor_record_offset":1592,"inlinee":14720,"source_line":58,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":2200,"parent_record_offset":1852,"ancestor_record_offset":1592,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":1852,"parent_record_offset":1824,"ancestor_record_offset":1592,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/04746.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4746,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":1592,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/04746.jsonl.gz"}
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
// 完整结构及原始类型记录: hide_trees.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hide_trees.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hide_trees.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees
// 信息 ID: d54c65c969e0f150340dceb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 93d1dd25423d8d0fb24626a1: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > @ 0x208d819 -> 0x2861020 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::tree_storage::HideTreesForWallGroupCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> @ 0x20999fc -> 0x2861020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees::closure$0
// 信息 ID: f46f43c00956c45ce2ffcf9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_tree::hide_trees::hide_trees", "captures": "unknown"}
// [闭包环境类型引用] system_tree::hide_trees::hide_trees::closure_env$0 <- country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; module=4746, inline_record=3948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees::closure$1
// 信息 ID: 7af615858ae81f800b9b9797
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_tree::hide_trees::hide_trees", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_tree::hide_trees::hide_trees; module=3899, inline_record=12404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees::closure$2
// 信息 ID: 9b977e860bddfbdfd559c71d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_tree::hide_trees::hide_trees", "captures": "unknown"}
// [闭包环境类型引用] system_tree::hide_trees::hide_trees::closure_env$2 <- country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$2>; module=4746, inline_record=1592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
