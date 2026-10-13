// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/load_language_library.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 69b5814e721d13c9a2d6944b0ed06c8aebce24e7134f9c9886f59a6a20a10a69
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 77
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use steamworks; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":77,"record_offset":10700,"rva":984688,"va":5369693808}
// 编译实例 RVAs: 0xf0670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e8880cffff","size":5,"soft_fail":false,"target_va":5369632448,"va":5369694771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3055831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e8d7a02e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5369693860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3055715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e863a02e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369693992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3455929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e8b9bb3400","size":5,"soft_fail":false,"target_va":5373149840,"va":5369693906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144740
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33898593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e861400502","size":5,"soft_fail":false,"target_va":5403592512,"va":5369693914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33283159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e857dcfb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369694980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_lowercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7836571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e89b937700","size":5,"soft_fail":false,"target_va":5377530544,"va":5369693968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1963024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e810f41d00","size":5,"soft_fail":false,"target_va":5371657040,"va":5369694011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7668849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e871047500","size":5,"soft_fail":false,"target_va":5377364336,"va":5369695482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33285074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e8d2e3fb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5369695049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6454410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e88a7c6200","size":5,"soft_fail":false,"target_va":5376149312,"va":5369694897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7659128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e878de7400","size":5,"soft_fail":false,"target_va":5377353776,"va":5369694643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] steamworks::app::Apps$LT$Manager$GT$::current_game_language
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t543951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369693808,"raw_hex":"e8cf4c0800","size":5,"soft_fail":false,"target_va":5370237904,"va":5369693948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":77,"record_offset":10884,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":77,"record_offset":14660,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":77,"record_offset":14544,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":77,"record_offset":14572,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":77,"record_offset":11376,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":77,"record_offset":15188,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":77,"record_offset":14628,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":15456,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":77,"record_offset":15388,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":77,"record_offset":12760,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":77,"record_offset":14600,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":77,"record_offset":14460,"parent_record_offset":10700,"ancestor_record_offset":10700,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":10700,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":10700,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":10700,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":10700,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11712,"parent_record_offset":11656,"ancestor_record_offset":10700,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":10700,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11116,"parent_record_offset":11088,"ancestor_record_offset":10700,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11088,"parent_record_offset":11060,"ancestor_record_offset":10700,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11260,"parent_record_offset":11088,"ancestor_record_offset":10700,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":10700,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":77,"record_offset":11024,"parent_record_offset":10996,"ancestor_record_offset":10700,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":77,"record_offset":10996,"parent_record_offset":10968,"ancestor_record_offset":10700,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":77,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10700,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":77,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10700,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":77,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10700,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":77,"record_offset":14688,"parent_record_offset":14660,"ancestor_record_offset":10700,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11868,"parent_record_offset":11832,"ancestor_record_offset":10700,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":10700,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":10700,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":11060,"parent_record_offset":11024,"ancestor_record_offset":10700,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":77,"record_offset":12788,"parent_record_offset":12760,"ancestor_record_offset":10700,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":77,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":10700,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":77,"record_offset":11296,"parent_record_offset":11024,"ancestor_record_offset":10700,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":11584,"parent_record_offset":11428,"ancestor_record_offset":10700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":77,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":10700,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":10700,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":77,"record_offset":12816,"parent_record_offset":12788,"ancestor_record_offset":10700,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":77,"record_offset":12844,"parent_record_offset":12816,"ancestor_record_offset":10700,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11520,"parent_record_offset":11492,"ancestor_record_offset":10700,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":10700,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":10700,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":10700,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11928,"parent_record_offset":11868,"ancestor_record_offset":10700,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11960,"parent_record_offset":11832,"ancestor_record_offset":10700,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/lib.rs
//     evidence={"module":77,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":10700,"inlinee":8755,"source_line":355,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::load_language_library::load_language_library::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/load_language_library.rs
//     evidence={"module":77,"record_offset":11428,"parent_record_offset":11376,"ancestor_record_offset":10700,"inlinee":8754,"source_line":11,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2232
// 首条位置: {"module":77,"inlinee":8754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":11584,"parent_record_offset":11428,"ancestor_record_offset":11428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/lib.rs
//     evidence={"module":77,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":11428,"inlinee":8755,"source_line":355,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":11428,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":77,"record_offset":11712,"parent_record_offset":11656,"ancestor_record_offset":11428,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":11428,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":77,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":11428,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11868,"parent_record_offset":11832,"ancestor_record_offset":11428,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11492,"parent_record_offset":11460,"ancestor_record_offset":11428,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":77,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":11428,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":77,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":11428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":77,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":11428,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11520,"parent_record_offset":11492,"ancestor_record_offset":11428,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":11428,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11928,"parent_record_offset":11868,"ancestor_record_offset":11428,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":11960,"parent_record_offset":11832,"ancestor_record_offset":11428,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00077.jsonl.gz"}
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
// 完整结构及原始类型记录: load_language_library.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_language_library.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_language_library.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library
// 信息 ID: fa9cb480224df5cdfabba454
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39a17f -> 0xf0670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library::closure$0
// 信息 ID: d958688d04bcf77ea5597028
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::load_language_library::load_language_library", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::load_language_library::load_language_library; module=77, inline_record=11428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
