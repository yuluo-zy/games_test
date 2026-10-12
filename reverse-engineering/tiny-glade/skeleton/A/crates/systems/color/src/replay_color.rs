// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/replay_color.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a363462469ef1d583cbe8b44277616fbe7f0bf684f95befc6b586412030ec431
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_color.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2229
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::replay_color::replay_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2229,"record_offset":252,"rva":21639264,"va":5390348384}
// 编译实例 RVAs: 0x14a3060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cee80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18697415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e839b3e2fe","size":5,"soft_fail":false,"target_va":5371653760,"va":5390351170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18695515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8a5bae2fe","size":5,"soft_fail":false,"target_va":5371655632,"va":5390351142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18694967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8c9bce2fe","size":5,"soft_fail":false,"target_va":5371656320,"va":5390351282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18691619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8ddc9e2fe","size":5,"soft_fail":false,"target_va":5371659472,"va":5390351086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18690799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e811cde2fe","size":5,"soft_fail":false,"target_va":5371660432,"va":5390351226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17601614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8b26bf3fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5390351321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14693675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8d5ca1fff","size":5,"soft_fail":false,"target_va":5375654800,"va":5390348470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a75a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11519139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e85d3b50ff","size":5,"soft_fail":false,"target_va":5378831776,"va":5390350910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_color.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13242349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e8ed0fca00","size":5,"soft_fail":false,"target_va":5403593376,"va":5390351022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t-19042254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e83270ddfe","size":5,"soft_fail":false,"target_va":5371306912,"va":5390349161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12627041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e861acc000","size":5,"soft_fail":false,"target_va":5402978432,"va":5390351386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6143090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390348384,"raw_hex":"e88e43a2ff","size":5,"soft_fail":false,"target_va":5384205408,"va":5390348493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":472,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":584,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":6224,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2229,"record_offset":4332,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2229,"record_offset":416,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2229,"record_offset":888,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2229,"record_offset":3892,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2229,"record_offset":712,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2229,"record_offset":4464,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2229,"record_offset":500,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2229,"record_offset":4044,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2229,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2229,"record_offset":6676,"parent_record_offset":6584,"ancestor_record_offset":252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2229,"record_offset":6612,"parent_record_offset":6584,"ancestor_record_offset":252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2229,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2229,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2229,"record_offset":4576,"parent_record_offset":4540,"ancestor_record_offset":252,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2229,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":252,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":624,"parent_record_offset":584,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":6272,"parent_record_offset":6224,"ancestor_record_offset":252,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":996,"parent_record_offset":968,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2229,"record_offset":6432,"parent_record_offset":6224,"ancestor_record_offset":252,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2229,"record_offset":6488,"parent_record_offset":6464,"ancestor_record_offset":252,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2229,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":252,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2229,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":252,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2229,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":252,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2229,"record_offset":968,"parent_record_offset":888,"ancestor_record_offset":252,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2229,"record_offset":4116,"parent_record_offset":4072,"ancestor_record_offset":252,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2229,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":252,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2229,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":252,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2229,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2229,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2229,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2229,"record_offset":4608,"parent_record_offset":4576,"ancestor_record_offset":252,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2229,"record_offset":4404,"parent_record_offset":4332,"ancestor_record_offset":252,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2229,"record_offset":792,"parent_record_offset":748,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2229,"record_offset":4188,"parent_record_offset":4144,"ancestor_record_offset":252,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2229,"record_offset":680,"parent_record_offset":584,"ancestor_record_offset":252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2229,"record_offset":4072,"parent_record_offset":4044,"ancestor_record_offset":252,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2229,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":252,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2229,"record_offset":824,"parent_record_offset":748,"ancestor_record_offset":252,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2229,"record_offset":748,"parent_record_offset":712,"ancestor_record_offset":252,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2229,"record_offset":4636,"parent_record_offset":4576,"ancestor_record_offset":252,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02229.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2229,"record_offset":4668,"parent_record_offset":4540,"ancestor_record_offset":252,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02229.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_color::replay_color::replay_color
// 信息 ID: 2de37b2312efac406c0f5256
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 28ee9a5e1dfc9abb3b949cae: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x9acf00 -> 0x14a3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x149a944 -> 0x14a3060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
