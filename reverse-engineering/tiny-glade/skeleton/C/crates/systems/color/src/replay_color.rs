// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/replay_color.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5fd5026b2ce775dec02865a6a49b497a09caa8c4a9413fa8dc8152eaa059fe98
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_color.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2705
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::replay_color::replay_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2705,"record_offset":268,"rva":35074208,"va":5403783328}
// 编译实例 RVAs: 0x21730a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12832436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e8b4cec300","size":5,"soft_fail":false,"target_va":5416621000,"va":5403788559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-28109792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e8201453fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5403788491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30657173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e86b352cfe","size":5,"soft_fail":false,"target_va":5373130976,"va":5403788144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27498438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e83a685cfe","size":5,"soft_fail":false,"target_va":5376289952,"va":5403788385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24988358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e83ab582fe","size":5,"soft_fail":false,"target_va":5378799536,"va":5403787889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24223677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e843608efe","size":5,"soft_fail":false,"target_va":5379560240,"va":5403783912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21904288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e860c4b1fe","size":5,"soft_fail":false,"target_va":5381884032,"va":5403788315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21900841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e8d7d1b1fe","size":5,"soft_fail":false,"target_va":5381887584,"va":5403788420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21900535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e809d3b1fe","size":5,"soft_fail":false,"target_va":5381887680,"va":5403788210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c918c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21900264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e818d4b1fe","size":5,"soft_fail":false,"target_va":5381888192,"va":5403788451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21898218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e816dcb1fe","size":5,"soft_fail":false,"target_va":5381890032,"va":5403788245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21897474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e8fedeb1fe","size":5,"soft_fail":false,"target_va":5381890640,"va":5403788109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12564285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e83db7bf00","size":5,"soft_fail":false,"target_va":5416350384,"va":5403786094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12566301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e81dbfbf00","size":5,"soft_fail":false,"target_va":5416350944,"va":5403784638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12566330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e83abfbf00","size":5,"soft_fail":false,"target_va":5416351056,"va":5403784721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13269061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e84578ca00","size":5,"soft_fail":false,"target_va":5417057072,"va":5403788006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12579386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e83af2bf00","size":5,"soft_fail":false,"target_va":5416365968,"va":5403786577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23064084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403783328,"raw_hex":"e8ec11a0fe","size":5,"soft_fail":false,"target_va":5380719856,"va":5403783935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":488,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":600,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":11852,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2705,"record_offset":9312,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2705,"record_offset":432,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2705,"record_offset":956,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2705,"record_offset":5348,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2705,"record_offset":816,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2705,"record_offset":9444,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2705,"record_offset":516,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2705,"record_offset":9200,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2705,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2705,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2705,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2705,"record_offset":12424,"parent_record_offset":12336,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2705,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2705,"record_offset":9556,"parent_record_offset":9520,"ancestor_record_offset":268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2705,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":668,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":11936,"parent_record_offset":11888,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":12156,"parent_record_offset":11888,"ancestor_record_offset":268,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2705,"record_offset":11888,"parent_record_offset":11852,"ancestor_record_offset":268,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2705,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":268,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":2705,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":268,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2705,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2705,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2705,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":268,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2705,"record_offset":1040,"parent_record_offset":980,"ancestor_record_offset":268,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2705,"record_offset":9272,"parent_record_offset":9232,"ancestor_record_offset":268,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2705,"record_offset":12392,"parent_record_offset":12364,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2705,"record_offset":9472,"parent_record_offset":9444,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2705,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2705,"record_offset":920,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2705,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2705,"record_offset":9384,"parent_record_offset":9312,"ancestor_record_offset":268,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2705,"record_offset":888,"parent_record_offset":852,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2705,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2705,"record_offset":780,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2705,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":268,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2705,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":268,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2705,"record_offset":852,"parent_record_offset":816,"ancestor_record_offset":268,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2705,"record_offset":9616,"parent_record_offset":9556,"ancestor_record_offset":268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2705,"record_offset":9648,"parent_record_offset":9520,"ancestor_record_offset":268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02705.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_color.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_color.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_color.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_color::replay_color::replay_color
// 信息 ID: 8896da41096e49d7d19bb2f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9b71655812441ba1c73f83aa: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x10b918c -> 0x21730a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x2161b60 -> 0x21730a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
