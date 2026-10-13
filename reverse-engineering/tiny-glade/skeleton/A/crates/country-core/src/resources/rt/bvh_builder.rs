// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/rt/bvh_builder.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 511683dff258130a09554976e668e452ffe1ffa162e20428ff263f0f15539959
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh_builder.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 416, 446, 496, 1308, 1321, 1351
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.24.1"},{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::RtBvhBuilderGlue::is_idle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CF5
// 首条位置: {"module":446,"inlinee":23797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":446,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":11652,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00446.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":446,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11652,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00446.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":416,"record_offset":7052,"rva":5784528,"va":5374493648}
// 编译实例 RVAs: 0x5843d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bf5f0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2903790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e812b1d3ff","size":5,"soft_fail":false,"target_va":5371590128,"va":5374493913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2836408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e848b8d4ff","size":5,"soft_fail":false,"target_va":5371659056,"va":5374495459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2222968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e88814deff","size":5,"soft_fail":false,"target_va":5372272592,"va":5374495555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1744715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e8b560e5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374494422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e6420
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1447963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e81b181600","size":5,"soft_fail":false,"target_va":5375943712,"va":5374495744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh_builder.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8390888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e8e8088000","size":5,"soft_fail":false,"target_va":5382885008,"va":5374494115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e867f0ffff","size":5,"soft_fail":false,"target_va":5374491872,"va":5374495860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rt::bvh::build_gpu_bvh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t841985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e801d90c00","size":5,"soft_fail":false,"target_va":5375336128,"va":5374494138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2858387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e8939d2b00","size":5,"soft_fail":false,"target_va":5377353776,"va":5374495384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28481095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e84796b201","size":5,"soft_fail":false,"target_va":5402975328,"va":5374494228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28483920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e850a1b201","size":5,"soft_fail":false,"target_va":5402977792,"va":5374493867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28483271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e8c79eb201","size":5,"soft_fail":false,"target_va":5402977488,"va":5374494212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2899316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374493648,"raw_hex":"e8743d2c00","size":5,"soft_fail":false,"target_va":5377393200,"va":5374493879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":416,"record_offset":7244,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":416,"record_offset":13244,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":416,"record_offset":13272,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":416,"record_offset":9404,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":416,"record_offset":9340,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":12473,"source_line":929,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":7908,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":416,"record_offset":11540,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":416,"record_offset":7792,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":8336,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":416,"record_offset":13300,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":416,"record_offset":13160,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":416,"record_offset":8900,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":416,"record_offset":7276,"parent_record_offset":7052,"ancestor_record_offset":7052,"inlinee":6693,"source_line":359,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":7052,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":10184,"parent_record_offset":10084,"ancestor_record_offset":7052,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":10116,"parent_record_offset":10084,"ancestor_record_offset":7052,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":416,"record_offset":10084,"parent_record_offset":10056,"ancestor_record_offset":7052,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":416,"record_offset":13820,"parent_record_offset":13784,"ancestor_record_offset":7052,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":416,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":7052,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":416,"record_offset":9428,"parent_record_offset":9404,"ancestor_record_offset":7052,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":416,"record_offset":9192,"parent_record_offset":9164,"ancestor_record_offset":7052,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":416,"record_offset":9252,"parent_record_offset":9220,"ancestor_record_offset":7052,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":416,"record_offset":9368,"parent_record_offset":9340,"ancestor_record_offset":7052,"inlinee":12475,"source_line":23,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":416,"record_offset":9452,"parent_record_offset":9428,"ancestor_record_offset":7052,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":7052,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":416,"record_offset":10148,"parent_record_offset":10116,"ancestor_record_offset":7052,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":416,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":7052,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":7052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":416,"record_offset":13852,"parent_record_offset":13820,"ancestor_record_offset":7052,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":7052,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":7872,"parent_record_offset":7816,"ancestor_record_offset":7052,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":416,"record_offset":7436,"parent_record_offset":7352,"ancestor_record_offset":7052,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":7568,"parent_record_offset":7540,"ancestor_record_offset":7052,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":8172,"parent_record_offset":7996,"ancestor_record_offset":7052,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":7352,"parent_record_offset":7316,"ancestor_record_offset":7052,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":8264,"parent_record_offset":8232,"ancestor_record_offset":7052,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":7052,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":7404,"parent_record_offset":7352,"ancestor_record_offset":7052,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":7052,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":7052,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7052,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":416,"record_offset":13912,"parent_record_offset":13784,"ancestor_record_offset":7052,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":416,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":7052,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":9104,"parent_record_offset":9012,"ancestor_record_offset":7052,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":7052,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":9076,"parent_record_offset":9012,"ancestor_record_offset":7052,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":9048,"parent_record_offset":9012,"ancestor_record_offset":7052,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":7052,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":416,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":7052,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":416,"record_offset":9164,"parent_record_offset":9104,"ancestor_record_offset":7052,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":416,"record_offset":9220,"parent_record_offset":9164,"ancestor_record_offset":7052,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":416,"record_offset":8928,"parent_record_offset":8900,"ancestor_record_offset":7052,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":416,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":7052,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":416,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":7052,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":416,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":7052,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":416,"record_offset":7540,"parent_record_offset":7504,"ancestor_record_offset":7052,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":416,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":7052,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":416,"record_offset":7476,"parent_record_offset":7276,"ancestor_record_offset":7052,"inlinee":6696,"source_line":590,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$32::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":416,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":7052,"inlinee":6705,"source_line":722,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":416,"record_offset":7316,"parent_record_offset":7276,"ancestor_record_offset":7052,"inlinee":6694,"source_line":152,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":416,"record_offset":8232,"parent_record_offset":7964,"ancestor_record_offset":7052,"inlinee":6707,"source_line":159,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":416,"record_offset":7636,"parent_record_offset":7504,"ancestor_record_offset":7052,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF157
// 首条位置: {"module":1321,"inlinee":61783}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":68032,"parent_record_offset":67948,"ancestor_record_offset":67948,"inlinee":29297,"source_line":1214,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::DirtyCow::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":1321,"record_offset":67976,"parent_record_offset":67948,"ancestor_record_offset":67948,"inlinee":23268,"source_line":99,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1321,"record_offset":68000,"parent_record_offset":67976,"ancestor_record_offset":67948,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":68108,"parent_record_offset":68084,"ancestor_record_offset":67948,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":68084,"parent_record_offset":68056,"ancestor_record_offset":67948,"inlinee":6167,"source_line":1973,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1321,"record_offset":68056,"parent_record_offset":68032,"ancestor_record_offset":67948,"inlinee":29299,"source_line":1836,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC5
// 首条位置: {"module":496,"inlinee":27845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1308,"record_offset":25060,"parent_record_offset":25016,"ancestor_record_offset":25016,"inlinee":26366,"source_line":370,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1308,"record_offset":25204,"parent_record_offset":25104,"ancestor_record_offset":25016,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1308,"record_offset":25176,"parent_record_offset":25104,"ancestor_record_offset":25016,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1308,"record_offset":25144,"parent_record_offset":25104,"ancestor_record_offset":25016,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.24.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.24.1/src/f32/sse2/vec3a.rs
//     evidence={"module":1308,"record_offset":25104,"parent_record_offset":25060,"ancestor_record_offset":25016,"inlinee":23066,"source_line":851,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1321,"record_offset":68624,"parent_record_offset":68572,"ancestor_record_offset":68536,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC1
// 首条位置: {"module":496,"inlinee":27841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$181::all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":496,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":14826,"source_line":262,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":496,"record_offset":668,"parent_record_offset":616,"ancestor_record_offset":508,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":496,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":508,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh_builder.rs
//     evidence={"module":496,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":508,"inlinee":27843,"source_line":89,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":496,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":508,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC3
// 首条位置: {"module":496,"inlinee":27843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":496,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":21853,"source_line":412,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":496,"record_offset":668,"parent_record_offset":616,"ancestor_record_offset":560,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":496,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":560,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00496.jsonl.gz"}
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
// 完整结构及原始类型记录: bvh_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::InputTriangle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::resources::rt::bvh_builder::InputTriangle,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>", "constructor": "core::iter::adapters::filter_map::impl$2::next", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>", "country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["country_core::resources::rt::bvh_builder::InputTriangle", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >", "constructor": "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0", "arguments": ["tuple$<array$<glam::f32::vec3::Vec3,3>,u32>", "country_core::resources::rt::bvh_builder::InputTriangle", "ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >", "constructor": "core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "constructor": "core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "tuple$<>", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >", "core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >", "std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >", "country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0", "tuple$<>", "core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::RtBvhBuilderGlue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "std::sync::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::rwlock::RwLock", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<std::sync::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["std::sync::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::rwlock::RwLockWriteGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::result::Result", "arguments": ["std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > >", "constructor": "enum2$", "arguments": ["core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>", "constructor": "std::sync::rwlock::RwLockReadGuard", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderGlue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::rwlock::RwLockReadGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::rwlock::RwLockWriteGuard<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh_builder::RtBvhBuilderOutput
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput>", "constructor": "core::option::Option", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderOutput"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::rt::bvh_builder::RtBvhBuilderOutput>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::rt::bvh_builder::RtBvhBuilderOutput"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bvh_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bvh_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::RtBvhBuilderGlue::is_idle
// 信息 ID: dccccd99a134cb52beef5eb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::RtBvhBuilderGlueResource::is_idle; module=446, inline_record=11652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker
// 信息 ID: e3056b1f8e18e436a7c27015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$6,tuple$<> > @ 0x1b374 -> 0x5843d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0
// 信息 ID: df14654f1ae1530aaf5f388f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1321, inline_record=67948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure$0
// 信息 ID: 4bb6fcffcedf7028788e1d81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1308, inline_record=25016; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1308, inline_record=26804; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=1308, inline_record=28304; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; module=1321, inline_record=68536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1
// 信息 ID: 6be50c7b8d0e87e5f95a67c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::filter_map::impl$2::next<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::bvh_builder::InputTriangle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,std::collections::hash::map::Values<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$33::iter_try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::adapters::flatten::impl$35::try_fold::flatten::closure_env$0<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<glam::f32::vec3::Vec3>,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$0::closure_env$0>,core::iter::sources::repeat::Repeat<u32> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > >,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<array$<glam::f32::vec3::Vec3,3>,u32>,country_core::resources::rt::bvh_builder::InputTriangle,ref_mut$<country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1> > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::rt::bvh_builder::InputTriangle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=496, inline_record=508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh_builder::bvh_builder_worker::closure$1::closure$0
// 信息 ID: de823f2d606ef1321b7b9790
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh_builder::bvh_builder_worker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<array$<glam::f32::vec3::Vec3,3>,u32> >,country_core::resources::rt::bvh_builder::bvh_builder_worker::closure_env$1>; module=496, inline_record=560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
