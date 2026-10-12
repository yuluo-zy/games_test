// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/crash_pickup/crash_payload.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7519f9b0290e25d6dd70bfc80fe0a0e204f5cbc77c4612df6df6fb3b7d9ed99e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: crash_payload.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4, 10, 30, 151, 177, 200, 218, 234, 246, 256
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"PDB_registry_path_only","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":7932,"rva":3177104,"va":5371886224}
// 编译实例 RVAs: 0x307a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14035d620
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t350270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371886224,"raw_hex":"e83e580500","size":5,"soft_fail":false,"target_va":5372237344,"va":5371887069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t863382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371886224,"raw_hex":"e8962c0d00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371886309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31091204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371886224,"raw_hex":"e8046ada01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371886935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-3154722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371886224,"raw_hex":"e8dedccfff","size":5,"soft_fail":false,"target_va":5368732400,"va":5371887117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3155311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371886224,"raw_hex":"e891dacfff","size":5,"soft_fail":false,"target_va":5368731856,"va":5371887162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":200,"record_offset":8132,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":8076,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":7932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":8420,"parent_record_offset":8392,"ancestor_record_offset":7932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":8392,"parent_record_offset":8364,"ancestor_record_offset":7932,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":8364,"parent_record_offset":8336,"ancestor_record_offset":7932,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":8536,"parent_record_offset":8364,"ancestor_record_offset":7932,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":7932,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":8272,"parent_record_offset":8244,"ancestor_record_offset":7932,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":8216,"parent_record_offset":8188,"ancestor_record_offset":7932,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":7932,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":7932,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":200,"record_offset":8160,"parent_record_offset":8132,"ancestor_record_offset":7932,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7932,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":8336,"parent_record_offset":8300,"ancestor_record_offset":7932,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":200,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":7932,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":200,"record_offset":8572,"parent_record_offset":8300,"ancestor_record_offset":7932,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":7932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=19
// 源码行: 6, 26, 38, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1ACA
// 首条位置: {"module":30,"inlinee":6858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":91708,"parent_record_offset":91684,"ancestor_record_offset":91684,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":91732,"parent_record_offset":91708,"ancestor_record_offset":91684,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 17, 38, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D13
// 首条位置: {"module":151,"inlinee":11539}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":87144,"parent_record_offset":87116,"ancestor_record_offset":87116,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":87168,"parent_record_offset":87144,"ancestor_record_offset":87116,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":87192,"parent_record_offset":87168,"ancestor_record_offset":87116,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":87216,"parent_record_offset":87192,"ancestor_record_offset":87116,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 6, 17, 26, 38, 44, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D10
// 首条位置: {"module":151,"inlinee":11536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":130760,"parent_record_offset":130732,"ancestor_record_offset":130732,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85324,"parent_record_offset":85300,"ancestor_record_offset":85300,"inlinee":11525,"source_line":1886,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":85620,"parent_record_offset":85348,"ancestor_record_offset":85300,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":87168,"parent_record_offset":87144,"ancestor_record_offset":86768,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":87192,"parent_record_offset":87168,"ancestor_record_offset":86768,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":87216,"parent_record_offset":87192,"ancestor_record_offset":86768,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":87144,"parent_record_offset":87116,"ancestor_record_offset":86768,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85504,"parent_record_offset":85408,"ancestor_record_offset":85300,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85648,"parent_record_offset":85348,"ancestor_record_offset":85300,"inlinee":11399,"source_line":367,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85408,"parent_record_offset":85348,"ancestor_record_offset":85300,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85436,"parent_record_offset":85408,"ancestor_record_offset":85300,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130788,"parent_record_offset":130760,"ancestor_record_offset":130732,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":85348,"parent_record_offset":85324,"ancestor_record_offset":85300,"inlinee":11526,"source_line":1511,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":85528,"parent_record_offset":85504,"ancestor_record_offset":85300,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":85464,"parent_record_offset":85436,"ancestor_record_offset":85300,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":87116,"parent_record_offset":86816,"ancestor_record_offset":86768,"inlinee":11539,"source_line":6,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 38, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FC7
// 首条位置: {"module":177,"inlinee":12231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 38, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D3F
// 首条位置: {"module":151,"inlinee":11583}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":151,"record_offset":131196,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":132120,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":132056,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":132680,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":130176,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":131224,"parent_record_offset":130120,"ancestor_record_offset":130120,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":132360,"parent_record_offset":132336,"ancestor_record_offset":130120,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":132336,"parent_record_offset":132312,"ancestor_record_offset":130120,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":163908,"parent_record_offset":163884,"ancestor_record_offset":159196,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":132312,"parent_record_offset":132288,"ancestor_record_offset":130120,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":130884,"parent_record_offset":130788,"ancestor_record_offset":130120,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":165068,"parent_record_offset":165040,"ancestor_record_offset":159196,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":151,"record_offset":200444,"parent_record_offset":200420,"ancestor_record_offset":198972,"inlinee":11510,"source_line":25,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":130964,"parent_record_offset":130940,"ancestor_record_offset":130120,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":151,"record_offset":165212,"parent_record_offset":165184,"ancestor_record_offset":159196,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=73
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":132144,"parent_record_offset":132120,"ancestor_record_offset":130120,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":160044,"parent_record_offset":159352,"ancestor_record_offset":159196,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":130988,"parent_record_offset":130964,"ancestor_record_offset":130120,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":131012,"parent_record_offset":130988,"ancestor_record_offset":130120,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":130940,"parent_record_offset":130912,"ancestor_record_offset":130120,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":151,"record_offset":195072,"parent_record_offset":195048,"ancestor_record_offset":193596,"inlinee":6855,"source_line":188,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":132752,"parent_record_offset":132728,"ancestor_record_offset":130120,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":165656,"parent_record_offset":165632,"ancestor_record_offset":159196,"inlinee":6853,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":165544,"parent_record_offset":165520,"ancestor_record_offset":159196,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":132728,"parent_record_offset":132704,"ancestor_record_offset":130120,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":195988,"parent_record_offset":195964,"ancestor_record_offset":193596,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":130704,"parent_record_offset":130224,"ancestor_record_offset":130120,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":160492,"parent_record_offset":160468,"ancestor_record_offset":159196,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$28::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":131600,"parent_record_offset":131576,"ancestor_record_offset":130120,"inlinee":6867,"source_line":1874,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$42::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":165632,"parent_record_offset":165608,"ancestor_record_offset":159196,"inlinee":6852,"source_line":2990,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":200888,"parent_record_offset":200864,"ancestor_record_offset":198972,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130428,"parent_record_offset":130312,"ancestor_record_offset":130120,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":131308,"parent_record_offset":131264,"ancestor_record_offset":130120,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130312,"parent_record_offset":130224,"ancestor_record_offset":130120,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130352,"parent_record_offset":130312,"ancestor_record_offset":130120,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130224,"parent_record_offset":130176,"ancestor_record_offset":130120,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":131264,"parent_record_offset":131224,"ancestor_record_offset":130120,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":130788,"parent_record_offset":130760,"ancestor_record_offset":130120,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":130760,"parent_record_offset":130732,"ancestor_record_offset":130120,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":130452,"parent_record_offset":130428,"ancestor_record_offset":130120,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":130384,"parent_record_offset":130352,"ancestor_record_offset":130120,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":130912,"parent_record_offset":130788,"ancestor_record_offset":130120,"inlinee":11539,"source_line":6,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":130732,"parent_record_offset":130704,"ancestor_record_offset":130120,"inlinee":11536,"source_line":6,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 38, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D40
// 首条位置: {"module":151,"inlinee":11584}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":134028,"parent_record_offset":133412,"ancestor_record_offset":133412,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":133456,"parent_record_offset":133412,"ancestor_record_offset":133412,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":133492,"parent_record_offset":133412,"ancestor_record_offset":133412,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":134252,"parent_record_offset":134228,"ancestor_record_offset":133412,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":134228,"parent_record_offset":134172,"ancestor_record_offset":133412,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":134200,"parent_record_offset":134172,"ancestor_record_offset":133412,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":134172,"parent_record_offset":134148,"ancestor_record_offset":133412,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":169080,"parent_record_offset":169052,"ancestor_record_offset":167076,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":151,"record_offset":202076,"parent_record_offset":202052,"ancestor_record_offset":201776,"inlinee":11510,"source_line":25,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":151,"record_offset":169224,"parent_record_offset":169196,"ancestor_record_offset":167076,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":134052,"parent_record_offset":134028,"ancestor_record_offset":133412,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":151,"record_offset":196784,"parent_record_offset":196760,"ancestor_record_offset":196476,"inlinee":6855,"source_line":188,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":133992,"parent_record_offset":133532,"ancestor_record_offset":133412,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":167424,"parent_record_offset":167400,"ancestor_record_offset":167076,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":133712,"parent_record_offset":133604,"ancestor_record_offset":133412,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":133604,"parent_record_offset":133532,"ancestor_record_offset":133412,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":133640,"parent_record_offset":133604,"ancestor_record_offset":133412,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":133532,"parent_record_offset":133492,"ancestor_record_offset":133412,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":133736,"parent_record_offset":133712,"ancestor_record_offset":133412,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":133668,"parent_record_offset":133640,"ancestor_record_offset":133412,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=6
// 源码行: 6, 17, 26, 38, 44, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":5292,"rva":3002096,"va":5371711216}
// 编译实例 RVAs: 0x2dcef0, 0x2dcfd0, 0x2dd440, 0x2dd8b0, 0x2dd990, 0x2ddab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2087667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371712576,"raw_hex":"e80d25e0ff","size":5,"soft_fail":false,"target_va":5369625520,"va":5371713182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401321c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1751115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371713936,"raw_hex":"e8b547e5ff","size":5,"soft_fail":false,"target_va":5369962944,"va":5371714054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140263510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-499926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371712576,"raw_hex":"e82a5ff8ff","size":5,"soft_fail":false,"target_va":5371213072,"va":5371712993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402636e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-499243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371712576,"raw_hex":"e8d561f8ff","size":5,"soft_fail":false,"target_va":5371213536,"va":5371712774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140271110
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-442102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711440,"raw_hex":"e80a41f9ff","size":5,"soft_fail":false,"target_va":5371269392,"va":5371711489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140272010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-441805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e83342f9ff","size":5,"soft_fail":false,"target_va":5371273232,"va":5371715032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140272480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-438287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371712576,"raw_hex":"e8f14ff9ff","size":5,"soft_fail":false,"target_va":5371274368,"va":5371712650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140272ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-438883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e89d4df9ff","size":5,"soft_fail":false,"target_va":5371276192,"va":5371715070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402730f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-433776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711216,"raw_hex":"e89061f9ff","size":5,"soft_fail":false,"target_va":5371277552,"va":5371711323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140273ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-431213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711216,"raw_hex":"e8936bf9ff","size":5,"soft_fail":false,"target_va":5371280080,"va":5371711288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140274940
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-429261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371712576,"raw_hex":"e83373f9ff","size":5,"soft_fail":false,"target_va":5371283776,"va":5371713032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402754b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-427120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371713712,"raw_hex":"e8907bf9ff","size":5,"soft_fail":false,"target_va":5371286704,"va":5371713819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140276240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-423613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371713712,"raw_hex":"e84389f9ff","size":5,"soft_fail":false,"target_va":5371290176,"va":5371713784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402dcef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e85cf3ffff","size":5,"soft_fail":false,"target_va":5371711216,"va":5371714447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402dd440
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e841f8ffff","size":5,"soft_fail":false,"target_va":5371712576,"va":5371714554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=38
//     evidence={"assembly":"\tcallq\t31207670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711216,"raw_hex":"e8f630dc01","size":5,"soft_fail":false,"target_va":5402919072,"va":5371711397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_types::protocol::v7::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$sentry_types..protocol..v7..Context$GT$::serialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-404552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e8b8d3f9ff","size":5,"soft_fail":false,"target_va":5371310304,"va":5371714851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31244478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371714224,"raw_hex":"e8bec0dc01","size":5,"soft_fail":false,"target_va":5402960048,"va":5371715565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tjmp\t31248359","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371711440,"raw_hex":"e9e7cfdc01","size":5,"soft_fail":false,"target_va":5402960048,"va":5371711684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":25632,"parent_record_offset":18472,"ancestor_record_offset":18472,"inlinee":12238,"source_line":808,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":20068,"parent_record_offset":18472,"ancestor_record_offset":18472,"inlinee":12237,"source_line":794,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6892,"parent_record_offset":6528,"ancestor_record_offset":6528,"inlinee":8770,"source_line":268,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":5576,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":5677,"source_line":409,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":18760,"parent_record_offset":18472,"ancestor_record_offset":18472,"inlinee":8773,"source_line":420,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6808,"parent_record_offset":6528,"ancestor_record_offset":6528,"inlinee":8725,"source_line":249,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6064,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":5699,"source_line":774,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6008,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":177,"record_offset":5848,"parent_record_offset":5816,"ancestor_record_offset":5292,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":177,"record_offset":5916,"parent_record_offset":5884,"ancestor_record_offset":5292,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":177,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":5292,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":5292,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":5884,"parent_record_offset":5752,"ancestor_record_offset":5292,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":5292,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":5948,"parent_record_offset":5752,"ancestor_record_offset":5292,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":5788,"parent_record_offset":5752,"ancestor_record_offset":5292,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":177,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":5292,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":177,"record_offset":26340,"parent_record_offset":26316,"ancestor_record_offset":18472,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":177,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":6528,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::SerializeMap::serialize_entry
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":177,"record_offset":10872,"parent_record_offset":10840,"ancestor_record_offset":10076,"inlinee":5684,"source_line":1804,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$0::serialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":177,"record_offset":11372,"parent_record_offset":10912,"ancestor_record_offset":10076,"inlinee":5689,"source_line":42,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":177,"record_offset":7776,"parent_record_offset":6892,"ancestor_record_offset":6528,"inlinee":5012,"source_line":54,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":5292,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_key
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":10980,"parent_record_offset":10912,"ancestor_record_offset":10076,"inlinee":5687,"source_line":1868,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":7388,"parent_record_offset":6892,"ancestor_record_offset":6528,"inlinee":5694,"source_line":1894,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":5292,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$11::serialize_str
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":10076,"inlinee":5691,"source_line":857,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$19::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":10956,"parent_record_offset":10912,"ancestor_record_offset":10076,"inlinee":5696,"source_line":503,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":5604,"parent_record_offset":5576,"ancestor_record_offset":5292,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":6528,"inlinee":5692,"source_line":219,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":6096,"parent_record_offset":6064,"ancestor_record_offset":5292,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_key
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":10912,"parent_record_offset":10872,"ancestor_record_offset":10076,"inlinee":5686,"source_line":667,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::serialize_value
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":11460,"parent_record_offset":10872,"ancestor_record_offset":10076,"inlinee":5693,"source_line":695,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":177,"record_offset":20096,"parent_record_offset":20068,"ancestor_record_offset":18472,"inlinee":5682,"source_line":746,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":177,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":5292,"inlinee":4269,"source_line":417,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":177,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":5292,"inlinee":4267,"source_line":77,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 17, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FCB
// 首条位置: {"module":177,"inlinee":12235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":177,"record_offset":9416,"parent_record_offset":8936,"ancestor_record_offset":8936,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":8816,"parent_record_offset":8732,"ancestor_record_offset":8732,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":177,"record_offset":30176,"parent_record_offset":30144,"ancestor_record_offset":29656,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":8936,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":177,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8936,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9176,"parent_record_offset":9072,"ancestor_record_offset":8936,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9032,"parent_record_offset":9000,"ancestor_record_offset":8936,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9072,"parent_record_offset":9032,"ancestor_record_offset":8936,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9104,"parent_record_offset":9072,"ancestor_record_offset":8936,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9000,"parent_record_offset":8968,"ancestor_record_offset":8936,"inlinee":9488,"source_line":2020,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8936,"inlinee":11634,"source_line":2037,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8732,"inlinee":9486,"source_line":2073,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":8732,"inlinee":11637,"source_line":2089,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":177,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":8936,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":177,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":8936,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::CrashRecovered::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":177,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":29656,"inlinee":12240,"source_line":71,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":177,"record_offset":30104,"parent_record_offset":30080,"ancestor_record_offset":29656,"inlinee":6858,"source_line":26,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 17, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":8256,"rva":3002640,"va":5371711760}
// 编译实例 RVAs: 0x2dd110, 0x2de1e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31247954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e852cedc01","size":5,"soft_fail":false,"target_va":5402960384,"va":5371712425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029ac70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-275991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e8e9c9fbff","size":5,"soft_fail":false,"target_va":5371440240,"va":5371716226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029af10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-270990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e872ddfbff","size":5,"soft_fail":false,"target_va":5371440912,"va":5371711897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029b1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-270622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e8e2defbff","size":5,"soft_fail":false,"target_va":5371441568,"va":5371712185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-274645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e82bcffbff","size":5,"soft_fail":false,"target_va":5371441856,"va":5371716496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029c280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-270750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e862defbff","size":5,"soft_fail":false,"target_va":5371445888,"va":5371716633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-268336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e8d0e7fbff","size":5,"soft_fail":false,"target_va":5371448272,"va":5371716603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14029dc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-264124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e844f8fbff","size":5,"soft_fail":false,"target_va":5371452544,"va":5371716663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402a0a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-247841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e8df37fcff","size":5,"soft_fail":false,"target_va":5371464304,"va":5371712140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402a5b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-231803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e88576fcff","size":5,"soft_fail":false,"target_va":5371485072,"va":5371716870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1037340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e81cd40f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371712367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a0d00
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31210355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e8733bdc01","size":5,"soft_fail":false,"target_va":5402922240,"va":5371711880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a16d0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31212433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371711760,"raw_hex":"e89143dc01","size":5,"soft_fail":false,"target_va":5402924752,"va":5371712314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31876655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e82f66e601","size":5,"soft_fail":false,"target_va":5403593360,"va":5371716700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371716064,"raw_hex":"e83be9ffff","size":5,"soft_fail":false,"target_va":5371710992,"va":5371716816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8476,"parent_record_offset":8256,"ancestor_record_offset":8256,"inlinee":9479,"source_line":1855,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":8256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":8256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":177,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":8256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":177,"record_offset":30176,"parent_record_offset":30144,"ancestor_record_offset":28952,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":9620,"parent_record_offset":8476,"ancestor_record_offset":8256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":177,"record_offset":9416,"parent_record_offset":8936,"ancestor_record_offset":8256,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":8256,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":8816,"parent_record_offset":8732,"ancestor_record_offset":8256,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":177,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":8256,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":177,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8256,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8672,"parent_record_offset":8572,"ancestor_record_offset":8256,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9032,"parent_record_offset":9000,"ancestor_record_offset":8256,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8572,"parent_record_offset":8476,"ancestor_record_offset":8256,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8604,"parent_record_offset":8572,"ancestor_record_offset":8256,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9000,"parent_record_offset":8968,"ancestor_record_offset":8256,"inlinee":9488,"source_line":2020,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8256,"inlinee":11634,"source_line":2037,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8256,"inlinee":9486,"source_line":2073,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":177,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":8256,"inlinee":11637,"source_line":2089,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":177,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8256,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":177,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":8256,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::CrashRecovered::constructor$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":177,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":28952,"inlinee":12240,"source_line":71,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":177,"record_offset":30104,"parent_record_offset":30080,"ancestor_record_offset":28952,"inlinee":6858,"source_line":26,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":177,"record_offset":8732,"parent_record_offset":8476,"ancestor_record_offset":8256,"inlinee":12235,"source_line":65,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":17052,"rva":3182496,"va":5371891616}
// 编译实例 RVAs: 0x308fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5688771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371891616,"raw_hex":"e8c3cd5600","size":5,"soft_fail":false,"target_va":5377580528,"va":5371891752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5692257","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371891616,"raw_hex":"e961db5600","size":5,"soft_fail":false,"target_va":5377583920,"va":5371891658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":200,"record_offset":17188,"parent_record_offset":17052,"ancestor_record_offset":17052,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1297
// 首条位置: {"module":4,"inlinee":4759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 26, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":14264,"rva":3004256,"va":5371713376}
// 编译实例 RVAs: 0x2dd760, 0x2de620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14209c620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31186675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371717152,"raw_hex":"e8f3dedb01","size":5,"soft_fail":false,"target_va":5402904096,"va":5371717416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31879709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371713376,"raw_hex":"e81d72e601","size":5,"soft_fail":false,"target_va":5403593344,"va":5371713630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":177,"record_offset":14456,"parent_record_offset":14264,"ancestor_record_offset":14264,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":14264,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":177,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14264,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":177,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":14264,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1295
// 首条位置: {"module":4,"inlinee":4757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4,"record_offset":12228,"parent_record_offset":12064,"ancestor_record_offset":12064,"inlinee":4720,"source_line":1680,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":12064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":12064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":12064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":12596,"parent_record_offset":12560,"ancestor_record_offset":12064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":15260,"parent_record_offset":15204,"ancestor_record_offset":12064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":15232,"parent_record_offset":15204,"ancestor_record_offset":12064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":12560,"parent_record_offset":12532,"ancestor_record_offset":12064,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":12532,"parent_record_offset":12504,"ancestor_record_offset":12064,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":12716,"parent_record_offset":12532,"ancestor_record_offset":12064,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":12064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":12064,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":12440,"parent_record_offset":12412,"ancestor_record_offset":12064,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":12412,"parent_record_offset":12384,"ancestor_record_offset":12064,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":12504,"parent_record_offset":12468,"ancestor_record_offset":12064,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":12384,"parent_record_offset":12352,"ancestor_record_offset":12064,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":4,"record_offset":12828,"parent_record_offset":12352,"ancestor_record_offset":12064,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":12064,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":14356,"parent_record_offset":13612,"ancestor_record_offset":12064,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4,"record_offset":12756,"parent_record_offset":12468,"ancestor_record_offset":12064,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":15060,"parent_record_offset":13612,"ancestor_record_offset":12064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":12672,"parent_record_offset":12644,"ancestor_record_offset":12064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":4,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":12064,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":12296,"parent_record_offset":12268,"ancestor_record_offset":12064,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":12268,"parent_record_offset":12228,"ancestor_record_offset":12064,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4,"record_offset":12324,"parent_record_offset":12296,"ancestor_record_offset":12064,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":4,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":12064,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":13576,"parent_record_offset":13524,"ancestor_record_offset":12064,"inlinee":4759,"source_line":17,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":13612,"parent_record_offset":13524,"ancestor_record_offset":12064,"inlinee":4761,"source_line":38,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":14388,"parent_record_offset":14356,"ancestor_record_offset":12064,"inlinee":4763,"source_line":44,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1299
// 首条位置: {"module":4,"inlinee":4761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4,"record_offset":14356,"parent_record_offset":13612,"ancestor_record_offset":13612,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":15060,"parent_record_offset":13612,"ancestor_record_offset":13612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":13668,"parent_record_offset":13612,"ancestor_record_offset":13612,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":13612,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14128,"parent_record_offset":14104,"ancestor_record_offset":13612,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":13612,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14080,"parent_record_offset":14036,"ancestor_record_offset":13612,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":15260,"parent_record_offset":15204,"ancestor_record_offset":13612,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":15232,"parent_record_offset":15204,"ancestor_record_offset":13612,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14036,"parent_record_offset":14000,"ancestor_record_offset":13612,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14000,"parent_record_offset":13964,"ancestor_record_offset":13612,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14200,"parent_record_offset":14000,"ancestor_record_offset":13612,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":13612,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":13924,"parent_record_offset":13888,"ancestor_record_offset":13612,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":13888,"parent_record_offset":13852,"ancestor_record_offset":13612,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":13852,"parent_record_offset":13816,"ancestor_record_offset":13612,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":13964,"parent_record_offset":13924,"ancestor_record_offset":13612,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":13816,"parent_record_offset":13776,"ancestor_record_offset":13612,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":4,"record_offset":14312,"parent_record_offset":13776,"ancestor_record_offset":13612,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":13612,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4,"record_offset":14240,"parent_record_offset":13924,"ancestor_record_offset":13612,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":13612,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":14156,"parent_record_offset":14128,"ancestor_record_offset":13612,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":4,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":13612,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":13704,"parent_record_offset":13668,"ancestor_record_offset":13612,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":14448,"parent_record_offset":14388,"ancestor_record_offset":13612,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4,"record_offset":13740,"parent_record_offset":13704,"ancestor_record_offset":13612,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":4,"record_offset":13776,"parent_record_offset":13740,"ancestor_record_offset":13612,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":4,"record_offset":14388,"parent_record_offset":14356,"ancestor_record_offset":13612,"inlinee":4763,"source_line":44,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x129B
// 首条位置: {"module":4,"inlinee":4763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":4,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":14388,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4,"record_offset":14448,"parent_record_offset":14388,"ancestor_record_offset":14388,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14388,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14824,"parent_record_offset":14800,"ancestor_record_offset":14388,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4,"record_offset":14776,"parent_record_offset":14740,"ancestor_record_offset":14388,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14740,"parent_record_offset":14712,"ancestor_record_offset":14388,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14712,"parent_record_offset":14684,"ancestor_record_offset":14388,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4,"record_offset":14896,"parent_record_offset":14712,"ancestor_record_offset":14388,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":14648,"parent_record_offset":14620,"ancestor_record_offset":14388,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":14620,"parent_record_offset":14592,"ancestor_record_offset":14388,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":14388,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":14684,"parent_record_offset":14648,"ancestor_record_offset":14388,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4,"record_offset":14564,"parent_record_offset":14532,"ancestor_record_offset":14388,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":4,"record_offset":15008,"parent_record_offset":14532,"ancestor_record_offset":14388,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":14388,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4,"record_offset":14936,"parent_record_offset":14648,"ancestor_record_offset":14388,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4,"record_offset":14852,"parent_record_offset":14824,"ancestor_record_offset":14388,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4,"record_offset":14476,"parent_record_offset":14448,"ancestor_record_offset":14388,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4,"record_offset":14504,"parent_record_offset":14476,"ancestor_record_offset":14388,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":4,"record_offset":14532,"parent_record_offset":14504,"ancestor_record_offset":14388,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FC6
// 首条位置: {"module":177,"inlinee":12230}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D1D
// 首条位置: {"module":151,"inlinee":11549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":90164,"parent_record_offset":90136,"ancestor_record_offset":90136,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":90188,"parent_record_offset":90164,"ancestor_record_offset":90136,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90212,"parent_record_offset":90188,"ancestor_record_offset":90136,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90236,"parent_record_offset":90212,"ancestor_record_offset":90136,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D29
// 首条位置: {"module":151,"inlinee":11561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":99544,"parent_record_offset":99516,"ancestor_record_offset":99516,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":99668,"parent_record_offset":99572,"ancestor_record_offset":99516,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":99748,"parent_record_offset":99724,"ancestor_record_offset":99516,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99772,"parent_record_offset":99748,"ancestor_record_offset":99516,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99796,"parent_record_offset":99772,"ancestor_record_offset":99516,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":99724,"parent_record_offset":99696,"ancestor_record_offset":99516,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99612,"parent_record_offset":99572,"ancestor_record_offset":99516,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99572,"parent_record_offset":99544,"ancestor_record_offset":99516,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":99636,"parent_record_offset":99612,"ancestor_record_offset":99516,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":99696,"parent_record_offset":99572,"ancestor_record_offset":99516,"inlinee":11563,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FC9
// 首条位置: {"module":177,"inlinee":12233}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D27
// 首条位置: {"module":151,"inlinee":11559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":151,"record_offset":99980,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":100904,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":100840,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":101072,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":98948,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":100008,"parent_record_offset":98884,"ancestor_record_offset":98884,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":101024,"parent_record_offset":101000,"ancestor_record_offset":98884,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":101000,"parent_record_offset":100976,"ancestor_record_offset":98884,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":101348,"parent_record_offset":101320,"ancestor_record_offset":98884,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":100976,"parent_record_offset":100952,"ancestor_record_offset":98884,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":99668,"parent_record_offset":99572,"ancestor_record_offset":98884,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":99748,"parent_record_offset":99724,"ancestor_record_offset":98884,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":100928,"parent_record_offset":100904,"ancestor_record_offset":98884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99772,"parent_record_offset":99748,"ancestor_record_offset":98884,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99796,"parent_record_offset":99772,"ancestor_record_offset":98884,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":99724,"parent_record_offset":99696,"ancestor_record_offset":98884,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":101144,"parent_record_offset":101120,"ancestor_record_offset":98884,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":101120,"parent_record_offset":101096,"ancestor_record_offset":98884,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":99488,"parent_record_offset":98996,"ancestor_record_offset":98884,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":100384,"parent_record_offset":100360,"ancestor_record_offset":98884,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99208,"parent_record_offset":99092,"ancestor_record_offset":98884,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":100092,"parent_record_offset":100048,"ancestor_record_offset":98884,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99092,"parent_record_offset":98996,"ancestor_record_offset":98884,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99132,"parent_record_offset":99092,"ancestor_record_offset":98884,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":98996,"parent_record_offset":98948,"ancestor_record_offset":98884,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":100048,"parent_record_offset":100008,"ancestor_record_offset":98884,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":99572,"parent_record_offset":99544,"ancestor_record_offset":98884,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":99544,"parent_record_offset":99516,"ancestor_record_offset":98884,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":99232,"parent_record_offset":99208,"ancestor_record_offset":98884,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":99164,"parent_record_offset":99132,"ancestor_record_offset":98884,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":99516,"parent_record_offset":99488,"ancestor_record_offset":98884,"inlinee":11561,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":99696,"parent_record_offset":99572,"ancestor_record_offset":98884,"inlinee":11563,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D2C
// 首条位置: {"module":151,"inlinee":11564}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":102552,"parent_record_offset":101908,"ancestor_record_offset":101908,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":101984,"parent_record_offset":101908,"ancestor_record_offset":101908,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":101956,"parent_record_offset":101908,"ancestor_record_offset":101908,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":102728,"parent_record_offset":102704,"ancestor_record_offset":101908,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":102704,"parent_record_offset":102636,"ancestor_record_offset":101908,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":102672,"parent_record_offset":102636,"ancestor_record_offset":101908,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":102636,"parent_record_offset":102608,"ancestor_record_offset":101908,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":102580,"parent_record_offset":102552,"ancestor_record_offset":101908,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":102516,"parent_record_offset":102052,"ancestor_record_offset":101908,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":102232,"parent_record_offset":102124,"ancestor_record_offset":101908,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":102124,"parent_record_offset":102052,"ancestor_record_offset":101908,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":102160,"parent_record_offset":102124,"ancestor_record_offset":101908,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":102052,"parent_record_offset":102012,"ancestor_record_offset":101908,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":102256,"parent_record_offset":102232,"ancestor_record_offset":101908,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":102188,"parent_record_offset":102160,"ancestor_record_offset":101908,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D1A
// 首条位置: {"module":151,"inlinee":11546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":89988,"parent_record_offset":89960,"ancestor_record_offset":89960,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":90108,"parent_record_offset":90016,"ancestor_record_offset":89960,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":90188,"parent_record_offset":90164,"ancestor_record_offset":89960,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90212,"parent_record_offset":90188,"ancestor_record_offset":89960,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90236,"parent_record_offset":90212,"ancestor_record_offset":89960,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":90164,"parent_record_offset":90136,"ancestor_record_offset":89960,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":90052,"parent_record_offset":90016,"ancestor_record_offset":89960,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":90016,"parent_record_offset":89988,"ancestor_record_offset":89960,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":90076,"parent_record_offset":90052,"ancestor_record_offset":89960,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":90136,"parent_record_offset":90016,"ancestor_record_offset":89960,"inlinee":11549,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FCA
// 首条位置: {"module":177,"inlinee":12234}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D17
// 首条位置: {"module":151,"inlinee":11543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":151,"record_offset":90420,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":91372,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":91308,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":91660,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":89404,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":90448,"parent_record_offset":89340,"ancestor_record_offset":89340,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":91608,"parent_record_offset":91584,"ancestor_record_offset":89340,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":91584,"parent_record_offset":91528,"ancestor_record_offset":89340,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":91556,"parent_record_offset":91528,"ancestor_record_offset":89340,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":91528,"parent_record_offset":91500,"ancestor_record_offset":89340,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":90108,"parent_record_offset":90016,"ancestor_record_offset":89340,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":90188,"parent_record_offset":90164,"ancestor_record_offset":89340,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":91424,"parent_record_offset":91400,"ancestor_record_offset":89340,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90212,"parent_record_offset":90188,"ancestor_record_offset":89340,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":90236,"parent_record_offset":90212,"ancestor_record_offset":89340,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":90164,"parent_record_offset":90136,"ancestor_record_offset":89340,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":91732,"parent_record_offset":91708,"ancestor_record_offset":89340,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":91708,"parent_record_offset":91684,"ancestor_record_offset":89340,"inlinee":6856,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":89932,"parent_record_offset":89452,"ancestor_record_offset":89340,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":89656,"parent_record_offset":89540,"ancestor_record_offset":89340,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":90532,"parent_record_offset":90488,"ancestor_record_offset":89340,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":89540,"parent_record_offset":89452,"ancestor_record_offset":89340,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":89580,"parent_record_offset":89540,"ancestor_record_offset":89340,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":89452,"parent_record_offset":89404,"ancestor_record_offset":89340,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":90488,"parent_record_offset":90448,"ancestor_record_offset":89340,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":90016,"parent_record_offset":89988,"ancestor_record_offset":89340,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":89988,"parent_record_offset":89960,"ancestor_record_offset":89340,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":89680,"parent_record_offset":89656,"ancestor_record_offset":89340,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":89612,"parent_record_offset":89580,"ancestor_record_offset":89340,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":90824,"parent_record_offset":90800,"ancestor_record_offset":89340,"inlinee":6858,"source_line":38,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":90136,"parent_record_offset":90016,"ancestor_record_offset":89340,"inlinee":11549,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":89960,"parent_record_offset":89932,"ancestor_record_offset":89340,"inlinee":11546,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D1E
// 首条位置: {"module":151,"inlinee":11550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":92856,"parent_record_offset":91944,"ancestor_record_offset":91944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":91992,"parent_record_offset":91944,"ancestor_record_offset":91944,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":93064,"parent_record_offset":93040,"ancestor_record_offset":91944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":93040,"parent_record_offset":92984,"ancestor_record_offset":91944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":93012,"parent_record_offset":92984,"ancestor_record_offset":91944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":92984,"parent_record_offset":92956,"ancestor_record_offset":91944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":92880,"parent_record_offset":92856,"ancestor_record_offset":91944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":92220,"parent_record_offset":92016,"ancestor_record_offset":91944,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":92160,"parent_record_offset":92052,"ancestor_record_offset":91944,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":92052,"parent_record_offset":92016,"ancestor_record_offset":91944,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":92088,"parent_record_offset":92052,"ancestor_record_offset":91944,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":92016,"parent_record_offset":91992,"ancestor_record_offset":91944,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":92184,"parent_record_offset":92160,"ancestor_record_offset":91944,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":92120,"parent_record_offset":92088,"ancestor_record_offset":91944,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":92244,"parent_record_offset":92220,"ancestor_record_offset":91944,"inlinee":6858,"source_line":38,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D24
// 首条位置: {"module":151,"inlinee":11556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":95324,"parent_record_offset":95296,"ancestor_record_offset":95296,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":95348,"parent_record_offset":95324,"ancestor_record_offset":95296,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95372,"parent_record_offset":95348,"ancestor_record_offset":95296,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95396,"parent_record_offset":95372,"ancestor_record_offset":95296,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D22
// 首条位置: {"module":151,"inlinee":11554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":95148,"parent_record_offset":95120,"ancestor_record_offset":95120,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":95268,"parent_record_offset":95176,"ancestor_record_offset":95120,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":95348,"parent_record_offset":95324,"ancestor_record_offset":95120,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95372,"parent_record_offset":95348,"ancestor_record_offset":95120,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95396,"parent_record_offset":95372,"ancestor_record_offset":95120,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":95324,"parent_record_offset":95296,"ancestor_record_offset":95120,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":95212,"parent_record_offset":95176,"ancestor_record_offset":95120,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":95176,"parent_record_offset":95148,"ancestor_record_offset":95120,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":95236,"parent_record_offset":95212,"ancestor_record_offset":95120,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":95296,"parent_record_offset":95176,"ancestor_record_offset":95120,"inlinee":11556,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FC8
// 首条位置: {"module":177,"inlinee":12232}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D20
// 首条位置: {"module":151,"inlinee":11552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":151,"record_offset":95580,"parent_record_offset":94524,"ancestor_record_offset":94524,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":96368,"parent_record_offset":94524,"ancestor_record_offset":94524,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":96432,"parent_record_offset":94524,"ancestor_record_offset":94524,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":94576,"parent_record_offset":94524,"ancestor_record_offset":94524,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":95608,"parent_record_offset":94524,"ancestor_record_offset":94524,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":95268,"parent_record_offset":95176,"ancestor_record_offset":94524,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":95348,"parent_record_offset":95324,"ancestor_record_offset":94524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95372,"parent_record_offset":95348,"ancestor_record_offset":94524,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":95396,"parent_record_offset":95372,"ancestor_record_offset":94524,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":95324,"parent_record_offset":95296,"ancestor_record_offset":94524,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":96504,"parent_record_offset":96480,"ancestor_record_offset":94524,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":96480,"parent_record_offset":96456,"ancestor_record_offset":94524,"inlinee":6849,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":95092,"parent_record_offset":94620,"ancestor_record_offset":94524,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":95948,"parent_record_offset":95924,"ancestor_record_offset":94524,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":94816,"parent_record_offset":94700,"ancestor_record_offset":94524,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":95668,"parent_record_offset":95636,"ancestor_record_offset":94524,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":94700,"parent_record_offset":94620,"ancestor_record_offset":94524,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":94740,"parent_record_offset":94700,"ancestor_record_offset":94524,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":94620,"parent_record_offset":94576,"ancestor_record_offset":94524,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":95636,"parent_record_offset":95608,"ancestor_record_offset":94524,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":95176,"parent_record_offset":95148,"ancestor_record_offset":94524,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":95148,"parent_record_offset":95120,"ancestor_record_offset":94524,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":94840,"parent_record_offset":94816,"ancestor_record_offset":94524,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":94772,"parent_record_offset":94740,"ancestor_record_offset":94524,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":95296,"parent_record_offset":95176,"ancestor_record_offset":94524,"inlinee":11556,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":95120,"parent_record_offset":95092,"ancestor_record_offset":94524,"inlinee":11554,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D25
// 首条位置: {"module":151,"inlinee":11557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":96648,"parent_record_offset":96604,"ancestor_record_offset":96604,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":96864,"parent_record_offset":96672,"ancestor_record_offset":96604,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":96888,"parent_record_offset":96864,"ancestor_record_offset":96604,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":96804,"parent_record_offset":96700,"ancestor_record_offset":96604,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":96700,"parent_record_offset":96672,"ancestor_record_offset":96604,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":96732,"parent_record_offset":96700,"ancestor_record_offset":96604,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":96672,"parent_record_offset":96648,"ancestor_record_offset":96604,"inlinee":11507,"source_line":1916,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":96828,"parent_record_offset":96804,"ancestor_record_offset":96604,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":96764,"parent_record_offset":96732,"ancestor_record_offset":96604,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D32
// 首条位置: {"module":151,"inlinee":11570}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":105204,"parent_record_offset":105176,"ancestor_record_offset":105176,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":105228,"parent_record_offset":105204,"ancestor_record_offset":105176,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105252,"parent_record_offset":105228,"ancestor_record_offset":105176,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105276,"parent_record_offset":105252,"ancestor_record_offset":105176,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D30
// 首条位置: {"module":151,"inlinee":11568}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":105024,"parent_record_offset":104996,"ancestor_record_offset":104996,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":105148,"parent_record_offset":105052,"ancestor_record_offset":104996,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":105228,"parent_record_offset":105204,"ancestor_record_offset":104996,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105252,"parent_record_offset":105228,"ancestor_record_offset":104996,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105276,"parent_record_offset":105252,"ancestor_record_offset":104996,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":105204,"parent_record_offset":105176,"ancestor_record_offset":104996,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":105092,"parent_record_offset":105052,"ancestor_record_offset":104996,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":105052,"parent_record_offset":105024,"ancestor_record_offset":104996,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":105116,"parent_record_offset":105092,"ancestor_record_offset":104996,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":105176,"parent_record_offset":105052,"ancestor_record_offset":104996,"inlinee":11570,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FC5
// 首条位置: {"module":177,"inlinee":12229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D2E
// 首条位置: {"module":151,"inlinee":11566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":151,"record_offset":105460,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":106412,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":106348,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":106632,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":104440,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":105488,"parent_record_offset":104384,"ancestor_record_offset":104384,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":106584,"parent_record_offset":106560,"ancestor_record_offset":104384,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":106560,"parent_record_offset":106496,"ancestor_record_offset":104384,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":106528,"parent_record_offset":106496,"ancestor_record_offset":104384,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":106496,"parent_record_offset":106468,"ancestor_record_offset":104384,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":151,"record_offset":105148,"parent_record_offset":105052,"ancestor_record_offset":104384,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":105228,"parent_record_offset":105204,"ancestor_record_offset":104384,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":106440,"parent_record_offset":106412,"ancestor_record_offset":104384,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105252,"parent_record_offset":105228,"ancestor_record_offset":104384,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":105276,"parent_record_offset":105252,"ancestor_record_offset":104384,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":105204,"parent_record_offset":105176,"ancestor_record_offset":104384,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":151,"record_offset":106704,"parent_record_offset":106680,"ancestor_record_offset":104384,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":106680,"parent_record_offset":106656,"ancestor_record_offset":104384,"inlinee":6868,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":104968,"parent_record_offset":104488,"ancestor_record_offset":104384,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":151,"record_offset":105864,"parent_record_offset":105840,"ancestor_record_offset":104384,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":104692,"parent_record_offset":104576,"ancestor_record_offset":104384,"inlinee":7920,"source_line":220,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":105572,"parent_record_offset":105528,"ancestor_record_offset":104384,"inlinee":9489,"source_line":982,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":104576,"parent_record_offset":104488,"ancestor_record_offset":104384,"inlinee":9480,"source_line":248,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":104616,"parent_record_offset":104576,"ancestor_record_offset":104384,"inlinee":9481,"source_line":212,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":104488,"parent_record_offset":104440,"ancestor_record_offset":104384,"inlinee":11544,"source_line":1963,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":105528,"parent_record_offset":105488,"ancestor_record_offset":104384,"inlinee":9858,"source_line":1996,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/de.rs
//     evidence={"module":151,"record_offset":105052,"parent_record_offset":105024,"ancestor_record_offset":104384,"inlinee":7918,"source_line":2151,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":151,"record_offset":105024,"parent_record_offset":104996,"ancestor_record_offset":104384,"inlinee":11547,"source_line":126,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":104716,"parent_record_offset":104692,"ancestor_record_offset":104384,"inlinee":7922,"source_line":513,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/read.rs
//     evidence={"module":151,"record_offset":104648,"parent_record_offset":104616,"ancestor_record_offset":104384,"inlinee":9482,"source_line":502,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":105176,"parent_record_offset":105052,"ancestor_record_offset":104384,"inlinee":11570,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":151,"record_offset":104996,"parent_record_offset":104968,"ancestor_record_offset":104384,"inlinee":11568,"source_line":65,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D33
// 首条位置: {"module":151,"inlinee":11571}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":108004,"parent_record_offset":107852,"ancestor_record_offset":107852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":151,"record_offset":107916,"parent_record_offset":107852,"ancestor_record_offset":107852,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":151,"record_offset":107888,"parent_record_offset":107852,"ancestor_record_offset":107852,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":108156,"parent_record_offset":108132,"ancestor_record_offset":107852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":151,"record_offset":108132,"parent_record_offset":108076,"ancestor_record_offset":107852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":108104,"parent_record_offset":108076,"ancestor_record_offset":107852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":151,"record_offset":108076,"parent_record_offset":108052,"ancestor_record_offset":107852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":151,"record_offset":108028,"parent_record_offset":108004,"ancestor_record_offset":107852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D2B
// 首条位置: {"module":151,"inlinee":11563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":151,"record_offset":99724,"parent_record_offset":99696,"ancestor_record_offset":99696,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":151,"record_offset":99748,"parent_record_offset":99724,"ancestor_record_offset":99696,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99772,"parent_record_offset":99748,"ancestor_record_offset":99696,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":151,"record_offset":99796,"parent_record_offset":99772,"ancestor_record_offset":99696,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00151.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::CrashRecovered::constructor$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FD0
// 首条位置: {"module":177,"inlinee":12240}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FD2
// 首条位置: {"module":256,"inlinee":12242}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":256,"record_offset":49572,"parent_record_offset":49528,"ancestor_record_offset":49528,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":49892,"parent_record_offset":49864,"ancestor_record_offset":49528,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":49920,"parent_record_offset":49892,"ancestor_record_offset":49528,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":49864,"parent_record_offset":49832,"ancestor_record_offset":49528,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":49832,"parent_record_offset":49800,"ancestor_record_offset":49528,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":49800,"parent_record_offset":49768,"ancestor_record_offset":49528,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":49996,"parent_record_offset":49800,"ancestor_record_offset":49528,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":49732,"parent_record_offset":49700,"ancestor_record_offset":49528,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":49700,"parent_record_offset":49668,"ancestor_record_offset":49528,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":49636,"parent_record_offset":49604,"ancestor_record_offset":49528,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":49668,"parent_record_offset":49636,"ancestor_record_offset":49528,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":256,"record_offset":49604,"parent_record_offset":49572,"ancestor_record_offset":49528,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":256,"record_offset":49768,"parent_record_offset":49732,"ancestor_record_offset":49528,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":256,"record_offset":50064,"parent_record_offset":50036,"ancestor_record_offset":49528,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":256,"record_offset":50036,"parent_record_offset":49732,"ancestor_record_offset":49528,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":49948,"parent_record_offset":49920,"ancestor_record_offset":49528,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14AB
// 首条位置: {"module":10,"inlinee":5291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":5768,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":5293,"source_line":2035,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x346B
// 首条位置: {"module":234,"inlinee":13419}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":234,"record_offset":803860,"parent_record_offset":803808,"ancestor_record_offset":803808,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":803976,"parent_record_offset":803808,"ancestor_record_offset":803808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":234,"record_offset":805048,"parent_record_offset":803808,"ancestor_record_offset":803808,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":234,"record_offset":804244,"parent_record_offset":803808,"ancestor_record_offset":803808,"inlinee":5858,"source_line":1887,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":804188,"parent_record_offset":804164,"ancestor_record_offset":803808,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":234,"record_offset":804164,"parent_record_offset":804108,"ancestor_record_offset":803808,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":804136,"parent_record_offset":804108,"ancestor_record_offset":803808,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":804108,"parent_record_offset":804080,"ancestor_record_offset":803808,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":234,"record_offset":803884,"parent_record_offset":803860,"ancestor_record_offset":803808,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":803996,"parent_record_offset":803976,"ancestor_record_offset":803808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":803912,"parent_record_offset":803884,"ancestor_record_offset":803808,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":803940,"parent_record_offset":803884,"ancestor_record_offset":803808,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":234,"record_offset":804348,"parent_record_offset":804324,"ancestor_record_offset":803808,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":234,"record_offset":804396,"parent_record_offset":804372,"ancestor_record_offset":803808,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":234,"record_offset":804372,"parent_record_offset":804348,"ancestor_record_offset":803808,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":234,"record_offset":805204,"parent_record_offset":805180,"ancestor_record_offset":803808,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":234,"record_offset":804324,"parent_record_offset":804300,"ancestor_record_offset":803808,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":234,"record_offset":804300,"parent_record_offset":804276,"ancestor_record_offset":803808,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":234,"record_offset":804276,"parent_record_offset":804244,"ancestor_record_offset":803808,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00234.jsonl.gz"}
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
// 完整结构及原始类型记录: crash_payload.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::BacktracePayload
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::BacktracePayload>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashPayload
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashPayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> >", "constructor": "core::option::Option", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashPickupMessage"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >", "constructor": "serde_json::de::from_slice", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::CrashRecovered
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashRecovered
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::CrashRecovered>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSave
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$42::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde::ser::impls::impl$21::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >", "constructor": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "serde_json::de::VariantAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::RecoveredSave>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSession
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$16::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSession,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$16::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::ResolvedCrash
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::crash_payload::ResolvedCrash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::ResolvedCrash> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::ResolvedCrash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::SaveRecoveryError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::SaveRecoveryError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$42::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde::ser::impls::impl$21::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >", "constructor": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "serde_json::de::VariantAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: crash_payload.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: crash_payload.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 61; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::merge_walls::MergeWallsCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::merge_walls::MergeWallsCmd>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::deferred_wall_history::PendingWallHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::merge_walls::MergeWallsCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::CrashRecovered::constructor$0
// 信息 ID: 99986732bb7a6fddd3488b5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=30200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 信息 ID: 8a39678ac6895d2f179277bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=49528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 信息 ID: e367c4e2e12b28ef6dc27854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=90824; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=91268; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=91684; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=91796; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=92244; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=92816; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=30104; 不是独立调用边。
// [PDB 内联宿主] serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >; module=218, inline_record=12188; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: f088b9094efa90218a9a3612
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__FieldVisitor>; module=177, inline_record=652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 875c1481b3c43c00985c8fa4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=151, inline_record=87116; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=130912; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=194384; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=199756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: aeed5d880c571c09937e054f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor> @ 0x2a5e9a -> 0x2dd760 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x29ad30 -> 0x2de620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: a3bcff24fb86c27a18e23be2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=151, inline_record=85300; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=151, inline_record=86768; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=130732; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=159864; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=194208; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=199576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: dac0a6ddc614e05d30130e88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=177, inline_record=864; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=177, inline_record=1308; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=177, inline_record=1520; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=177, inline_record=1964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 331a844a56f34587ae59fa8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=8732; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=8936; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=29440; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=177, inline_record=29656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: ec39dcd5c9e38aa20f7fb6f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=90136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: fe0ff507ae5fc5da84a1fa67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=99516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 信息 ID: 673ebb7e3f7b166fa7a5eb55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=177, inline_record=1752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: 920c6df8d65dcf720ed745bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=98884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 信息 ID: 7632b13ff5951a817fc681bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=101908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 0d8a92d272d04c43a498932c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=89960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: 23c3547a9530105eed783721
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=177, inline_record=2196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 4fb4d017662710053ed733c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=89340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: 2884afa1957d2c2ebde851b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=91944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 信息 ID: b4203fc4c3a620dc00056683
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=95296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: 33c1f408e87d40d8aa449069
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=95120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 信息 ID: dadfefdda965ccb63fd1d669
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=177, inline_record=1096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: a2a897104ba34c042d059281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=94524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 信息 ID: 3f25d3d8baa805596612efb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=96604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 信息 ID: ea6fa8682e08b88e52fa2555
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=105176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: 345b92514914bb1c81c62841
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=104996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 信息 ID: d1b3ccbbb8e46b5e3446c2b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=177, inline_record=436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: 7c7d71f664804900e0563af4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=104384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 信息 ID: f064b7fb48f557f27208aacb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=107852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 信息 ID: 3e663986da3939c70212d730
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=151, inline_record=99696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: e4b4cc699224825071dce36a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=130120; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=159196; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=193596; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=198972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 7c0f6ea11dc13fe74d6933e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=133412; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=167076; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=196476; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=151, inline_record=201776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 信息 ID: 797dc99e9df076a7bd21ae8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::SliceRead>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> > @ 0x33842f -> 0x2dd110 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::from_slice<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> > @ 0x2936ae -> 0x2de1e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: ac178fafbffde81692c8a5f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x2ddb8f -> 0x2dcef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x263598 -> 0x2dcfd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x2ddbfa -> 0x2dd440 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x263606 -> 0x2dd8b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > > @ 0x275556 -> 0x2dd990 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1> @ 0x1a9a9e -> 0x2ddab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::ClientHandle::send_message @ 0x3b46f4 -> 0x2ddab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 信息 ID: c8315511bd64dc8efdb9b82e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_minidump_created @ 0x15fe9 -> 0x307a90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_message @ 0x165a6 -> 0x307a90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 信息 ID: a6b48bd46cc179873b5cbac2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=10, inline_record=5732; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=10, inline_record=7808; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=10, inline_record=9300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 信息 ID: 7cbcc37e40fbbd839d77b1c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>; module=234, inline_record=803808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$5::fmt
// 信息 ID: 8603a28d1f9f81fb5574bdc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 信息 ID: 0b38305bf22ec1db9f5776ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=13576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 信息 ID: 79f355e66c909584893ec340
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=12064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 信息 ID: c9c62f150a86b865a19e784d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=13612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 信息 ID: 473d32dfca890dcb58bcd815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=4, inline_record=14388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
