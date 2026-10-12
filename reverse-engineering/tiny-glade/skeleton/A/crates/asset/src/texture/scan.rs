// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset/src/texture/scan.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 66e00313197edc07b8fdffd8911fb1a6bff05cab7ea1806b0725bd87b22837f8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3912, 3977
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3912,"record_offset":2268,"rva":29918912,"va":5398628032}
// 编译实例 RVAs: 0x1c886c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26003071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628032,"raw_hex":"e8813973fe","size":5,"soft_fail":false,"target_va":5372625024,"va":5398628090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree_walker
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628032,"raw_hex":"e895000000","size":5,"soft_fail":false,"target_va":5398628336,"va":5398628182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t28508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628032,"raw_hex":"e85c6f0000","size":5,"soft_fail":false,"target_va":5398656736,"va":5398628223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21261849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628032,"raw_hex":"e8e791bbfe","size":5,"soft_fail":false,"target_va":5377366224,"va":5398628068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::texture::config::InheritableTextureOpts::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3912,"record_offset":2856,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":112155,"source_line":46,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":2912,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":2884,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":2404,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":2268,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2268,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":2692,"parent_record_offset":2640,"ancestor_record_offset":2268,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3912,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2268,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2268,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2268,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2268,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":2824,"parent_record_offset":2404,"ancestor_record_offset":2268,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2268,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3912,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2268,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":2640,"parent_record_offset":2500,"ancestor_record_offset":2268,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":2268,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3912,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":2268,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3912,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2268,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3912,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2268,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3912,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2268,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3912,"record_offset":3140,"rva":29919216,"va":5398628336}
// 编译实例 RVAs: 0x1c887f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2073096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e808a21f00","size":5,"soft_fail":false,"target_va":5400706688,"va":5398633587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$relative_path..RelativePathBuf$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18981412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8dc5ddefe","size":5,"soft_fail":false,"target_va":5379649904,"va":5398631311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21221251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e87d30bcfe","size":5,"soft_fail":false,"target_va":5377408080,"va":5398629326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21168013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e87300bdfe","size":5,"soft_fail":false,"target_va":5377465440,"va":5398633448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29016212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e86c3f45fe","size":5,"soft_fail":false,"target_va":5369614224,"va":5398630431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26973191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8f96b64fe","size":5,"soft_fail":false,"target_va":5371656704,"va":5398629890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26329435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8a53e6efe","size":5,"soft_fail":false,"target_va":5372309344,"va":5398638774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26314408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e858796efe","size":5,"soft_fail":false,"target_va":5372314208,"va":5398628611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26243306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8168f6ffe","size":5,"soft_fail":false,"target_va":5372390432,"va":5398633733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26003619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e85d3773fe","size":5,"soft_fail":false,"target_va":5372625024,"va":5398628638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=33
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25879849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8d71a75fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398629556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=9
//     target_aliases=28 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23457408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e880119afe","size":5,"soft_fail":false,"target_va":5375171056,"va":5398628459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21224633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e84723bcfe","size":5,"soft_fail":false,"target_va":5377405008,"va":5398629636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21020888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8283fbffe","size":5,"soft_fail":false,"target_va":5377611216,"va":5398632099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca05a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t94880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8a0720100","size":5,"soft_fail":false,"target_va":5398726048,"va":5398631163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca0840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t94558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e85e710100","size":5,"soft_fail":false,"target_va":5398726720,"va":5398632157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca13c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t98114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8427f0100","size":5,"soft_fail":false,"target_va":5398729664,"va":5398631545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca4a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t114430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8febe0100","size":5,"soft_fail":false,"target_va":5398743632,"va":5398629197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ca5190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t112286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e89eb60100","size":5,"soft_fail":false,"target_va":5398745488,"va":5398633197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4964181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e855bf4b00","size":5,"soft_fail":false,"target_va":5403593360,"va":5398629174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-21103826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e82efbbdfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5398633709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27326738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8ee065ffe","size":5,"soft_fail":false,"target_va":5371305776,"va":5398632509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4915098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e89aff4a00","size":5,"soft_fail":false,"target_va":5403549344,"va":5398634241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h1d09fb2f238b2066 (.llvm.2838924830685125660)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4919847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e827124b00","size":5,"soft_fail":false,"target_va":5403553984,"va":5398634132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4920098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e822134b00","size":5,"soft_fail":false,"target_va":5403553824,"va":5398633721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4926332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e87c2b4b00","size":5,"soft_fail":false,"target_va":5403555264,"va":5398628927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21271748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e83c6bbbfe","size":5,"soft_fail":false,"target_va":5377364336,"va":5398636079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::config::InheritableTextureOpts::into_concrete
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2074638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e80ea81f00","size":5,"soft_fail":false,"target_va":5400705664,"va":5398631021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree_walker
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8e0f8ffff","size":5,"soft_fail":false,"target_va":5398628336,"va":5398630155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4343338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e82a464200","size":5,"soft_fail":false,"target_va":5402978880,"va":5398635537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4343823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e80f484200","size":5,"soft_fail":false,"target_va":5402979344,"va":5398635516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$asset..texture..desc..TextureAssetDesc$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-14498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e85ec7ffff","size":5,"soft_fail":false,"target_va":5398620256,"va":5398634749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..texture..desc..TextureAssetDesc$GT$
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-12215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e849d0ffff","size":5,"soft_fail":false,"target_va":5398619376,"va":5398631586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4344541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8dd4a4200","size":5,"soft_fail":false,"target_va":5402980128,"va":5398635582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::remove_entry
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t109492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8b4ab0100","size":5,"soft_fail":false,"target_va":5398739968,"va":5398630471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t23631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e84f5c0000","size":5,"soft_fail":false,"target_va":5398656736,"va":5398633100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t113886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8debc0100","size":5,"soft_fail":false,"target_va":5398744000,"va":5398630109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21220762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e86632bcfe","size":5,"soft_fail":false,"target_va":5377408720,"va":5398629477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21220821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e82b32bcfe","size":5,"soft_fail":false,"target_va":5377408592,"va":5398629408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-21221353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e81730bcfe","size":5,"soft_fail":false,"target_va":5377408224,"va":5398629572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21223275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e89528bcfe","size":5,"soft_fail":false,"target_va":5377405616,"va":5398628886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21262397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8c38fbbfe","size":5,"soft_fail":false,"target_va":5377366224,"va":5398628616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21188680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8b8afbcfe","size":5,"soft_fail":false,"target_va":5377439776,"va":5398628451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21189947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8c5aabcfe","size":5,"soft_fail":false,"target_va":5377439664,"va":5398629606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21190785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e87fa7bcfe","size":5,"soft_fail":false,"target_va":5377439520,"va":5398630300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21160895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e8411cbdfe","size":5,"soft_fail":false,"target_va":5377468224,"va":5398629114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21155218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398628336,"raw_hex":"e86e32bdfe","size":5,"soft_fail":false,"target_va":5377473264,"va":5398628477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3912,"record_offset":16580,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3912,"record_offset":13000,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":12684,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":12768,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":14004,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3912,"record_offset":15252,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":10987,"source_line":58,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::InheritableTextureOpts::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3912,"record_offset":4204,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":112158,"source_line":33,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3912,"record_offset":3708,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":112157,"source_line":7,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::scan::load_texture_group_config
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3912,"record_offset":4780,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":112165,"source_line":156,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3912,"record_offset":16720,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3912,"record_offset":12916,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3912,"record_offset":16460,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":12902,"source_line":3464,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3912,"record_offset":16524,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":6732,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":13708,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":8264,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":10068,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":6320,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":5928,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":16664,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3912,"record_offset":8300,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":7369,"source_line":2217,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3912,"record_offset":6892,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":10960,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":11944,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":12712,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":16024,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":9740,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":10096,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":8204,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":8120,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":37781,"source_line":204,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":6144,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":112169,"source_line":657,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":6032,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":13851,"source_line":1193,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":5864,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3912,"record_offset":3576,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3912,"record_offset":3408,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3912,"record_offset":3436,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":5160,"parent_record_offset":5104,"ancestor_record_offset":3140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3912,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":3140,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3912,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":3140,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":3140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":12852,"parent_record_offset":12820,"ancestor_record_offset":3140,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":3140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3912,"record_offset":13052,"parent_record_offset":13000,"ancestor_record_offset":3140,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3912,"record_offset":7788,"parent_record_offset":7752,"ancestor_record_offset":3140,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3912,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":3140,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":12820,"parent_record_offset":12768,"ancestor_record_offset":3140,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3912,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":3140,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":3912,"record_offset":15428,"parent_record_offset":15404,"ancestor_record_offset":3140,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3912,"record_offset":15404,"parent_record_offset":15252,"ancestor_record_offset":3140,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3912,"record_offset":4268,"parent_record_offset":4204,"ancestor_record_offset":3140,"inlinee":112159,"source_line":30,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3912,"record_offset":4300,"parent_record_offset":4268,"ancestor_record_offset":3140,"inlinee":112161,"source_line":13,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":3140,"inlinee":112163,"source_line":4,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3912,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":3140,"inlinee":112167,"source_line":19,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3912,"record_offset":15292,"parent_record_offset":15252,"ancestor_record_offset":3140,"inlinee":112172,"source_line":34,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3912,"record_offset":13744,"parent_record_offset":13708,"ancestor_record_offset":3140,"inlinee":112173,"source_line":68,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3912,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":3140,"inlinee":112171,"source_line":106,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":3140,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3140,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3912,"record_offset":4064,"parent_record_offset":4008,"ancestor_record_offset":3140,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3912,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":3140,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3912,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":3140,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3912,"record_offset":16312,"parent_record_offset":16284,"ancestor_record_offset":3140,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3912,"record_offset":11220,"parent_record_offset":11196,"ancestor_record_offset":3140,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3912,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":3140,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3912,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":3140,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3912,"record_offset":13080,"parent_record_offset":13000,"ancestor_record_offset":3140,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3912,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":3140,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::Cloned::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":3912,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":3140,"inlinee":12904,"source_line":23,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3912,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":3140,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3912,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3140,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3912,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":3140,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3140,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":3140,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":6848,"parent_record_offset":6764,"ancestor_record_offset":3140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":3140,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3912,"record_offset":16256,"parent_record_offset":16224,"ancestor_record_offset":3140,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=169
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":3676,"parent_record_offset":3576,"ancestor_record_offset":3140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3912,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":3140,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3912,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":3140,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3912,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":3140,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":13108,"parent_record_offset":13000,"ancestor_record_offset":3140,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":3140,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":3140,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":4872,"parent_record_offset":4780,"ancestor_record_offset":3140,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":4908,"parent_record_offset":4780,"ancestor_record_offset":3140,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3912,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":3140,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3912,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":3140,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3912,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":3140,"inlinee":5503,"source_line":2554,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3912,"record_offset":14032,"parent_record_offset":14004,"ancestor_record_offset":3140,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3912,"record_offset":14060,"parent_record_offset":14032,"ancestor_record_offset":3140,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3912,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":3140,"inlinee":7373,"source_line":962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3912,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":3140,"inlinee":7372,"source_line":994,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3912,"record_offset":7848,"parent_record_offset":7788,"ancestor_record_offset":3140,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3912,"record_offset":7880,"parent_record_offset":7752,"ancestor_record_offset":3140,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3912,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":3140,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":3140,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":3140,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":3140,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":16080,"parent_record_offset":16052,"ancestor_record_offset":3140,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":3140,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":9796,"parent_record_offset":9764,"ancestor_record_offset":3140,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":11708,"parent_record_offset":11488,"ancestor_record_offset":3140,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3912,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":3140,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":3140,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11596,"parent_record_offset":11488,"ancestor_record_offset":3140,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11536,"parent_record_offset":11512,"ancestor_record_offset":3140,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11888,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":16224,"parent_record_offset":16136,"ancestor_record_offset":3140,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":3140,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11092,"parent_record_offset":11068,"ancestor_record_offset":3140,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11068,"parent_record_offset":11008,"ancestor_record_offset":3140,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":16108,"parent_record_offset":16080,"ancestor_record_offset":3140,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":16196,"parent_record_offset":16136,"ancestor_record_offset":3140,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":16168,"parent_record_offset":16136,"ancestor_record_offset":3140,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":3140,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":16136,"parent_record_offset":16108,"ancestor_record_offset":3140,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3912,"record_offset":11368,"parent_record_offset":11336,"ancestor_record_offset":3140,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3912,"record_offset":11396,"parent_record_offset":11368,"ancestor_record_offset":3140,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3912,"record_offset":11456,"parent_record_offset":11336,"ancestor_record_offset":3140,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3912,"record_offset":11336,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11168,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":3140,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3912,"record_offset":11488,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3912,"record_offset":11196,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3912,"record_offset":16284,"parent_record_offset":16224,"ancestor_record_offset":3140,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3912,"record_offset":11280,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3912,"record_offset":11796,"parent_record_offset":11116,"ancestor_record_offset":3140,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3912,"record_offset":16340,"parent_record_offset":16284,"ancestor_record_offset":3140,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":3912,"record_offset":4936,"parent_record_offset":4780,"ancestor_record_offset":3140,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":3912,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3140,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":16052,"parent_record_offset":16024,"ancestor_record_offset":3140,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":10556,"parent_record_offset":10096,"ancestor_record_offset":3140,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3912,"record_offset":3744,"parent_record_offset":3708,"ancestor_record_offset":3140,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":3140,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$40::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":6224,"parent_record_offset":6168,"ancestor_record_offset":3140,"inlinee":8925,"source_line":1348,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":3140,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3140,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":3140,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3140,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":3608,"parent_record_offset":3576,"ancestor_record_offset":3140,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":4840,"parent_record_offset":4780,"ancestor_record_offset":3140,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":3140,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3912,"record_offset":3888,"parent_record_offset":3856,"ancestor_record_offset":3140,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":4008,"parent_record_offset":3856,"ancestor_record_offset":3140,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3912,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3140,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3912,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":3140,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3912,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3140,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3912,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3140,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3912,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3140,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3912,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":3140,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3912,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":3140,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3912,"record_offset":3828,"parent_record_offset":3800,"ancestor_record_offset":3140,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B62C
// 首条位置: {"module":3912,"inlinee":112172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3912,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":15292,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3912,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":15292,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":15292,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B873
// 首条位置: {"module":3977,"inlinee":112755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3977,"record_offset":512,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03977.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3977,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":492,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03977.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3977,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":492,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03977.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B62D
// 首条位置: {"module":3912,"inlinee":112173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3912,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":13744,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B62B
// 首条位置: {"module":3912,"inlinee":112171}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3912,"record_offset":6848,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3912,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":6764,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":6764,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::load_texture_group_config
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B625
// 首条位置: {"module":3912,"inlinee":112165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":5032,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":4872,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3912,"record_offset":4908,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":3912,"record_offset":4936,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3912,"record_offset":4840,"parent_record_offset":4780,"ancestor_record_offset":4780,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4780,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3912,"record_offset":5160,"parent_record_offset":5104,"ancestor_record_offset":4780,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":4780,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3912,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4780,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3912,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":4780,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03912.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":3912,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4780,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/03912.jsonl.gz"}
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
// 完整结构及原始类型记录: scan.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: scan.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: scan.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::scan::load_texture_group_config
// 信息 ID: f227f7be196387e16ca951a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree
// 信息 ID: d8f6160a95d4fe5287c96474
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_texture_assets @ 0x18102a5 -> 0x1c886c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker
// 信息 ID: 6376bac29a0a5c1dafc6c0da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::scan::scan_asset_tree @ 0x1c88756 -> 0x1c887f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x1c88f0b -> 0x1c887f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x1c89535 -> 0x1c887f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$0
// 信息 ID: ac444fcff1e80d404433a7c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=15292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$1
// 信息 ID: 736dd2ac38bdf7c44e8eba7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// [闭包环境类型引用] asset::texture::scan::scan_asset_tree_walker::closure_env$1 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,asset::texture::scan::scan_asset_tree_walker::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,asset::texture::scan::scan_asset_tree_walker::closure_env$1>; module=3977, inline_record=492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$2
// 信息 ID: 60d7a9f25ddf6f71e90beb8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=13744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$3
// 信息 ID: 32ebad27abdddc8e4b332207
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=6764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
