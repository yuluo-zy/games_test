// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset/src/texture/desc.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f90f8fa7ef7b030b22de096efd4b51abc0b5a072a8735129db34006fcaaec3b8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: desc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2828, 3912, 3913, 3927, 4396, 4397, 4398, 4431, 4432, 4434, 4591
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 4, 11, 19, 26, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B689
// 首条位置: {"module":4396,"inlinee":112265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":42460,"parent_record_offset":42436,"ancestor_record_offset":42436,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":29612,"parent_record_offset":29588,"ancestor_record_offset":29588,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 4, 11, 19, 26, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B686
// 首条位置: {"module":4396,"inlinee":112262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":3200,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":3200,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":3200,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":3200,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 4, 11, 19, 26, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8BA
// 首条位置: {"module":4396,"inlinee":121018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":3084,"parent_record_offset":3048,"ancestor_record_offset":3048,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4396,"record_offset":3200,"parent_record_offset":3168,"ancestor_record_offset":3048,"inlinee":112262,"source_line":26,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":3048,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":3132,"parent_record_offset":3084,"ancestor_record_offset":3048,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":3048,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":3048,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":3048,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":3168,"parent_record_offset":3084,"ancestor_record_offset":3048,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 4, 11, 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B684
// 首条位置: {"module":4396,"inlinee":112260}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":23572,"parent_record_offset":23448,"ancestor_record_offset":23448,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":21124,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":23544,"parent_record_offset":23448,"ancestor_record_offset":23448,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=5
// 源码行: 4, 11, 19, 26, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3927,"record_offset":248,"rva":29957808,"va":5398666928}
// 编译实例 RVAs: 0x1c91eb0, 0x1c92040, 0x1c921c0, 0x1c925b0, 0x1c92910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$ron..ser..Serializer$LT$W$GT$$u20$as$u20$serde..ser..Serializer$GT$::serialize_newtype_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1982649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398667328,"raw_hex":"e8b9401e00","size":5,"soft_fail":false,"target_va":5400650256,"va":5398667602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$ron..ser..Serializer$LT$W$GT$$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1981813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398667712,"raw_hex":"e8753d1e00","size":5,"soft_fail":false,"target_va":5400649600,"va":5398667782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1976992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398667712,"raw_hex":"e8a02a1e00","size":5,"soft_fail":false,"target_va":5400644880,"va":5398667883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140009a40
//     owner=unresolved_namespace; count=6
//     target_aliases=4 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29918367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398666928,"raw_hex":"e8617b37fe","size":5,"soft_fail":false,"target_va":5368748608,"va":5398666970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400105d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29891998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398667712,"raw_hex":"e862e237fe","size":5,"soft_fail":false,"target_va":5368776144,"va":5398668137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::ser::Serializer$LT$W$GT$::write_identifier::h34b4eec305381b73 (.llvm.9691779022443984248)
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1963888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398666928,"raw_hex":"e870f71d00","size":5,"soft_fail":false,"target_va":5400630896,"va":5398667003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3927,"record_offset":1660,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3927,"record_offset":1424,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3927,"record_offset":1604,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":3927,"record_offset":444,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5057,"source_line":854,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3927,"record_offset":1712,"parent_record_offset":1684,"ancestor_record_offset":1220,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3927,"record_offset":476,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3927,"record_offset":1184,"parent_record_offset":1080,"ancestor_record_offset":764,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":3927,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1220,"inlinee":4979,"source_line":1098,"archive":"_pdb/A/modules/03927.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$0::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B623
// 首条位置: {"module":3912,"inlinee":112163}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160BA
// 首条位置: {"module":2828,"inlinee":90298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$13::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160BC
// 首条位置: {"module":2828,"inlinee":90300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B627
// 首条位置: {"module":3912,"inlinee":112167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$20::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160BE
// 首条位置: {"module":2828,"inlinee":90302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 26, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8C4
// 首条位置: {"module":4396,"inlinee":121028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4396,"record_offset":32156,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4396,"record_offset":33156,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":32072,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":33096,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":30880,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":32100,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4396,"record_offset":34048,"parent_record_offset":34024,"ancestor_record_offset":30812,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4396,"record_offset":34024,"parent_record_offset":33968,"ancestor_record_offset":30812,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4396,"record_offset":33996,"parent_record_offset":33968,"ancestor_record_offset":30812,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4396,"record_offset":33968,"parent_record_offset":33944,"ancestor_record_offset":30812,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4396,"record_offset":32436,"parent_record_offset":32412,"ancestor_record_offset":30812,"inlinee":112265,"source_line":11,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4396,"record_offset":31676,"parent_record_offset":31648,"ancestor_record_offset":30812,"inlinee":112262,"source_line":26,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4396,"record_offset":31216,"parent_record_offset":31192,"ancestor_record_offset":30812,"inlinee":121018,"source_line":26,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":31728,"parent_record_offset":31704,"ancestor_record_offset":30812,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":31072,"parent_record_offset":30924,"ancestor_record_offset":30812,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":31112,"parent_record_offset":31072,"ancestor_record_offset":30812,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":31248,"parent_record_offset":30924,"ancestor_record_offset":30812,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4396,"record_offset":33824,"parent_record_offset":33248,"ancestor_record_offset":30812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":31032,"parent_record_offset":30924,"ancestor_record_offset":30812,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":31752,"parent_record_offset":31728,"ancestor_record_offset":30812,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":31776,"parent_record_offset":31752,"ancestor_record_offset":30812,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":31704,"parent_record_offset":31676,"ancestor_record_offset":30812,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":4396,"record_offset":31152,"parent_record_offset":30924,"ancestor_record_offset":30812,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":31572,"parent_record_offset":31544,"ancestor_record_offset":30812,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":33564,"parent_record_offset":33540,"ancestor_record_offset":30812,"inlinee":9499,"source_line":445,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":33540,"parent_record_offset":33516,"ancestor_record_offset":30812,"inlinee":7915,"source_line":455,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":30924,"parent_record_offset":30880,"ancestor_record_offset":30812,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":32208,"parent_record_offset":32184,"ancestor_record_offset":30812,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":42460,"parent_record_offset":42436,"ancestor_record_offset":40240,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_option
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":42028,"parent_record_offset":42004,"ancestor_record_offset":40240,"inlinee":9520,"source_line":160,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":32944,"parent_record_offset":32920,"ancestor_record_offset":30812,"inlinee":8558,"source_line":188,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":33516,"parent_record_offset":33492,"ancestor_record_offset":30812,"inlinee":7913,"source_line":139,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":32460,"parent_record_offset":32436,"ancestor_record_offset":30812,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":31648,"parent_record_offset":31572,"ancestor_record_offset":30812,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":31192,"parent_record_offset":30924,"ancestor_record_offset":30812,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":42004,"parent_record_offset":41980,"ancestor_record_offset":40240,"inlinee":7231,"source_line":931,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":32920,"parent_record_offset":32896,"ancestor_record_offset":30812,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$27::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":33648,"parent_record_offset":33624,"ancestor_record_offset":30812,"inlinee":9502,"source_line":1838,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$28::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":33492,"parent_record_offset":33468,"ancestor_record_offset":30812,"inlinee":6867,"source_line":1874,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":4396,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":30812,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":4396,"record_offset":33672,"parent_record_offset":33648,"ancestor_record_offset":30812,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B633
// 首条位置: {"module":3913,"inlinee":112179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$26::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160B0
// 首条位置: {"module":2828,"inlinee":90288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33084,"parent_record_offset":32888,"ancestor_record_offset":32888,"inlinee":90300,"source_line":11,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33112,"parent_record_offset":32888,"ancestor_record_offset":32888,"inlinee":90302,"source_line":19,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33056,"parent_record_offset":32888,"ancestor_record_offset":32888,"inlinee":90298,"source_line":4,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32928,"parent_record_offset":32888,"ancestor_record_offset":32888,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":32992,"parent_record_offset":32956,"ancestor_record_offset":32888,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":32888,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32956,"parent_record_offset":32928,"ancestor_record_offset":32888,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$27::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160B4
// 首条位置: {"module":2828,"inlinee":90292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$21::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3913,"record_offset":2688,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":112179,"source_line":26,"archive":"_pdb/A/modules/03913.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":40420,"parent_record_offset":39644,"ancestor_record_offset":39644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":39696,"parent_record_offset":39644,"ancestor_record_offset":39644,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":40164,"parent_record_offset":40140,"ancestor_record_offset":39644,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":40188,"parent_record_offset":40164,"ancestor_record_offset":39644,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":40644,"parent_record_offset":40620,"ancestor_record_offset":39644,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":40140,"parent_record_offset":40096,"ancestor_record_offset":39644,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":40620,"parent_record_offset":40564,"ancestor_record_offset":39644,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":40592,"parent_record_offset":40564,"ancestor_record_offset":39644,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":40096,"parent_record_offset":40060,"ancestor_record_offset":39644,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":40060,"parent_record_offset":40024,"ancestor_record_offset":39644,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":40260,"parent_record_offset":40060,"ancestor_record_offset":39644,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":40564,"parent_record_offset":40540,"ancestor_record_offset":39644,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":39980,"parent_record_offset":39940,"ancestor_record_offset":39644,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":39940,"parent_record_offset":39900,"ancestor_record_offset":39644,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":39900,"parent_record_offset":39860,"ancestor_record_offset":39644,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":40024,"parent_record_offset":39980,"ancestor_record_offset":39644,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":39860,"parent_record_offset":39816,"ancestor_record_offset":39644,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":2828,"record_offset":40376,"parent_record_offset":39816,"ancestor_record_offset":39644,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2828,"record_offset":40328,"parent_record_offset":40304,"ancestor_record_offset":39644,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":40304,"parent_record_offset":39980,"ancestor_record_offset":39644,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":40444,"parent_record_offset":40420,"ancestor_record_offset":39644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":40216,"parent_record_offset":40188,"ancestor_record_offset":39644,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":39736,"parent_record_offset":39696,"ancestor_record_offset":39644,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2828,"record_offset":39776,"parent_record_offset":39736,"ancestor_record_offset":39644,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":2828,"record_offset":39816,"parent_record_offset":39776,"ancestor_record_offset":39644,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$31::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160AE
// 首条位置: {"module":2828,"inlinee":90286}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":2828,"record_offset":33144,"parent_record_offset":32012,"ancestor_record_offset":32012,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$26::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":32888,"parent_record_offset":32012,"ancestor_record_offset":32012,"inlinee":90288,"source_line":26,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$57::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32052,"parent_record_offset":32012,"ancestor_record_offset":32012,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33084,"parent_record_offset":32888,"ancestor_record_offset":32012,"inlinee":90300,"source_line":11,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33112,"parent_record_offset":32888,"ancestor_record_offset":32012,"inlinee":90302,"source_line":19,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33056,"parent_record_offset":32888,"ancestor_record_offset":32012,"inlinee":90298,"source_line":4,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":32992,"parent_record_offset":32956,"ancestor_record_offset":32012,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":32012,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":2828,"record_offset":32576,"parent_record_offset":32484,"ancestor_record_offset":32012,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":32400,"parent_record_offset":32332,"ancestor_record_offset":32012,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2828,"record_offset":32668,"parent_record_offset":32636,"ancestor_record_offset":32012,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":32636,"parent_record_offset":32332,"ancestor_record_offset":32012,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32928,"parent_record_offset":32888,"ancestor_record_offset":32012,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32956,"parent_record_offset":32928,"ancestor_record_offset":32012,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32516,"parent_record_offset":32484,"ancestor_record_offset":32012,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":32424,"parent_record_offset":32400,"ancestor_record_offset":32012,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":32448,"parent_record_offset":32424,"ancestor_record_offset":32012,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":32128,"parent_record_offset":32104,"ancestor_record_offset":32012,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":32176,"parent_record_offset":32152,"ancestor_record_offset":32012,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":32152,"parent_record_offset":32128,"ancestor_record_offset":32012,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":32012,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32484,"parent_record_offset":32332,"ancestor_record_offset":32012,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32332,"parent_record_offset":32052,"ancestor_record_offset":32012,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":32012,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32544,"parent_record_offset":32516,"ancestor_record_offset":32012,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32604,"parent_record_offset":32484,"ancestor_record_offset":32012,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32372,"parent_record_offset":32332,"ancestor_record_offset":32012,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/02828.jsonl.gz"}
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
// 完整结构及原始类型记录: desc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::desc::CompressionMethod
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::CompressionMethod>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "asset::texture::desc::CompressionMethod"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::desc::Gamma
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::Gamma>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "asset::texture::desc::Gamma"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::desc::MipChain
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset::texture::desc::MipChain>", "constructor": "enum2$", "arguments": ["asset::texture::desc::MipChain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<asset::texture::desc::MipChain> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "enum2$<asset::texture::desc::MipChain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::desc::TextureAssetDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "asset_pipe::texture::compile_texture_assets::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::texture::desc::TextureAssetDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "std::collections::hash::map::Keys", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<relative_path::RelativePathBuf,core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["relative_path::RelativePathBuf", "core::iter::adapters::cloned::Cloned<std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "tuple$", "arguments": ["enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >", "relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::Parallel::each", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::impl$0::each::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "tuple$", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>", "constructor": "core::slice::cmp::impl$4::equal", "arguments": ["asset::texture::desc::TextureAssetDesc", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "constructor": "tuple$", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >,asset_pipe::texture::compile_texture_assets::closure_env$3>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "asset_pipe::texture::compile_texture_assets::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["relative_path::RelativePath", "relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureAssetDesc>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset::texture::desc::TextureAssetDesc>", "constructor": "core::marker::PhantomData", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<asset::texture::desc::TextureAssetDesc> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<asset::texture::desc::TextureAssetDesc,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["asset::texture::desc::TextureAssetDesc", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::desc::TextureOpts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureOpts>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "asset::texture::desc::TextureOpts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: desc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: desc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize
// 信息 ID: 3b70fae976a63ca2b09efba5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<asset::texture::desc::TextureAssetDesc> >; module=4396, inline_record=10668; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=29588; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=32436; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=42436; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=42812; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=43196; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=51788; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=52288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 23c9b2a3d2b05b599d685f4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=3200; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=4104; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=5132; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=31676; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=41132; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4396, inline_record=46444; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=7096; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=7756; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 1f74c0da9fe01a7a96700a1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=3048; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=3964; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4396, inline_record=4980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=31216; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=31420; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=31544; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=40640; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=40848; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: b3eb8826c724db99d15960f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=21072; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=22260; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=23448; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=24304; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=25564; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4431, inline_record=26752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 997b1e82ff3d20f3256298c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=30812; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=40240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 46a342b9ab90e38eff845f5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] asset::texture::desc::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::Gamma> @ 0x1e72594 -> 0x1c91eb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<asset::texture::desc::MipChain> > @ 0x1e71ea4 -> 0x1c92040 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureOpts> @ 0x1e72bb4 -> 0x1c921c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureAssetDesc> @ 0x1e73a94 -> 0x1c925b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> > @ 0x1e74466 -> 0x1c925b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::CompressionMethod> @ 0x1e71b94 -> 0x1c92910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$0::clone
// 信息 ID: 4fdf2a1ae348dab38b3564af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4432; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$13::eq
// 信息 ID: b5a5520b3527c5185a106828
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=33084; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3927, inline_record=4324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$14::clone
// 信息 ID: 2a022c0fd419dccfbd98bd6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=4600; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3912, inline_record=10920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$20::eq
// 信息 ID: 27e75f653dde90025ac9a6ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=33112; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3927, inline_record=4352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$21::clone
// 信息 ID: f23dfa9cc84ba908fd4a9837
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>; module=3913, inline_record=2688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$26::eq
// 信息 ID: 9c072b2b570d5e6fb1fbf670
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=32888; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3927, inline_record=4144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$27::clone
// 信息 ID: 181f554cd71a111f6b9d0358
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=39644; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$10::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>; module=3913, inline_record=1988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$31::eq
// 信息 ID: a4b57bedd2441e595fd771db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=32012; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3927, inline_record=3264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$3::eq
// 信息 ID: e262af685c40c6a8c422de57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=33056; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=44952; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3927, inline_record=4296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
