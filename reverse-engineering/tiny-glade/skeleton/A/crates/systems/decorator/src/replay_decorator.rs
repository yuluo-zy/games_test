// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/replay_decorator.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d7585cc2ec71c418f8e7b6a6a6bc8b28dd5fed5a1666889e32bc7ee9b9b505b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_decorator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2808
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::replay_decorator::replay_decorator
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2808,"record_offset":256,"rva":25110368,"va":5393819488}
// 编译实例 RVAs: 0x17f2760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22169223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e879b9adfe","size":5,"soft_fail":false,"target_va":5371650496,"va":5393819714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cee80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22166775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e809c3adfe","size":5,"soft_fail":false,"target_va":5371653760,"va":5393820530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22164875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e875caadfe","size":5,"soft_fail":false,"target_va":5371655632,"va":5393820502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22164327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e899ccadfe","size":5,"soft_fail":false,"target_va":5371656320,"va":5393820642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22160979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8add9adfe","size":5,"soft_fail":false,"target_va":5371659472,"va":5393820446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22160159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8e1dcadfe","size":5,"soft_fail":false,"target_va":5371660432,"va":5393820586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21070968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8887bbefe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393820675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069fb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18164751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8f1d3eafe","size":5,"soft_fail":false,"target_va":5375654800,"va":5393819546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14168412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8a4ce27ff","size":5,"soft_fail":false,"target_va":5379651280,"va":5393819687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d04a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11460625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8ef1f51ff","size":5,"soft_fail":false,"target_va":5382359648,"va":5393820268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9772989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8bd1f9500","size":5,"soft_fail":false,"target_va":5403593376,"va":5393820382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-22514835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e86d73a8fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5393819886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9157709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e84dbc8b00","size":5,"soft_fail":false,"target_va":5402978432,"va":5393820718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9614166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393819488,"raw_hex":"e8aa4c6dff","size":5,"soft_fail":false,"target_va":5384205408,"va":5393819569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":1412,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":1720,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":4140,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2808,"record_offset":2248,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2808,"record_offset":436,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2808,"record_offset":1692,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2808,"record_offset":1524,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2808,"record_offset":492,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2808,"record_offset":1224,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2808,"record_offset":1440,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2808,"record_offset":1960,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":2808,"record_offset":660,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21699,"source_line":243,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2808,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2808,"record_offset":4592,"parent_record_offset":4500,"ancestor_record_offset":256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2808,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2808,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2808,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2808,"record_offset":1284,"parent_record_offset":1248,"ancestor_record_offset":256,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":2808,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":256,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":1752,"parent_record_offset":1720,"ancestor_record_offset":256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":4188,"parent_record_offset":4140,"ancestor_record_offset":256,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2808,"record_offset":4348,"parent_record_offset":4140,"ancestor_record_offset":256,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2808,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":256,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2808,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":256,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2808,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2808,"record_offset":1016,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":17354,"source_line":32,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2808,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":256,"inlinee":17352,"source_line":73,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":2808,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":256,"inlinee":18208,"source_line":41,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":2808,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":256,"inlinee":18206,"source_line":153,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2808,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":256,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2808,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":256,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2808,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":256,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2808,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":256,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2808,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2808,"record_offset":2032,"parent_record_offset":1988,"ancestor_record_offset":256,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2808,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":256,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2808,"record_offset":1156,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2808,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2808,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":256,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2808,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2808,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2808,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2808,"record_offset":1316,"parent_record_offset":1284,"ancestor_record_offset":256,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2808,"record_offset":2320,"parent_record_offset":2248,"ancestor_record_offset":256,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2808,"record_offset":960,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2808,"record_offset":932,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2808,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2808,"record_offset":568,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2808,"record_offset":1988,"parent_record_offset":1960,"ancestor_record_offset":256,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2808,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":256,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2808,"record_offset":1628,"parent_record_offset":1552,"ancestor_record_offset":256,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2808,"record_offset":888,"parent_record_offset":860,"ancestor_record_offset":256,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2808,"record_offset":1552,"parent_record_offset":1524,"ancestor_record_offset":256,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2808,"record_offset":1344,"parent_record_offset":1284,"ancestor_record_offset":256,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2808,"record_offset":1376,"parent_record_offset":1248,"ancestor_record_offset":256,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2808,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":256,"inlinee":18288,"source_line":6499,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2808,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":256,"inlinee":18212,"source_line":6511,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$11::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":2808,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":256,"inlinee":18286,"source_line":1354,"archive":"_pdb/A/modules/02808.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":2808,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":256,"inlinee":18284,"source_line":1072,"archive":"_pdb/A/modules/02808.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_decorator.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_decorator.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_decorator.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>

// ------------------------------------------------------------------
// 函数: system_decorator::replay_decorator::replay_decorator
// 信息 ID: c1cb36a9a4de6874d828d955
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ec1197e5251cda0eee2cfcfd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)> @ 0xd1caca -> 0x17f2760 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)> > @ 0x17caafc -> 0x17f2760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
