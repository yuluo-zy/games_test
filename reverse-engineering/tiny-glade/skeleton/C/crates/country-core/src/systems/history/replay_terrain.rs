// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/history/replay_terrain.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 60637483c4640e9d8095f0a4327b05af136a3293a5b51c01eaa0bf04f0d6c4fa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 594
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::replay_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":594,"record_offset":24248,"rva":12719264,"va":5381428384}
// 编译实例 RVAs: 0xc214a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35191023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8eff81802","size":5,"soft_fail":false,"target_va":5416621000,"va":5381429972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5750439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e85941a8ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381429122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5750609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8af40a8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381429308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12706922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8961b3eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381429109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a7fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12085161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8a967b800","size":5,"soft_fail":false,"target_va":5393514432,"va":5381429266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35627886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e86ea31f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381429165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35627299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e823a11f02","size":5,"soft_fail":false,"target_va":5417057072,"va":5381429768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34936939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e86b181502","size":5,"soft_fail":false,"target_va":5416365968,"va":5381429024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35191378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e852fa1802","size":5,"soft_fail":false,"target_va":5416620794,"va":5381429411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35192252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8bcfd1802","size":5,"soft_fail":false,"target_va":5416622208,"va":5381429951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e87b240000","size":5,"soft_fail":false,"target_va":5381439168,"va":5381429824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::reapply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3139798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e82a17d0ff","size":5,"soft_fail":false,"target_va":5378289184,"va":5381428977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::terrain_edit::TerrainEdit::revert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3141391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381428384,"raw_hex":"e8f110d0ff","size":5,"soft_fail":false,"target_va":5378287536,"va":5381428922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":594,"record_offset":25480,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":24880,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":24992,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":27232,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25604,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":594,"record_offset":27092,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":594,"record_offset":24448,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":594,"record_offset":25208,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":26632,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":24908,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":26980,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":594,"record_offset":24512,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::terrain_edit_animation_progress
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":594,"record_offset":25348,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":29063,"source_line":330,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::set_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":594,"record_offset":26212,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":34958,"source_line":110,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26040,"parent_record_offset":26016,"ancestor_record_offset":24248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26064,"parent_record_offset":26040,"ancestor_record_offset":24248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":24248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":24248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":24248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26568,"parent_record_offset":26544,"ancestor_record_offset":24248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26016,"parent_record_offset":25836,"ancestor_record_offset":24248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":594,"record_offset":26496,"parent_record_offset":26412,"ancestor_record_offset":24248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24648,"parent_record_offset":24620,"ancestor_record_offset":24248,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24536,"parent_record_offset":24512,"ancestor_record_offset":24248,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24560,"parent_record_offset":24536,"ancestor_record_offset":24248,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24748,"parent_record_offset":24560,"ancestor_record_offset":24248,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24592,"parent_record_offset":24560,"ancestor_record_offset":24248,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":24620,"parent_record_offset":24592,"ancestor_record_offset":24248,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":24680,"parent_record_offset":24648,"ancestor_record_offset":24248,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":24780,"parent_record_offset":24748,"ancestor_record_offset":24248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":25776,"parent_record_offset":25748,"ancestor_record_offset":24248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":24248,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":26440,"parent_record_offset":26412,"ancestor_record_offset":24248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":26412,"parent_record_offset":26388,"ancestor_record_offset":24248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":24828,"parent_record_offset":24804,"ancestor_record_offset":24248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":24804,"parent_record_offset":24780,"ancestor_record_offset":24248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":25836,"parent_record_offset":25804,"ancestor_record_offset":24248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":25804,"parent_record_offset":25776,"ancestor_record_offset":24248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":26388,"parent_record_offset":26364,"ancestor_record_offset":24248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":25872,"parent_record_offset":25836,"ancestor_record_offset":24248,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":594,"record_offset":25704,"parent_record_offset":25636,"ancestor_record_offset":24248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25060,"parent_record_offset":25020,"ancestor_record_offset":24248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25676,"parent_record_offset":25636,"ancestor_record_offset":24248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":27312,"parent_record_offset":27268,"ancestor_record_offset":24248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25020,"parent_record_offset":24992,"ancestor_record_offset":24248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":27528,"parent_record_offset":27268,"ancestor_record_offset":24248,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":27268,"parent_record_offset":27232,"ancestor_record_offset":24248,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25748,"parent_record_offset":25704,"ancestor_record_offset":24248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":594,"record_offset":25636,"parent_record_offset":25604,"ancestor_record_offset":24248,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":594,"record_offset":27588,"parent_record_offset":27564,"ancestor_record_offset":24248,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":594,"record_offset":27636,"parent_record_offset":27612,"ancestor_record_offset":24248,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":594,"record_offset":24476,"parent_record_offset":24448,"ancestor_record_offset":24248,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":594,"record_offset":25896,"parent_record_offset":25872,"ancestor_record_offset":24248,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":594,"record_offset":26260,"parent_record_offset":26212,"ancestor_record_offset":24248,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":25388,"parent_record_offset":25348,"ancestor_record_offset":24248,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":24248,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":594,"record_offset":25976,"parent_record_offset":25920,"ancestor_record_offset":24248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":594,"record_offset":27052,"parent_record_offset":27012,"ancestor_record_offset":24248,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":25920,"parent_record_offset":25896,"ancestor_record_offset":24248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":25948,"parent_record_offset":25920,"ancestor_record_offset":24248,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":594,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":24248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":594,"record_offset":26144,"parent_record_offset":25704,"ancestor_record_offset":24248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":26168,"parent_record_offset":26144,"ancestor_record_offset":24248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":26316,"parent_record_offset":26212,"ancestor_record_offset":24248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":594,"record_offset":24956,"parent_record_offset":24932,"ancestor_record_offset":24248,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":594,"record_offset":25312,"parent_record_offset":25244,"ancestor_record_offset":24248,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":594,"record_offset":27172,"parent_record_offset":27092,"ancestor_record_offset":24248,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":594,"record_offset":25280,"parent_record_offset":25244,"ancestor_record_offset":24248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":27196,"parent_record_offset":27172,"ancestor_record_offset":24248,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":25172,"parent_record_offset":25020,"ancestor_record_offset":24248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":27012,"parent_record_offset":26980,"ancestor_record_offset":24248,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":594,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":24248,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":594,"record_offset":25244,"parent_record_offset":25208,"ancestor_record_offset":24248,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":594,"record_offset":26288,"parent_record_offset":26212,"ancestor_record_offset":24248,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":594,"record_offset":25420,"parent_record_offset":25348,"ancestor_record_offset":24248,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":594,"record_offset":28604,"rva":12721328,"va":5381430448}
// 编译实例 RVAs: 0xc21cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t456974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430448,"raw_hex":"e80ef90600","size":5,"soft_fail":false,"target_va":5381887680,"va":5381430701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180fb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12508753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430448,"raw_hex":"e851debe00","size":5,"soft_fail":false,"target_va":5393939296,"va":5381430538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35191542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430448,"raw_hex":"e8f6fa1802","size":5,"soft_fail":false,"target_va":5416622208,"va":5381430661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":29108,"parent_record_offset":28604,"ancestor_record_offset":28604,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":594,"record_offset":28772,"parent_record_offset":28604,"ancestor_record_offset":28604,"inlinee":29061,"source_line":78,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::TerrainReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":594,"record_offset":29080,"parent_record_offset":28604,"ancestor_record_offset":28604,"inlinee":37831,"source_line":51,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":594,"record_offset":29132,"parent_record_offset":29108,"ancestor_record_offset":28604,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29044,"parent_record_offset":28772,"ancestor_record_offset":28604,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29016,"parent_record_offset":28772,"ancestor_record_offset":28604,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":594,"record_offset":28928,"parent_record_offset":28904,"ancestor_record_offset":28604,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":594,"record_offset":28984,"parent_record_offset":28960,"ancestor_record_offset":28604,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":28904,"parent_record_offset":28772,"ancestor_record_offset":28604,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":28960,"parent_record_offset":28772,"ancestor_record_offset":28604,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":594,"record_offset":28868,"parent_record_offset":28844,"ancestor_record_offset":28604,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":28820,"parent_record_offset":28772,"ancestor_record_offset":28604,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":594,"record_offset":28844,"parent_record_offset":28820,"ancestor_record_offset":28604,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29168,"parent_record_offset":29132,"ancestor_record_offset":28604,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29192,"parent_record_offset":29168,"ancestor_record_offset":28604,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29224,"parent_record_offset":29132,"ancestor_record_offset":28604,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":594,"record_offset":29264,"rva":12721600,"va":5381430720}
// 编译实例 RVAs: 0xc21dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t456702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430720,"raw_hex":"e8fef70600","size":5,"soft_fail":false,"target_va":5381887680,"va":5381430973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180fb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12508481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430720,"raw_hex":"e841ddbe00","size":5,"soft_fail":false,"target_va":5393939296,"va":5381430810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35191270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381430720,"raw_hex":"e8e6f91802","size":5,"soft_fail":false,"target_va":5416622208,"va":5381430933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":594,"record_offset":29768,"parent_record_offset":29264,"ancestor_record_offset":29264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::terrain_edit::TerrainEdit::duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/terrain_edit.rs
//     evidence={"module":594,"record_offset":29432,"parent_record_offset":29264,"ancestor_record_offset":29264,"inlinee":29061,"source_line":78,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_terrain::TerrainReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_terrain.rs
//     evidence={"module":594,"record_offset":29740,"parent_record_offset":29264,"ancestor_record_offset":29264,"inlinee":37831,"source_line":51,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":594,"record_offset":29792,"parent_record_offset":29768,"ancestor_record_offset":29264,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29704,"parent_record_offset":29432,"ancestor_record_offset":29264,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":594,"record_offset":29676,"parent_record_offset":29432,"ancestor_record_offset":29264,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":594,"record_offset":29588,"parent_record_offset":29564,"ancestor_record_offset":29264,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":594,"record_offset":29644,"parent_record_offset":29620,"ancestor_record_offset":29264,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":29564,"parent_record_offset":29432,"ancestor_record_offset":29264,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":594,"record_offset":29620,"parent_record_offset":29432,"ancestor_record_offset":29264,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":594,"record_offset":29528,"parent_record_offset":29504,"ancestor_record_offset":29264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":594,"record_offset":29480,"parent_record_offset":29432,"ancestor_record_offset":29264,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":594,"record_offset":29504,"parent_record_offset":29480,"ancestor_record_offset":29264,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29828,"parent_record_offset":29792,"ancestor_record_offset":29264,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29852,"parent_record_offset":29828,"ancestor_record_offset":29264,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":594,"record_offset":29884,"parent_record_offset":29792,"ancestor_record_offset":29264,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93C7
// 首条位置: {"module":594,"inlinee":37831}
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
// 完整结构及原始类型记录: replay_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_terrain::TerrainReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::backward
// 信息 ID: ff7f105ff8fef8413e18c5e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0> @ 0x17f6ca1 -> 0xc21dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::forward
// 信息 ID: e03bd7ee13674e358131d087
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0> @ 0x17f6cd7 -> 0xc21cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::TerrainReplayCmd::new
// 信息 ID: 5665bf1841eecdf2bcc61fac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::forward; module=594, inline_record=29080; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_terrain::TerrainReplayCmd::backward; module=594, inline_record=29740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_terrain::replay_terrain
// 信息 ID: e59280ecc9c7d7edc020bf85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 77fb54d0548f41cdef2beb5c: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> @ 0x18c75ea -> 0xc214a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> > @ 0x193d3f1 -> 0xc214a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
