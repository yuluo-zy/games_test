// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/texture/desc.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b4bb6253f9f0d4dfd6b52f475c1e5f9290f9f25f06312c1dbc8be1587ea7be00
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: desc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1727, 3168, 3176, 4139, 4150, 4589, 4590, 4591, 4745
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 4, 11, 19, 27, 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A693
// 首条位置: {"module":4139,"inlinee":108179}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":47252,"parent_record_offset":47228,"ancestor_record_offset":47228,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":33840,"parent_record_offset":33816,"ancestor_record_offset":33816,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 4, 11, 19, 27, 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A690
// 首条位置: {"module":4139,"inlinee":108176}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":2980,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":2980,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2980,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":2980,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 4, 11, 19, 27, 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EA26
// 首条位置: {"module":4139,"inlinee":125478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":2848,"inlinee":108176,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":2848,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":2916,"parent_record_offset":2876,"ancestor_record_offset":2848,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":3376,"parent_record_offset":2876,"ancestor_record_offset":2848,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2848,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":2848,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":2848,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":2952,"parent_record_offset":2876,"ancestor_record_offset":2848,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 4, 11, 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A68E
// 首条位置: {"module":4139,"inlinee":108174}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":25052,"parent_record_offset":24928,"ancestor_record_offset":24928,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":24988,"parent_record_offset":24928,"ancestor_record_offset":24928,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":24960,"parent_record_offset":24928,"ancestor_record_offset":24928,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":25024,"parent_record_offset":24928,"ancestor_record_offset":24928,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=5
// 源码行: 4, 11, 19, 27, 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3176,"record_offset":256,"rva":34472512,"va":5403181632}
// 编译实例 RVAs: 0x20e0240, 0x20e0310, 0x20e0420, 0x20e0640, 0x20e0830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3607623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403181632,"raw_hex":"e8470c3700","size":5,"soft_fail":false,"target_va":5406789328,"va":5403181700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142451200
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3608472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403181632,"raw_hex":"e8980f3700","size":5,"soft_fail":false,"target_va":5406790144,"va":5403181667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142451b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3610185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e849163700","size":5,"soft_fail":false,"target_va":5406792480,"va":5403182290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142451e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3610819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e8c3183700","size":5,"soft_fail":false,"target_va":5406793216,"va":5403182392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142452f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3615254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e8162a3700","size":5,"soft_fail":false,"target_va":5406797600,"va":5403182341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142453200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3615446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182656,"raw_hex":"e8d62a3700","size":5,"soft_fail":false,"target_va":5406798336,"va":5403182885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142453a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3617609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182656,"raw_hex":"e849333700","size":5,"soft_fail":false,"target_va":5406800448,"va":5403182834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142453d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3618396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182656,"raw_hex":"e85c363700","size":5,"soft_fail":false,"target_va":5406801184,"va":5403182783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424547c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3621660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e81c433700","size":5,"soft_fail":false,"target_va":5406803904,"va":5403182239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142455590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3625003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e82b503700","size":5,"soft_fail":false,"target_va":5406807440,"va":5403182432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142455e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3627435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403182112,"raw_hex":"e8ab593700","size":5,"soft_fail":false,"target_va":5406809616,"va":5403182176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142455ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3628008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403181840,"raw_hex":"e8e85b3700","size":5,"soft_fail":false,"target_va":5406810096,"va":5403182083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3176,"record_offset":1532,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3176,"record_offset":1324,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3176,"record_offset":1472,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":3176,"record_offset":452,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5560,"source_line":854,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3176,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":1120,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3176,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":256,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":3176,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":1120,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F9B
// 首条位置: {"module":1727,"inlinee":77723}
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
// IPI IDs: 0x12F9D
// 首条位置: {"module":1727,"inlinee":77725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":31980,"parent_record_offset":31952,"ancestor_record_offset":31952,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":32004,"parent_record_offset":31980,"ancestor_record_offset":31952,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$20::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F9F
// 首条位置: {"module":1727,"inlinee":77727}
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
// 源码行: 27, 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EA30
// 首条位置: {"module":4139,"inlinee":125488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4139,"record_offset":39812,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":40648,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":39728,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":44016,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":38648,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":39756,"parent_record_offset":38572,"ancestor_record_offset":38572,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":41004,"parent_record_offset":40980,"ancestor_record_offset":38572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":40980,"parent_record_offset":40892,"ancestor_record_offset":38572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40752,"parent_record_offset":40728,"ancestor_record_offset":38572,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40920,"parent_record_offset":40892,"ancestor_record_offset":38572,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40892,"parent_record_offset":40868,"ancestor_record_offset":38572,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":38572,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40776,"parent_record_offset":40752,"ancestor_record_offset":38572,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":40868,"parent_record_offset":40844,"ancestor_record_offset":38572,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4139,"record_offset":40728,"parent_record_offset":40704,"ancestor_record_offset":38572,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4139,"record_offset":40704,"parent_record_offset":40676,"ancestor_record_offset":38572,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":40036,"parent_record_offset":40012,"ancestor_record_offset":38572,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":39332,"parent_record_offset":39304,"ancestor_record_offset":38572,"inlinee":108176,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":38948,"parent_record_offset":38924,"ancestor_record_offset":38572,"inlinee":125478,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":39384,"parent_record_offset":39360,"ancestor_record_offset":38572,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":38852,"parent_record_offset":38692,"ancestor_record_offset":38572,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":38980,"parent_record_offset":38692,"ancestor_record_offset":38572,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":40948,"parent_record_offset":40920,"ancestor_record_offset":38572,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":40676,"parent_record_offset":40648,"ancestor_record_offset":38572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":38816,"parent_record_offset":38692,"ancestor_record_offset":38572,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":38572,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":39432,"parent_record_offset":39408,"ancestor_record_offset":38572,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":39360,"parent_record_offset":39332,"ancestor_record_offset":38572,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":38884,"parent_record_offset":38692,"ancestor_record_offset":38572,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":39228,"parent_record_offset":39200,"ancestor_record_offset":38572,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":41680,"parent_record_offset":41648,"ancestor_record_offset":38572,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":41648,"parent_record_offset":41616,"ancestor_record_offset":38572,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":38692,"parent_record_offset":38648,"ancestor_record_offset":38572,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":39864,"parent_record_offset":39840,"ancestor_record_offset":38572,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":47252,"parent_record_offset":47228,"ancestor_record_offset":45040,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":46804,"parent_record_offset":46780,"ancestor_record_offset":45040,"inlinee":6910,"source_line":160,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":40500,"parent_record_offset":40476,"ancestor_record_offset":38572,"inlinee":6904,"source_line":188,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":41616,"parent_record_offset":41584,"ancestor_record_offset":38572,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":40060,"parent_record_offset":40036,"ancestor_record_offset":38572,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":39304,"parent_record_offset":39228,"ancestor_record_offset":38572,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":38924,"parent_record_offset":38692,"ancestor_record_offset":38572,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":46780,"parent_record_offset":46756,"ancestor_record_offset":45040,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":40476,"parent_record_offset":40452,"ancestor_record_offset":38572,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$27::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":41784,"parent_record_offset":41760,"ancestor_record_offset":38572,"inlinee":5167,"source_line":1838,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$27::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":41872,"parent_record_offset":41680,"ancestor_record_offset":38572,"inlinee":5172,"source_line":1845,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$28::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":41584,"parent_record_offset":41552,"ancestor_record_offset":38572,"inlinee":5208,"source_line":1874,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":4139,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":38572,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":4139,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":38572,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A677
// 首条位置: {"module":3168,"inlinee":108151}
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
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F99
// 首条位置: {"module":1727,"inlinee":77721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31952,"parent_record_offset":31800,"ancestor_record_offset":31800,"inlinee":77725,"source_line":11,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":32040,"parent_record_offset":31800,"ancestor_record_offset":31800,"inlinee":77727,"source_line":19,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31924,"parent_record_offset":31800,"ancestor_record_offset":31800,"inlinee":77723,"source_line":4,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31836,"parent_record_offset":31800,"ancestor_record_offset":31800,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31864,"parent_record_offset":31836,"ancestor_record_offset":31800,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":31800,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":32004,"parent_record_offset":31980,"ancestor_record_offset":31800,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":31980,"parent_record_offset":31952,"ancestor_record_offset":31800,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$27::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12FB0
// 首条位置: {"module":1727,"inlinee":77744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::desc::impl$21::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":3168,"record_offset":3104,"parent_record_offset":2320,"ancestor_record_offset":2320,"inlinee":108151,"source_line":27,"archive":"_pdb/B/modules/03168.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":46648,"parent_record_offset":45760,"ancestor_record_offset":45760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":45812,"parent_record_offset":45760,"ancestor_record_offset":45760,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":46484,"parent_record_offset":46460,"ancestor_record_offset":45760,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":46508,"parent_record_offset":46484,"ancestor_record_offset":45760,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":46896,"parent_record_offset":46872,"ancestor_record_offset":45760,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":46460,"parent_record_offset":46276,"ancestor_record_offset":45760,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1727,"record_offset":46872,"parent_record_offset":46816,"ancestor_record_offset":45760,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46200,"parent_record_offset":46164,"ancestor_record_offset":45760,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46844,"parent_record_offset":46816,"ancestor_record_offset":45760,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46816,"parent_record_offset":46792,"ancestor_record_offset":45760,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46276,"parent_record_offset":46236,"ancestor_record_offset":45760,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46236,"parent_record_offset":46200,"ancestor_record_offset":45760,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46792,"parent_record_offset":46768,"ancestor_record_offset":45760,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1727,"record_offset":46320,"parent_record_offset":46276,"ancestor_record_offset":45760,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":46120,"parent_record_offset":46080,"ancestor_record_offset":45760,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1727,"record_offset":46080,"parent_record_offset":46004,"ancestor_record_offset":45760,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":46052,"parent_record_offset":46004,"ancestor_record_offset":45760,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":46164,"parent_record_offset":46120,"ancestor_record_offset":45760,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":45964,"parent_record_offset":45928,"ancestor_record_offset":45760,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1727,"record_offset":46004,"parent_record_offset":45964,"ancestor_record_offset":45760,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":46344,"parent_record_offset":46320,"ancestor_record_offset":45760,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1727,"record_offset":46368,"parent_record_offset":46344,"ancestor_record_offset":45760,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":46584,"parent_record_offset":46560,"ancestor_record_offset":45760,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":46416,"parent_record_offset":46392,"ancestor_record_offset":45760,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1727,"record_offset":46392,"parent_record_offset":46368,"ancestor_record_offset":45760,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1727,"record_offset":46560,"parent_record_offset":46120,"ancestor_record_offset":45760,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":46672,"parent_record_offset":46648,"ancestor_record_offset":45760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":45852,"parent_record_offset":45812,"ancestor_record_offset":45760,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1727,"record_offset":45892,"parent_record_offset":45852,"ancestor_record_offset":45760,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1727,"record_offset":45928,"parent_record_offset":45892,"ancestor_record_offset":45760,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::desc::impl$31::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12F97
// 首条位置: {"module":1727,"inlinee":77719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1727,"record_offset":32072,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$26::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31800,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":77721,"source_line":27,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":30844,"parent_record_offset":30812,"ancestor_record_offset":30812,"inlinee":10524,"source_line":2057,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3176,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":3116,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3176,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":3116,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3176,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":3116,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3116,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3116,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":3116,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3176,"record_offset":4484,"parent_record_offset":4452,"ancestor_record_offset":3116,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03176.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31952,"parent_record_offset":31800,"ancestor_record_offset":30812,"inlinee":77725,"source_line":11,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":32040,"parent_record_offset":31800,"ancestor_record_offset":30812,"inlinee":77727,"source_line":19,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":1727,"record_offset":31924,"parent_record_offset":31800,"ancestor_record_offset":30812,"inlinee":77723,"source_line":4,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31864,"parent_record_offset":31836,"ancestor_record_offset":30812,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":1727,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":30812,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":32004,"parent_record_offset":31980,"ancestor_record_offset":30812,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":31420,"parent_record_offset":31124,"ancestor_record_offset":30812,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1727,"record_offset":31644,"parent_record_offset":31504,"ancestor_record_offset":30812,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1727,"record_offset":31256,"parent_record_offset":31220,"ancestor_record_offset":30812,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31220,"parent_record_offset":31192,"ancestor_record_offset":30812,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31836,"parent_record_offset":31800,"ancestor_record_offset":30812,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1727,"record_offset":31556,"parent_record_offset":31504,"ancestor_record_offset":30812,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":31444,"parent_record_offset":31420,"ancestor_record_offset":30812,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":30812,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":30920,"parent_record_offset":30896,"ancestor_record_offset":30812,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":30968,"parent_record_offset":30944,"ancestor_record_offset":30812,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1727,"record_offset":30944,"parent_record_offset":30920,"ancestor_record_offset":30812,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31192,"parent_record_offset":31124,"ancestor_record_offset":30812,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":30896,"parent_record_offset":30868,"ancestor_record_offset":30812,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31284,"parent_record_offset":31256,"ancestor_record_offset":30812,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31164,"parent_record_offset":31124,"ancestor_record_offset":30812,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31504,"parent_record_offset":31124,"ancestor_record_offset":30812,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31124,"parent_record_offset":30844,"ancestor_record_offset":30812,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":30868,"parent_record_offset":30844,"ancestor_record_offset":30812,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31584,"parent_record_offset":31556,"ancestor_record_offset":30812,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1727,"record_offset":31616,"parent_record_offset":31504,"ancestor_record_offset":30812,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01727.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 27
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::texture::desc::TextureAssetDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "std::collections::hash::map::IntoIter", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "asset_pipe::texture::compile_texture_assets::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<asset::texture::desc::TextureAssetDesc>", "constructor": "slice2$", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<slice2$<asset::texture::desc::TextureAssetDesc> >", "constructor": "core::ptr::drop_in_place", "arguments": ["slice2$<asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["asset::texture::desc::TextureAssetDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["relative_path::RelativePath", "relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePath,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset::texture::desc::TextureAssetDesc>", "constructor": "core::marker::PhantomData", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<asset::texture::desc::TextureAssetDesc> >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc> >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<asset::texture::desc::TextureAssetDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureAssetDesc>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize
// 信息 ID: 152106fe4525d41539b2a7fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<asset::texture::desc::TextureAssetDesc> >; module=4139, inline_record=9368; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=33816; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=40036; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=47228; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=47616; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=47960; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33276; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33740; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: b3807cb062cb1eb6b6362faa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=2980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=3972; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=4868; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=39332; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=45956; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4139, inline_record=51200; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=6388; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset::texture::desc::_::impl$0::deserialize::__FieldVisitor>; module=4150, inline_record=7052; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: b6e31d9b4d0e3b5f58feee36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=2848; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=3828; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >; module=4139, inline_record=4712; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=38948; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=39104; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=39200; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=45504; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=45660; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 1b30911a4febbc0bc3ef45eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=24928; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=26248; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=27508; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=27244; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=28504; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=29764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: c6e76021e4d7e18bf42ea7b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=38572; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>; module=4139, inline_record=45040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: 38921b44a65bafd9f62a7715
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] asset::texture::desc::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::Gamma> @ 0x2451d22 -> 0x20e0240 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<asset::texture::desc::MipChain> > @ 0x2453122 -> 0x20e0310 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureOpts> @ 0x2453c42 -> 0x20e0420 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureAssetDesc> @ 0x2452e42 -> 0x20e0640 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> > @ 0x2453715 -> 0x20e0640 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::CompressionMethod> @ 0x2452002 -> 0x20e0830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$13::eq
// 信息 ID: fdc5383fd5fad3b0d2db4669
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=31952; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3176, inline_record=4308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$20::eq
// 信息 ID: 4c4f79a75662a7fc49300fa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=32040; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3176, inline_record=4396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$21::clone
// 信息 ID: 219d59a8400abca30c1f96b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>; module=3168, inline_record=3104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$26::eq
// 信息 ID: f327a0a11264337da122ca00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=31800; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3176, inline_record=4156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$27::clone
// 信息 ID: ed6c0f7cbc54949a205ce7ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=45760; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$11::clone<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>; module=3168, inline_record=2320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$31::eq
// 信息 ID: 8fe2dfe65507a8e3a1fb4f9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=30812; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3176, inline_record=3116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::desc::impl$3::eq
// 信息 ID: d20d4ccccae8a54e9c903a20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=1727, inline_record=31924; 不是独立调用边。
// [PDB 内联宿主] core::slice::cmp::impl$4::equal<asset::texture::desc::TextureAssetDesc,asset::texture::desc::TextureAssetDesc>; module=3176, inline_record=4280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
