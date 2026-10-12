// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/gothic_dormer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2c9dc7a3527dc113e6d4883067d1fb23450aba44f19e39e449b2deba8797db52
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_dormer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1287
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_dormer::gothic_dormer_profile_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1287,"record_offset":15292,"rva":13688016,"va":5382397136}
// 编译实例 RVAs: 0xd0dcd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9647497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382397136,"raw_hex":"e877ca6cff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382397188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20580646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382397136,"raw_hex":"e826093a01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382397493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20581669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382397136,"raw_hex":"e8250d3a01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382397478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1287,"record_offset":15460,"parent_record_offset":15292,"ancestor_record_offset":15292,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1287,"record_offset":15912,"parent_record_offset":15292,"ancestor_record_offset":15292,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1287,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":15292,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1287,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":15292,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1287,"record_offset":15492,"parent_record_offset":15460,"ancestor_record_offset":15292,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1287,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":15292,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1287,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":15292,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1287,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":15292,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1287,"record_offset":3608,"rva":13681424,"va":5382390544}
// 编译实例 RVAs: 0xd0c310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12778860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e894023dff","size":5,"soft_fail":false,"target_va":5369613520,"va":5382392375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12579052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8140f40ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382392599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10744974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8720b5cff","size":5,"soft_fail":false,"target_va":5371651744,"va":5382396713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9244575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e861f072ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5382394410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8331578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8c6de80ff","size":5,"soft_fail":false,"target_va":5374061056,"va":5382392629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7837398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e82a6988ff","size":5,"soft_fail":false,"target_va":5374554704,"va":5382392097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7678476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8f4d58aff","size":5,"soft_fail":false,"target_va":5374712880,"va":5382391351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7675444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8cce18aff","size":5,"soft_fail":false,"target_va":5374716800,"va":5382392239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7182347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8f56792ff","size":5,"soft_fail":false,"target_va":5375209776,"va":5382392118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7178095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8917892ff","size":5,"soft_fail":false,"target_va":5375213232,"va":5382391322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5451323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8c5d1acff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382395286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5451263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e801d2acff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382395258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-483602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8ee9ef8ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5382393485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8ca9dffff","size":5,"soft_fail":false,"target_va":5382367008,"va":5382392145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10985560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e858a0a700","size":5,"soft_fail":false,"target_va":5393378400,"va":5382392835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20582346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8ca0f3a01","size":5,"soft_fail":false,"target_va":5402978688,"va":5382396337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20583054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e88e123a01","size":5,"soft_fail":false,"target_va":5402978912,"va":5382395853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20582850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8c2113a01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382396297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20582966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e836123a01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382395973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e868e6ffff","size":5,"soft_fail":false,"target_va":5382389184,"va":5382395731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20589011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8d3293a01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382391112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8cffaffff","size":5,"soft_fail":false,"target_va":5382389376,"va":5382390700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20589874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8322d3a01","size":5,"soft_fail":false,"target_va":5402980656,"va":5382390777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6628309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e82bdc9aff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382391856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8029483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8d57a85ff","size":5,"soft_fail":false,"target_va":5374362576,"va":5382392054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7821210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e866a888ff","size":5,"soft_fail":false,"target_va":5374571120,"va":5382392325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7179320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8c87392ff","size":5,"soft_fail":false,"target_va":5375212688,"va":5382392003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::gothic
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7574550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8ea6b8cff","size":5,"soft_fail":false,"target_va":5374821040,"va":5382395585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6955601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8afdd95ff","size":5,"soft_fail":false,"target_va":5375436032,"va":5382391628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6034371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e83deca3ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5382391278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6030485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e86bfba3ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5382391040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6034534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e89aeba3ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5382390673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-101257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e87774feff","size":5,"soft_fail":false,"target_va":5382289808,"va":5382391060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11096423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e86751a900","size":5,"soft_fail":false,"target_va":5393490672,"va":5382394244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11099201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8415ca900","size":5,"soft_fail":false,"target_va":5393491488,"va":5382392282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4442459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8a536bcff","size":5,"soft_fail":false,"target_va":5377952848,"va":5382395302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5448019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382390544,"raw_hex":"e8addeacff","size":5,"soft_fail":false,"target_va":5376946416,"va":5382394430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1287,"record_offset":10480,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1287,"record_offset":9248,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1287,"record_offset":9304,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1287,"record_offset":6592,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1287,"record_offset":6564,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":7120,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1287,"record_offset":15136,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":9084,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":10616,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":12080,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":6508,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1287,"record_offset":13896,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1287,"record_offset":14908,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1287,"record_offset":14728,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1287,"record_offset":14540,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":5460,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":3872,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":4448,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":8128,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":8488,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":9332,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":1287,"record_offset":13548,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1287,"record_offset":10452,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1287,"record_offset":13728,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1287,"record_offset":10424,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1287,"record_offset":14512,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1287,"record_offset":10396,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1287,"record_offset":10552,"parent_record_offset":10520,"ancestor_record_offset":3608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1287,"record_offset":11900,"parent_record_offset":10652,"ancestor_record_offset":3608,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1287,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":3608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1287,"record_offset":10520,"parent_record_offset":10480,"ancestor_record_offset":3608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1287,"record_offset":7384,"parent_record_offset":7240,"ancestor_record_offset":3608,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1287,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":3608,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1287,"record_offset":7240,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":8028,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":7660,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":3608,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1287,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":3608,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1287,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":3608,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":3608,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":3608,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":3608,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":3608,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7964,"parent_record_offset":7684,"ancestor_record_offset":3608,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":3608,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7632,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1287,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":3608,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1287,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":3608,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1287,"record_offset":7448,"parent_record_offset":7416,"ancestor_record_offset":3608,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":3608,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3608,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":3608,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":5832,"parent_record_offset":5800,"ancestor_record_offset":3608,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":6004,"parent_record_offset":5832,"ancestor_record_offset":3608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1287,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3608,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1287,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":3608,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1287,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":3608,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1287,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1287,"record_offset":6828,"parent_record_offset":6796,"ancestor_record_offset":3608,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1287,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":3608,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1287,"record_offset":6892,"parent_record_offset":6856,"ancestor_record_offset":3608,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1287,"record_offset":14336,"parent_record_offset":13920,"ancestor_record_offset":3608,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1287,"record_offset":14244,"parent_record_offset":14188,"ancestor_record_offset":3608,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1287,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":3608,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1287,"record_offset":14048,"parent_record_offset":13964,"ancestor_record_offset":3608,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1287,"record_offset":14108,"parent_record_offset":13920,"ancestor_record_offset":3608,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1287,"record_offset":13640,"parent_record_offset":13548,"ancestor_record_offset":3608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1287,"record_offset":9204,"parent_record_offset":9180,"ancestor_record_offset":3608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1287,"record_offset":9660,"parent_record_offset":9636,"ancestor_record_offset":3608,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1287,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1287,"record_offset":4084,"parent_record_offset":3988,"ancestor_record_offset":3608,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1287,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":3608,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1287,"record_offset":7328,"parent_record_offset":7240,"ancestor_record_offset":3608,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1287,"record_offset":14576,"parent_record_offset":14540,"ancestor_record_offset":3608,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1287,"record_offset":14940,"parent_record_offset":14908,"ancestor_record_offset":3608,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1287,"record_offset":14760,"parent_record_offset":14728,"ancestor_record_offset":3608,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1287,"record_offset":7084,"parent_record_offset":7056,"ancestor_record_offset":3608,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":3608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":13584,"parent_record_offset":13548,"ancestor_record_offset":3608,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":3608,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":5044,"parent_record_offset":4584,"ancestor_record_offset":3608,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":5744,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":3608,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1287,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":3608,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1287,"record_offset":6068,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1287,"record_offset":7556,"parent_record_offset":7524,"ancestor_record_offset":3608,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1287,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":3608,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1287,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":3608,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1287,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3608,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1287,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":3608,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1287,"record_offset":5004,"parent_record_offset":4972,"ancestor_record_offset":3608,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1287,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":3608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1287,"record_offset":6468,"parent_record_offset":5532,"ancestor_record_offset":3608,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1287,"record_offset":7604,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1287,"record_offset":4020,"parent_record_offset":3988,"ancestor_record_offset":3608,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1287,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3608,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1287,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":3608,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1287,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3608,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1287,"record_offset":4748,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1287,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":3608,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1287,"record_offset":4480,"parent_record_offset":4448,"ancestor_record_offset":3608,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1287,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":3608,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1287,"record_offset":4584,"parent_record_offset":4552,"ancestor_record_offset":3608,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1287,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3608,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1287,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":3608,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1287,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3608,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1287,"record_offset":5076,"parent_record_offset":4552,"ancestor_record_offset":3608,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":4184,"parent_record_offset":4152,"ancestor_record_offset":3608,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":4344,"parent_record_offset":4184,"ancestor_record_offset":3608,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3608,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":3904,"parent_record_offset":3872,"ancestor_record_offset":3608,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":3608,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":3960,"parent_record_offset":3928,"ancestor_record_offset":3608,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1287,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3608,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1287,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":3608,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1287,"record_offset":4888,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1287,"record_offset":4860,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1287,"record_offset":4692,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1287,"record_offset":11928,"parent_record_offset":11900,"ancestor_record_offset":3608,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1287,"record_offset":15196,"parent_record_offset":15160,"ancestor_record_offset":3608,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1287,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":3608,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1287,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":3608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1287,"record_offset":15252,"parent_record_offset":15160,"ancestor_record_offset":3608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":10652,"parent_record_offset":10616,"ancestor_record_offset":3608,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":9120,"parent_record_offset":9084,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":12120,"parent_record_offset":12080,"ancestor_record_offset":3608,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":11592,"parent_record_offset":11564,"ancestor_record_offset":3608,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":3608,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":9180,"parent_record_offset":9148,"ancestor_record_offset":3608,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1287,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":3608,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1287,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":3608,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7416,"parent_record_offset":7196,"ancestor_record_offset":3608,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":3608,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":3608,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7476,"parent_record_offset":7448,"ancestor_record_offset":3608,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1287,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":3608,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1287,"record_offset":14364,"parent_record_offset":13920,"ancestor_record_offset":3608,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1287,"record_offset":13992,"parent_record_offset":13964,"ancestor_record_offset":3608,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1287,"record_offset":13964,"parent_record_offset":13920,"ancestor_record_offset":3608,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1287,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":3608,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":9400,"parent_record_offset":9364,"ancestor_record_offset":3608,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":3608,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":3608,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":10128,"parent_record_offset":9904,"ancestor_record_offset":3608,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":3608,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1287,"record_offset":9456,"parent_record_offset":9424,"ancestor_record_offset":3608,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":10040,"parent_record_offset":10016,"ancestor_record_offset":3608,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":10016,"parent_record_offset":9904,"ancestor_record_offset":3608,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":3608,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":6928,"parent_record_offset":6692,"ancestor_record_offset":3608,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":10336,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":8284,"parent_record_offset":8224,"ancestor_record_offset":3608,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":6692,"parent_record_offset":6664,"ancestor_record_offset":3608,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":3608,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":3608,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9484,"parent_record_offset":9424,"ancestor_record_offset":3608,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":3608,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":3608,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":3608,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":3608,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1287,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":3608,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1287,"record_offset":8676,"parent_record_offset":8644,"ancestor_record_offset":3608,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1287,"record_offset":9872,"parent_record_offset":9752,"ancestor_record_offset":3608,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1287,"record_offset":6732,"parent_record_offset":6692,"ancestor_record_offset":3608,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9584,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":3608,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1287,"record_offset":9904,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1287,"record_offset":9612,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1287,"record_offset":6796,"parent_record_offset":6692,"ancestor_record_offset":3608,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1287,"record_offset":9696,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1287,"record_offset":10244,"parent_record_offset":9532,"ancestor_record_offset":3608,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1287,"record_offset":6856,"parent_record_offset":6796,"ancestor_record_offset":3608,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1287,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":3608,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":5496,"parent_record_offset":5460,"ancestor_record_offset":3608,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1287,"record_offset":5608,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":5772,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1287,"record_offset":6040,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":9364,"parent_record_offset":9332,"ancestor_record_offset":3608,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":3608,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1287,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1287,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":3608,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1287,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":3608,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1287,"record_offset":5568,"parent_record_offset":5532,"ancestor_record_offset":3608,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1287,"record_offset":5532,"parent_record_offset":5496,"ancestor_record_offset":3608,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_dormer.rs
//     evidence={"module":1287,"record_offset":7148,"parent_record_offset":7120,"ancestor_record_offset":3608,"inlinee":60284,"source_line":36,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB7C
// 首条位置: {"module":1287,"inlinee":60284}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1287,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":7148,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1287,"record_offset":7384,"parent_record_offset":7240,"ancestor_record_offset":7148,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1287,"record_offset":7240,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":8028,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":7660,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1287,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":7148,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1287,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":7148,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1287,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":7148,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":7148,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":7148,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":7148,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1287,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7148,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7964,"parent_record_offset":7684,"ancestor_record_offset":7148,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":7148,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1287,"record_offset":7632,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1287,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":7148,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1287,"record_offset":7448,"parent_record_offset":7416,"ancestor_record_offset":7148,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1287,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":7148,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1287,"record_offset":7328,"parent_record_offset":7240,"ancestor_record_offset":7148,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":7148,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1287,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":7148,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1287,"record_offset":7556,"parent_record_offset":7524,"ancestor_record_offset":7148,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1287,"record_offset":7604,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1287,"record_offset":7272,"parent_record_offset":7240,"ancestor_record_offset":7148,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1287,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":7148,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7416,"parent_record_offset":7196,"ancestor_record_offset":7148,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":7148,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":7148,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1287,"record_offset":7476,"parent_record_offset":7448,"ancestor_record_offset":7148,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01287.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1287,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":7148,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01287.jsonl.gz"}
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
// 完整结构及原始类型记录: gothic_dormer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gothic_dormer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gothic_dormer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers
// 信息 ID: 8f1d38e9ee275335247a694e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6e909eca477b9135f3588367: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17be1f6 -> 0xd0c310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1c821b8 -> 0xd0c310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers::closure$0
// 信息 ID: 23f9aefd1c4f22a1ea61bec6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=7148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_dormer::gothic_dormer_profile_curve
// 信息 ID: 68ff52843425fcc22e43a045
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x5026d8 -> 0xd0dcd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17a9eb8 -> 0xd0dcd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17aa4ab -> 0xd0dcd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
