// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 37409d5a7eec2db599e19c61724520810d98e8309977176ebb8b5ddb5412f5ff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1608
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use dxgi_enumerate_gpus; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use windows; // namespace candidate
//   package identities: [{"name":"windows","source":"PDB_registry_path_only","version":"0.37.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::enumerate
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1608,"record_offset":256,"rva":18370448,"va":5387079568}
// 编译实例 RVAs: 0x1184f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7945971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8f33e7900","size":5,"soft_fail":false,"target_va":5395026064,"va":5387080088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$windows..core..error..Error$u20$as$u20$core..convert..From$LT$windows..core..hresult..HRESULT$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-270298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e826e0fbff","size":5,"soft_fail":false,"target_va":5386810128,"va":5387080421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$windows..core..unknown..IUnknown$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-268122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8a6e8fbff","size":5,"soft_fail":false,"target_va":5386812176,"va":5387080293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402db2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15376920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8e85d15ff","size":5,"soft_fail":false,"target_va":5371704048,"va":5387080963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14330976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8a05325ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387080683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141918fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7946048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8403f7900","size":5,"soft_fail":false,"target_va":5395025824,"va":5387079771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141919010
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7946121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8893f7900","size":5,"soft_fail":false,"target_va":5395025936,"va":5387079810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16513510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e8e6f9fb00","size":5,"soft_fail":false,"target_va":5403593360,"va":5387079845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15839036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e83caff100","size":5,"soft_fail":false,"target_va":5402919072,"va":5387080031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15772883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e82d530fff","size":5,"soft_fail":false,"target_va":5371307280,"va":5387080158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::haf11dc4108d347f5 (.llvm.5396423983225260351)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16445186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e802effa00","size":5,"soft_fail":false,"target_va":5403525792,"va":5387080601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9684953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387079568,"raw_hex":"e827386cff","size":5,"soft_fail":false,"target_va":5377395552,"va":5387080500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24207718(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387079568,"raw_hex":"ff1566617101","size":6,"soft_fail":false,"va":5387079668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":396,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":1564,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1608,"record_offset":2464,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1608,"record_offset":1932,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1608,"record_offset":2684,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] dxgi_enumerate_gpus::get_string_from_wide
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1608,"record_offset":764,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":70197,"source_line":36,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] windows::Windows::Win32::Graphics::Dxgi::CreateDXGIFactory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/windows-0.37.0/src/Windows/Win32/Graphics/Dxgi/mod.rs
//     evidence={"module":1608,"record_offset":424,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":70195,"source_line":5,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1608,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1608,"record_offset":1476,"parent_record_offset":1420,"ancestor_record_offset":256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1640,"parent_record_offset":1608,"ancestor_record_offset":256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":256,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf16_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":940,"parent_record_offset":764,"ancestor_record_offset":256,"inlinee":31814,"source_line":724,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1044,"parent_record_offset":1004,"ancestor_record_offset":256,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1192,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":16681,"source_line":1165,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$13::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1072,"parent_record_offset":1004,"ancestor_record_offset":256,"inlinee":16680,"source_line":2187,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1004,"parent_record_offset":972,"ancestor_record_offset":256,"inlinee":16712,"source_line":2108,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":1608,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":256,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":256,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1608,"record_offset":2532,"parent_record_offset":2496,"ancestor_record_offset":256,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1608,"record_offset":2496,"parent_record_offset":2464,"ancestor_record_offset":256,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::char::decode::impl$0::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\decode.rs
//     evidence={"module":1608,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":256,"inlinee":31816,"source_line":76,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1608,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1608,"record_offset":972,"parent_record_offset":940,"ancestor_record_offset":256,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1608,"record_offset":1308,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1608,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":256,"inlinee":8872,"source_line":2605,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1608,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1608,"record_offset":532,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1608,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1608,"record_offset":1672,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1608,"record_offset":872,"parent_record_offset":800,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1608,"record_offset":800,"parent_record_offset":764,"ancestor_record_offset":256,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] dxgi_enumerate_gpus::get_string_from_wide::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1608,"record_offset":832,"parent_record_offset":800,"ancestor_record_offset":256,"inlinee":70199,"source_line":37,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] windows::core::hresult::HRESULT::and_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/windows-0.37.0/src/core/hresult.rs
//     evidence={"module":1608,"record_offset":464,"parent_record_offset":424,"ancestor_record_offset":256,"inlinee":28124,"source_line":46,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] windows::core::hresult::HRESULT::is_ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/windows-0.37.0/src/core/hresult.rs
//     evidence={"module":1608,"record_offset":504,"parent_record_offset":464,"ancestor_record_offset":256,"inlinee":28077,"source_line":14,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11235
// 首条位置: {"module":1608,"inlinee":70197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::from_utf16_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":940,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":31814,"source_line":724,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1608,"record_offset":800,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":8990,"source_line":331,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1608,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1608,"record_offset":1476,"parent_record_offset":1420,"ancestor_record_offset":764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":764,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":764,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1608,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1044,"parent_record_offset":1004,"ancestor_record_offset":764,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1192,"parent_record_offset":1072,"ancestor_record_offset":764,"inlinee":16681,"source_line":1165,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$13::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1072,"parent_record_offset":1004,"ancestor_record_offset":764,"inlinee":16680,"source_line":2187,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1608,"record_offset":1004,"parent_record_offset":972,"ancestor_record_offset":764,"inlinee":16712,"source_line":2108,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1608,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":764,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::char::decode::impl$0::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\char\decode.rs
//     evidence={"module":1608,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":764,"inlinee":31816,"source_line":76,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1608,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":764,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1608,"record_offset":972,"parent_record_offset":940,"ancestor_record_offset":764,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1608,"record_offset":1308,"parent_record_offset":1072,"ancestor_record_offset":764,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1608,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":764,"inlinee":8872,"source_line":2605,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1608,"record_offset":1340,"parent_record_offset":1004,"ancestor_record_offset":764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1608,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1608,"record_offset":872,"parent_record_offset":800,"ancestor_record_offset":764,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   [pdb_inline_transitive] dxgi_enumerate_gpus::get_string_from_wide::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1608,"record_offset":832,"parent_record_offset":800,"ancestor_record_offset":764,"inlinee":70199,"source_line":37,"archive":"_pdb/A/modules/01608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11237
// 首条位置: {"module":1608,"inlinee":70199}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::enumerate
// 信息 ID: c2a27ea09736aa07fd3a5196
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices @ 0x73295f -> 0x1184f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::get_string_from_wide
// 信息 ID: 86cbb024079e46a27564c2b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] dxgi_enumerate_gpus::enumerate; module=1608, inline_record=764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::get_string_from_wide::closure$0
// 信息 ID: 85be62b0592b396e9a3a4bef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "dxgi_enumerate_gpus::get_string_from_wide", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] dxgi_enumerate_gpus::enumerate; module=1608, inline_record=832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
