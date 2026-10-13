// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/load_texture.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a003c3174eefb298562b2a71acf1c47fd041e97e0f498d075d58a7556794cabc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 436
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"PDB_registry_path_only","version":"0.5.8"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use rkyv; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"PDB_registry_path_only","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_compiled_texture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":436,"record_offset":7776,"rva":6142624,"va":5374851744}
// 编译实例 RVAs: 0x5dbaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11284715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8eb30ac00","size":5,"soft_fail":false,"target_va":5386138016,"va":5374853296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5771240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e818f0a7ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5374854675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2540212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e84c3dd9ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374854415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2102904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e888e9dfff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374852611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t318547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e853dc0400","size":5,"soft_fail":false,"target_va":5375171056,"va":5374852504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2584021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8d56d2700","size":5,"soft_fail":false,"target_va":5377436496,"va":5374852470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8455940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e804078100","size":5,"soft_fail":false,"target_va":5383308144,"va":5374852199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e9e1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9183568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e850218c00","size":5,"soft_fail":false,"target_va":5384036816,"va":5374853243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9980
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28104478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e81ed7ac01","size":5,"soft_fail":false,"target_va":5402958208,"va":5374853725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28105036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e84cd9ac01","size":5,"soft_fail":false,"target_va":5402958656,"va":5374853615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28124679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e80726ad01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374854004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28125402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8da28ad01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374853537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e866f1ffff","size":5,"soft_fail":false,"target_va":5374849616,"va":5374853349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28127872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e88032ad01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374852251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28126737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8112ead01","size":5,"soft_fail":false,"target_va":5402980304,"va":5374853562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e81df8ffff","size":5,"soft_fail":false,"target_va":5374849808,"va":5374851822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28128752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8f035ad01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374851899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2500993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e881292600","size":5,"soft_fail":false,"target_va":5377353776,"va":5374852778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11285678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8ae34ac00","size":5,"soft_fail":false,"target_va":5386138608,"va":5374852925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1195330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8423d1200","size":5,"soft_fail":false,"target_va":5376048224,"va":5374852889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1504498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8f2f41600","size":5,"soft_fail":false,"target_va":5376356912,"va":5374852409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1508376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e818041700","size":5,"soft_fail":false,"target_va":5376360560,"va":5374852179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1504337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e851f41600","size":5,"soft_fail":false,"target_va":5376356144,"va":5374851802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::array_elements
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1488000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e880b41600","size":5,"soft_fail":false,"target_va":5376341168,"va":5374853163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1487785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8a9b31600","size":5,"soft_fail":false,"target_va":5376340928,"va":5374853138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1487376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e810b21600","size":5,"soft_fail":false,"target_va":5376340464,"va":5374853083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1487616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e800b31600","size":5,"soft_fail":false,"target_va":5376340720,"va":5374853099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2555146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e80afd2600","size":5,"soft_fail":false,"target_va":5377408016,"va":5374852865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2587236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8647a2700","size":5,"soft_fail":false,"target_va":5377439776,"va":5374852535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_with_extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2587750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8667c2700","size":5,"soft_fail":false,"target_va":5377440336,"va":5374852581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::as_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1495014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8e6cf1600","size":5,"soft_fail":false,"target_va":5376348208,"va":5374853189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1494467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374851744,"raw_hex":"e8c3cd1600","size":5,"soft_fail":false,"target_va":5376347648,"va":5374853176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*36431246(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374851744,"raw_hex":"ff158ee52b02","size":6,"soft_fail":false,"va":5374853308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":436,"record_offset":11716,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":436,"record_offset":12592,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":436,"record_offset":11744,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":436,"record_offset":11032,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":12420,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":12648,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":12116,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":436,"record_offset":12460,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":436,"record_offset":11772,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":436,"record_offset":11632,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":436,"record_offset":12216,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":13488,"source_line":579,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":9596,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":8008,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":8584,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::bytes_as_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":436,"record_offset":12560,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":25170,"source_line":29,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":436,"record_offset":12712,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":25174,"source_line":50,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":436,"record_offset":11800,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":10832,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_extension
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":11000,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":11091,"source_line":2607,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":10628,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":10692,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ustr-1.0.0/src/lib.rs
//     evidence={"module":436,"record_offset":12684,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":8417,"source_line":469,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":436,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":7776,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":436,"record_offset":11248,"parent_record_offset":11192,"ancestor_record_offset":7776,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":436,"record_offset":11220,"parent_record_offset":11192,"ancestor_record_offset":7776,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":436,"record_offset":11192,"parent_record_offset":11164,"ancestor_record_offset":7776,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":9980,"parent_record_offset":9952,"ancestor_record_offset":7776,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":7776,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":10008,"parent_record_offset":9980,"ancestor_record_offset":7776,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":7776,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":9952,"parent_record_offset":9920,"ancestor_record_offset":7776,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":10124,"parent_record_offset":9952,"ancestor_record_offset":7776,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":436,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":7776,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":436,"record_offset":11884,"parent_record_offset":11800,"ancestor_record_offset":7776,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$86::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":436,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":7776,"inlinee":25172,"source_line":248,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":436,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":7776,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":436,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":7776,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":436,"record_offset":8220,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":436,"record_offset":9816,"parent_record_offset":9788,"ancestor_record_offset":7776,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":436,"record_offset":9180,"parent_record_offset":8720,"ancestor_record_offset":7776,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":436,"record_offset":9864,"parent_record_offset":9692,"ancestor_record_offset":7776,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":436,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":7776,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":436,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":7776,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":436,"record_offset":10188,"parent_record_offset":9892,"ancestor_record_offset":7776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":436,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":7776,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":436,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":7776,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":436,"record_offset":9140,"parent_record_offset":9108,"ancestor_record_offset":7776,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":436,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":7776,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":10588,"parent_record_offset":9660,"ancestor_record_offset":7776,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":12380,"parent_record_offset":12240,"ancestor_record_offset":7776,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":436,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":7776,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":436,"record_offset":8156,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":436,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7776,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":436,"record_offset":12516,"parent_record_offset":12488,"ancestor_record_offset":7776,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":436,"record_offset":12488,"parent_record_offset":12460,"ancestor_record_offset":7776,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":436,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":7776,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":436,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":7776,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":436,"record_offset":8884,"parent_record_offset":8776,"ancestor_record_offset":7776,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":436,"record_offset":8124,"parent_record_offset":8096,"ancestor_record_offset":7776,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":436,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":7776,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":436,"record_offset":8748,"parent_record_offset":8720,"ancestor_record_offset":7776,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":436,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":7776,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":436,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":7776,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":436,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":7776,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":436,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":7776,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":436,"record_offset":9212,"parent_record_offset":8688,"ancestor_record_offset":7776,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8320,"parent_record_offset":8288,"ancestor_record_offset":7776,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8480,"parent_record_offset":8320,"ancestor_record_offset":7776,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7776,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7776,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":7776,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8096,"parent_record_offset":8064,"ancestor_record_offset":7776,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":436,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7776,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":436,"record_offset":8776,"parent_record_offset":8748,"ancestor_record_offset":7776,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":436,"record_offset":9024,"parent_record_offset":8776,"ancestor_record_offset":7776,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":436,"record_offset":8996,"parent_record_offset":8776,"ancestor_record_offset":7776,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":436,"record_offset":8828,"parent_record_offset":8776,"ancestor_record_offset":7776,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":436,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":7776,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":436,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":7776,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::load_texture::load_compiled_texture_image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/load_texture.rs
//     evidence={"module":436,"record_offset":12152,"parent_record_offset":12116,"ancestor_record_offset":7776,"inlinee":25184,"source_line":19,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":436,"record_offset":12268,"parent_record_offset":12240,"ancestor_record_offset":7776,"inlinee":11471,"source_line":228,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":436,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":7776,"inlinee":11475,"source_line":338,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":436,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":7776,"inlinee":11473,"source_line":229,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":9628,"parent_record_offset":9596,"ancestor_record_offset":7776,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":436,"record_offset":9728,"parent_record_offset":9692,"ancestor_record_offset":7776,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":9892,"parent_record_offset":9692,"ancestor_record_offset":7776,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":436,"record_offset":10160,"parent_record_offset":9892,"ancestor_record_offset":7776,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":436,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":7776,"inlinee":25179,"source_line":212,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":436,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":7776,"inlinee":25180,"source_line":200,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":436,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":7776,"inlinee":25177,"source_line":418,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":436,"record_offset":12844,"parent_record_offset":12812,"ancestor_record_offset":7776,"inlinee":25182,"source_line":110,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":436,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":7776,"inlinee":25175,"source_line":32,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":436,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":7776,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":436,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":7776,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":436,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":7776,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":436,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":7776,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":436,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":7776,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":436,"record_offset":12080,"parent_record_offset":11800,"ancestor_record_offset":7776,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":436,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":7776,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":7776,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":7776,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":436,"record_offset":11908,"parent_record_offset":11884,"ancestor_record_offset":7776,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":436,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":7776,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":436,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":7776,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":436,"record_offset":9788,"parent_record_offset":9756,"ancestor_record_offset":7776,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":436,"record_offset":9692,"parent_record_offset":9660,"ancestor_record_offset":7776,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":436,"record_offset":9660,"parent_record_offset":9628,"ancestor_record_offset":7776,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_compiled_texture_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6260
// 首条位置: {"module":436,"inlinee":25184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":436,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":12152,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00436.jsonl.gz"}
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
// 完整结构及原始类型记录: load_texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_compiled_texture_image
// 信息 ID: 800898335511a195ede6fb5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$2::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0xd75730 -> 0x5dbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<ref$<relative_path::RelativePathBuf> > >,country_core::startup::load_textures::load_textures::closure$1::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0xd77075 -> 0x5dbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<ref$<relative_path::RelativePathBuf> > >,country_core::startup::load_textures::load_textures::closure$1::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > @ 0xd7b660 -> 0x5dbaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$2::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > @ 0xd7fae4 -> 0x5dbaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_compiled_texture_image::closure$0
// 信息 ID: 3ade5ef852002770ad29e79e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::load_texture::load_compiled_texture_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::load_texture::load_compiled_texture_image; module=436, inline_record=12152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
