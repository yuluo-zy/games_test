// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/tree-tool/src/replay_tree.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ae2bed0c68e4b7d78c38a9cabf1ee458023be637d0997ff27cc6680d3b82192c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_tree.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2116
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::replay_tree::replay_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2116,"record_offset":248,"rva":21126880,"va":5389836000}
// 编译实例 RVAs: 0x1425ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20204108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8b4b5cbfe","size":5,"soft_fail":false,"target_va":5369632448,"va":5389836551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cee80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18184103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e85988eafe","size":5,"soft_fail":false,"target_va":5371653760,"va":5389837858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18182203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8c58feafe","size":5,"soft_fail":false,"target_va":5371655632,"va":5389837830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18181655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8e991eafe","size":5,"soft_fail":false,"target_va":5371656320,"va":5389837970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18178307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8fd9eeafe","size":5,"soft_fail":false,"target_va":5371659472,"va":5389837774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18177487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e831a2eafe","size":5,"soft_fail":false,"target_va":5371660432,"va":5389837914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17088296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8d840fbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5389838003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14181291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e8559c27ff","size":5,"soft_fail":false,"target_va":5375654800,"va":5389836086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13755661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e80de5d100","size":5,"soft_fail":false,"target_va":5403593376,"va":5389837710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-18531266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e83e3ce5fe","size":5,"soft_fail":false,"target_va":5371305232,"va":5389836493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13140359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e88781c800","size":5,"soft_fail":false,"target_va":5402978432,"va":5389838068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$country_core..systems..history..replay_wall..WallReplayCmd$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11127457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e85f3556ff","size":5,"soft_fail":false,"target_va":5378710144,"va":5389837596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5630706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389836000,"raw_hex":"e80e15aaff","size":5,"soft_fail":false,"target_va":5384205408,"va":5389836109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":472,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":584,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":5392,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2116,"record_offset":3500,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2116,"record_offset":416,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2116,"record_offset":888,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2116,"record_offset":2680,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2116,"record_offset":712,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2116,"record_offset":3632,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2116,"record_offset":500,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2116,"record_offset":3212,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::tree_edit::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/tree_edit.rs
//     evidence={"module":2116,"record_offset":1044,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":80817,"source_line":12,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2116,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2116,"record_offset":1452,"parent_record_offset":1396,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2116,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2116,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2116,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2116,"record_offset":3744,"parent_record_offset":3708,"ancestor_record_offset":248,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2116,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":248,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":624,"parent_record_offset":584,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":5440,"parent_record_offset":5392,"ancestor_record_offset":248,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2116,"record_offset":5600,"parent_record_offset":5392,"ancestor_record_offset":248,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2116,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":248,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2116,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":248,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2116,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2116,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2116,"record_offset":968,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2116,"record_offset":3284,"parent_record_offset":3240,"ancestor_record_offset":248,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2116,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":248,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2116,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2116,"record_offset":1252,"parent_record_offset":1212,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2116,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2116,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2116,"record_offset":3776,"parent_record_offset":3744,"ancestor_record_offset":248,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2116,"record_offset":3572,"parent_record_offset":3500,"ancestor_record_offset":248,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2116,"record_offset":792,"parent_record_offset":748,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2116,"record_offset":3356,"parent_record_offset":3312,"ancestor_record_offset":248,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2116,"record_offset":680,"parent_record_offset":584,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2116,"record_offset":3240,"parent_record_offset":3212,"ancestor_record_offset":248,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2116,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2116,"record_offset":824,"parent_record_offset":748,"ancestor_record_offset":248,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2116,"record_offset":748,"parent_record_offset":712,"ancestor_record_offset":248,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2116,"record_offset":3804,"parent_record_offset":3744,"ancestor_record_offset":248,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02116.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2116,"record_offset":3836,"parent_record_offset":3708,"ancestor_record_offset":248,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02116.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_tree.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_tree.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_tree.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_tree::replay_tree::replay_tree
// 信息 ID: 7ea925980d59de542aab5db2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c86d5acb305ea221a57154cf: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x98cb8b -> 0x1425ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x14160bd -> 0x1425ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
