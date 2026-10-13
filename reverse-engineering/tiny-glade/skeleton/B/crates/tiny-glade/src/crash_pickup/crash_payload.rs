// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/crash_pickup/crash_payload.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1839d43c8980c8812a24c16cb0a29adc88a43ebcdc4989881241aee8294f6c75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: crash_payload.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 10, 16, 49, 101, 157, 170, 188, 208, 240, 253
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":5500,"rva":3994432,"va":5372703552}
// 编译实例 RVAs: 0x3cf340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0570
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2100620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e88c0d2000","size":5,"soft_fail":false,"target_va":5374805360,"va":5372704735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2377844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e874482400","size":5,"soft_fail":false,"target_va":5375081488,"va":5372703639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39142868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e8d4455502","size":5,"soft_fail":false,"target_va":5411847168,"va":5372704295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t38836449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e8e1985002","size":5,"soft_fail":false,"target_va":5411540963,"va":5372704509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-3624052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e88cb3c8ff","size":5,"soft_fail":false,"target_va":5369080752,"va":5372704799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3625937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372703552,"raw_hex":"e82facc8ff","size":5,"soft_fail":false,"target_va":5369078928,"va":5372704860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":5700,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":5644,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5500,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5500,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6020,"parent_record_offset":5992,"ancestor_record_offset":5500,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":5932,"parent_record_offset":5904,"ancestor_record_offset":5500,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":5500,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":5500,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":5868,"parent_record_offset":5840,"ancestor_record_offset":5500,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":5500,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":5784,"parent_record_offset":5756,"ancestor_record_offset":5500,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":5500,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":5756,"parent_record_offset":5728,"ancestor_record_offset":5500,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":5728,"parent_record_offset":5700,"ancestor_record_offset":5500,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":5500,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":5904,"parent_record_offset":5868,"ancestor_record_offset":5500,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5500,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":6120,"parent_record_offset":5868,"ancestor_record_offset":5500,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 6, 26, 40, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x168D
// 首条位置: {"module":10,"inlinee":5773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":103468,"parent_record_offset":103444,"ancestor_record_offset":103444,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":103492,"parent_record_offset":103468,"ancestor_record_offset":103444,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 17, 40, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A4
// 首条位置: {"module":16,"inlinee":5796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":98064,"parent_record_offset":98036,"ancestor_record_offset":98036,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":98092,"parent_record_offset":98064,"ancestor_record_offset":98036,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":98120,"parent_record_offset":98092,"ancestor_record_offset":98036,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":98148,"parent_record_offset":98120,"ancestor_record_offset":98036,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 6, 17, 26, 40, 46, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19A3
// 首条位置: {"module":16,"inlinee":6563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":166892,"parent_record_offset":166864,"ancestor_record_offset":166864,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":87980,"parent_record_offset":87952,"ancestor_record_offset":87952,"inlinee":6564,"source_line":1886,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":88312,"parent_record_offset":88008,"ancestor_record_offset":87952,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":98092,"parent_record_offset":98064,"ancestor_record_offset":97652,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":98120,"parent_record_offset":98092,"ancestor_record_offset":97652,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":98148,"parent_record_offset":98120,"ancestor_record_offset":97652,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":98064,"parent_record_offset":98036,"ancestor_record_offset":97652,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":88196,"parent_record_offset":88088,"ancestor_record_offset":87952,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":88340,"parent_record_offset":88008,"ancestor_record_offset":87952,"inlinee":6496,"source_line":367,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":88088,"parent_record_offset":88008,"ancestor_record_offset":87952,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":88128,"parent_record_offset":88088,"ancestor_record_offset":87952,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166920,"parent_record_offset":166892,"ancestor_record_offset":166864,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":88008,"parent_record_offset":87980,"ancestor_record_offset":87952,"inlinee":6565,"source_line":1511,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":88220,"parent_record_offset":88196,"ancestor_record_offset":87952,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":88156,"parent_record_offset":88128,"ancestor_record_offset":87952,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":98036,"parent_record_offset":97708,"ancestor_record_offset":97652,"inlinee":5796,"source_line":40,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 40, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1667
// 首条位置: {"module":10,"inlinee":5735}
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
// 源码行: 6, 26, 40, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A00
// 首条位置: {"module":16,"inlinee":6656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":16,"record_offset":167340,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":168276,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":168244,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":168336,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":166276,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":167368,"parent_record_offset":166220,"ancestor_record_offset":166220,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":168740,"parent_record_offset":168712,"ancestor_record_offset":166220,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":168712,"parent_record_offset":168656,"ancestor_record_offset":166220,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":168684,"parent_record_offset":168656,"ancestor_record_offset":166220,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":168656,"parent_record_offset":168632,"ancestor_record_offset":166220,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":168632,"parent_record_offset":168608,"ancestor_record_offset":166220,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":167028,"parent_record_offset":166920,"ancestor_record_offset":166220,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":205724,"parent_record_offset":205696,"ancestor_record_offset":197856,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":16,"record_offset":167752,"parent_record_offset":167728,"ancestor_record_offset":166220,"inlinee":6556,"source_line":25,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":167108,"parent_record_offset":167084,"ancestor_record_offset":166220,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":16,"record_offset":205892,"parent_record_offset":205864,"ancestor_record_offset":197856,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=122
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":168584,"parent_record_offset":168560,"ancestor_record_offset":166220,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":198752,"parent_record_offset":198024,"ancestor_record_offset":197856,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":167132,"parent_record_offset":167108,"ancestor_record_offset":166220,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":167156,"parent_record_offset":167132,"ancestor_record_offset":166220,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":167084,"parent_record_offset":167056,"ancestor_record_offset":166220,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":16,"record_offset":172480,"parent_record_offset":172456,"ancestor_record_offset":170792,"inlinee":6659,"source_line":123,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":168408,"parent_record_offset":168384,"ancestor_record_offset":166220,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":206276,"parent_record_offset":206248,"ancestor_record_offset":197856,"inlinee":6675,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":205240,"parent_record_offset":205216,"ancestor_record_offset":197856,"inlinee":6672,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":168496,"parent_record_offset":168472,"ancestor_record_offset":166220,"inlinee":5157,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":168384,"parent_record_offset":168360,"ancestor_record_offset":166220,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":166836,"parent_record_offset":166328,"ancestor_record_offset":166220,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":199208,"parent_record_offset":199184,"ancestor_record_offset":197856,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$28::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":172932,"parent_record_offset":172908,"ancestor_record_offset":170792,"inlinee":5208,"source_line":1874,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$42::deserialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":206248,"parent_record_offset":206224,"ancestor_record_offset":197856,"inlinee":6674,"source_line":2990,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":168204,"parent_record_offset":168180,"ancestor_record_offset":166220,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=50
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166552,"parent_record_offset":166416,"ancestor_record_offset":166220,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":167452,"parent_record_offset":167408,"ancestor_record_offset":166220,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166416,"parent_record_offset":166328,"ancestor_record_offset":166220,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166476,"parent_record_offset":166416,"ancestor_record_offset":166220,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166328,"parent_record_offset":166276,"ancestor_record_offset":166220,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":167408,"parent_record_offset":167368,"ancestor_record_offset":166220,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":166920,"parent_record_offset":166892,"ancestor_record_offset":166220,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":166892,"parent_record_offset":166864,"ancestor_record_offset":166220,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=50
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":166576,"parent_record_offset":166552,"ancestor_record_offset":166220,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":166508,"parent_record_offset":166476,"ancestor_record_offset":166220,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":167056,"parent_record_offset":166920,"ancestor_record_offset":166220,"inlinee":5796,"source_line":40,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":166864,"parent_record_offset":166836,"ancestor_record_offset":166220,"inlinee":6563,"source_line":40,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 6, 26, 40, 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A01
// 首条位置: {"module":16,"inlinee":6657}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":176812,"parent_record_offset":176364,"ancestor_record_offset":176364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":169452,"parent_record_offset":169048,"ancestor_record_offset":169048,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":208712,"parent_record_offset":208204,"ancestor_record_offset":208204,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":169084,"parent_record_offset":169048,"ancestor_record_offset":169048,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":177060,"parent_record_offset":177036,"ancestor_record_offset":176364,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":177036,"parent_record_offset":176980,"ancestor_record_offset":176364,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":177008,"parent_record_offset":176980,"ancestor_record_offset":176364,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":176980,"parent_record_offset":176956,"ancestor_record_offset":176364,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":176956,"parent_record_offset":176932,"ancestor_record_offset":176364,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":210472,"parent_record_offset":210444,"ancestor_record_offset":208204,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::_::impl$0::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":16,"record_offset":169384,"parent_record_offset":169360,"ancestor_record_offset":169048,"inlinee":6556,"source_line":25,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":16,"record_offset":210640,"parent_record_offset":210612,"ancestor_record_offset":208204,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":176836,"parent_record_offset":176812,"ancestor_record_offset":176364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":16,"record_offset":176708,"parent_record_offset":176684,"ancestor_record_offset":176364,"inlinee":6659,"source_line":123,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":169360,"parent_record_offset":169112,"ancestor_record_offset":169048,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":208604,"parent_record_offset":208580,"ancestor_record_offset":208204,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":169300,"parent_record_offset":169160,"ancestor_record_offset":169048,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":169160,"parent_record_offset":169112,"ancestor_record_offset":169048,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":169224,"parent_record_offset":169160,"ancestor_record_offset":169048,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":169112,"parent_record_offset":169084,"ancestor_record_offset":169048,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":169324,"parent_record_offset":169300,"ancestor_record_offset":169048,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":169256,"parent_record_offset":169224,"ancestor_record_offset":169048,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=6
// 源码行: 6, 17, 26, 40, 46, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":36948,"rva":254544,"va":5368963664}
// 编译实例 RVAs: 0x3e250, 0x3e350, 0x3e7c0, 0x3eb80, 0x3ec80, 0x3eda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003e7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e8e3f5ffff","size":5,"soft_fail":false,"target_va":5368965056,"va":5368967640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2584293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368965056,"raw_hex":"e8e56e2700","size":5,"soft_fail":false,"target_va":5371549808,"va":5368965510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b6100
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4685186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368963920,"raw_hex":"e8827d4700","size":5,"soft_fail":false,"target_va":5373649152,"va":5368963961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b7b10
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4690655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368965056,"raw_hex":"e8df924700","size":5,"soft_fail":false,"target_va":5373655824,"va":5368965164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b9010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4697459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368963664,"raw_hex":"e873ad4700","size":5,"soft_fail":false,"target_va":5373661200,"va":5368963736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bb050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4702410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e8cac04700","size":5,"soft_fail":false,"target_va":5373669456,"va":5368967041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4707936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966016,"raw_hex":"e860d64700","size":5,"soft_fail":false,"target_va":5373674064,"va":5368966123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4707160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e858d34700","size":5,"soft_fail":false,"target_va":5373674384,"va":5368967219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4714177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e8c1ee4700","size":5,"soft_fail":false,"target_va":5373681440,"va":5368967258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4715555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966272,"raw_hex":"e823f44700","size":5,"soft_fail":false,"target_va":5373681904,"va":5368966344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404be1d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4716035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966016,"raw_hex":"e803f64700","size":5,"soft_fail":false,"target_va":5373682128,"va":5368966088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404beb00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4719239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368965056,"raw_hex":"e887024800","size":5,"soft_fail":false,"target_va":5373684480,"va":5368965236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404becc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4721152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368963664,"raw_hex":"e8000a4800","size":5,"soft_fail":false,"target_va":5373684928,"va":5368963771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c0300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4723795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e853144800","size":5,"soft_fail":false,"target_va":5373690624,"va":5368966824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c1710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4728970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e88a284800","size":5,"soft_fail":false,"target_va":5373695760,"va":5368966785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c17f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4730851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368965056,"raw_hex":"e8e32f4800","size":5,"soft_fail":false,"target_va":5373695984,"va":5368965128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t42354375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368963664,"raw_hex":"e8c7468602","size":5,"soft_fail":false,"target_va":5411318240,"va":5368963860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42509971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368966560,"raw_hex":"e893a68802","size":5,"soft_fail":false,"target_va":5411477664,"va":5368967688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t42513499","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5368963920,"raw_hex":"e95bb48802","size":5,"soft_fail":false,"target_va":5411477664,"va":5368964160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":54968,"parent_record_offset":47884,"ancestor_record_offset":47884,"inlinee":5726,"source_line":808,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":49756,"parent_record_offset":47884,"ancestor_record_offset":47884,"inlinee":5725,"source_line":794,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":38728,"parent_record_offset":38364,"ancestor_record_offset":38364,"inlinee":5418,"source_line":268,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37232,"parent_record_offset":36948,"ancestor_record_offset":36948,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":48172,"parent_record_offset":47884,"ancestor_record_offset":47884,"inlinee":5433,"source_line":420,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":38644,"parent_record_offset":38364,"ancestor_record_offset":38364,"inlinee":5432,"source_line":249,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37808,"parent_record_offset":36948,"ancestor_record_offset":36948,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37752,"parent_record_offset":36948,"ancestor_record_offset":36948,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37604,"parent_record_offset":37572,"ancestor_record_offset":36948,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37472,"parent_record_offset":37444,"ancestor_record_offset":36948,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37532,"parent_record_offset":37500,"ancestor_record_offset":36948,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37652,"parent_record_offset":37628,"ancestor_record_offset":36948,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37628,"parent_record_offset":37604,"ancestor_record_offset":36948,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":37500,"parent_record_offset":37472,"ancestor_record_offset":36948,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":37408,"parent_record_offset":37384,"ancestor_record_offset":36948,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":37572,"parent_record_offset":37408,"ancestor_record_offset":36948,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":37360,"parent_record_offset":37336,"ancestor_record_offset":36948,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":37692,"parent_record_offset":37408,"ancestor_record_offset":36948,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":37444,"parent_record_offset":37408,"ancestor_record_offset":36948,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":10,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":36948,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":55540,"parent_record_offset":55516,"ancestor_record_offset":47884,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":10,"record_offset":38692,"parent_record_offset":38668,"ancestor_record_offset":38364,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":10,"record_offset":39796,"parent_record_offset":38728,"ancestor_record_offset":38364,"inlinee":5748,"source_line":54,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37284,"parent_record_offset":37260,"ancestor_record_offset":36948,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":39316,"parent_record_offset":38728,"ancestor_record_offset":38364,"inlinee":5429,"source_line":1894,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37872,"parent_record_offset":37840,"ancestor_record_offset":36948,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37260,"parent_record_offset":37232,"ancestor_record_offset":36948,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":38668,"parent_record_offset":38644,"ancestor_record_offset":38364,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":37840,"parent_record_offset":37808,"ancestor_record_offset":36948,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":10,"record_offset":49780,"parent_record_offset":49756,"ancestor_record_offset":47884,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":37336,"parent_record_offset":37312,"ancestor_record_offset":36948,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":10,"record_offset":37312,"parent_record_offset":37284,"ancestor_record_offset":36948,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 17, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x167E
// 首条位置: {"module":10,"inlinee":5758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":10,"record_offset":41404,"parent_record_offset":40880,"ancestor_record_offset":40880,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":40916,"parent_record_offset":40880,"ancestor_record_offset":40880,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":41292,"parent_record_offset":40880,"ancestor_record_offset":40880,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":40880,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":10,"record_offset":41364,"parent_record_offset":41340,"ancestor_record_offset":40880,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41164,"parent_record_offset":41048,"ancestor_record_offset":40880,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41004,"parent_record_offset":40952,"ancestor_record_offset":40880,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41048,"parent_record_offset":41004,"ancestor_record_offset":40880,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41092,"parent_record_offset":41048,"ancestor_record_offset":40880,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40952,"parent_record_offset":40916,"ancestor_record_offset":40880,"inlinee":5760,"source_line":2020,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":40880,"inlinee":5765,"source_line":2037,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41492,"parent_record_offset":41468,"ancestor_record_offset":41440,"inlinee":5763,"source_line":2073,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":41440,"inlinee":5771,"source_line":2089,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":10,"record_offset":41188,"parent_record_offset":41164,"ancestor_record_offset":40880,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":10,"record_offset":41120,"parent_record_offset":41092,"ancestor_record_offset":40880,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":57816,"parent_record_offset":57792,"ancestor_record_offset":57320,"inlinee":5773,"source_line":26,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 17, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":40368,"rva":255168,"va":5368964288}
// 编译实例 RVAs: 0x3e4c0, 0x3f4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t42513087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8bfb28802","size":5,"soft_fail":false,"target_va":5411478016,"va":5368964924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005e400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t126617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e899ee0100","size":5,"soft_fail":false,"target_va":5369095168,"va":5368968546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005f540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t135154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8f20f0200","size":5,"soft_fail":false,"target_va":5369099584,"va":5368964425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t135914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8ea120200","size":5,"soft_fail":false,"target_va":5369100608,"va":5368964689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005fa60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t132069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e8e5030200","size":5,"soft_fail":false,"target_va":5369100896,"va":5368968822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140060a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t136059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e87b130200","size":5,"soft_fail":false,"target_va":5369104976,"va":5368968912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140061920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t139881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e869220200","size":5,"soft_fail":false,"target_va":5369108768,"va":5368968882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400622a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t142343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e8072c0200","size":5,"soft_fail":false,"target_va":5369111200,"va":5368968852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400656d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t159904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8a0700200","size":5,"soft_fail":false,"target_va":5369124560,"va":5368964651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14006d410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t187462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e846dc0200","size":5,"soft_fail":false,"target_va":5369156624,"va":5368969157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6116650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e82a555d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5368964849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42345443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8e3238602","size":5,"soft_fail":false,"target_va":5411309856,"va":5368964408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42346719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368964288,"raw_hex":"e8df288602","size":5,"soft_fail":false,"target_va":5411311520,"va":5368964796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42878203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e8fb448e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5368968960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368968384,"raw_hex":"e821e8ffff","size":5,"soft_fail":false,"target_va":5368962960,"va":5368969066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40588,"parent_record_offset":40368,"ancestor_record_offset":40368,"inlinee":5750,"source_line":1855,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":42028,"parent_record_offset":42004,"ancestor_record_offset":40368,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":42004,"parent_record_offset":41980,"ancestor_record_offset":40368,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":41980,"parent_record_offset":41956,"ancestor_record_offset":40368,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":41956,"parent_record_offset":41932,"ancestor_record_offset":40368,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":41860,"parent_record_offset":40588,"ancestor_record_offset":40368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":10,"record_offset":41404,"parent_record_offset":40880,"ancestor_record_offset":40368,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":40916,"parent_record_offset":40880,"ancestor_record_offset":40368,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":41292,"parent_record_offset":40880,"ancestor_record_offset":40368,"inlinee":5585,"source_line":2117,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":10,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":40368,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":10,"record_offset":41364,"parent_record_offset":41340,"ancestor_record_offset":40368,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40820,"parent_record_offset":40700,"ancestor_record_offset":40368,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41004,"parent_record_offset":40952,"ancestor_record_offset":40368,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40700,"parent_record_offset":40588,"ancestor_record_offset":40368,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40752,"parent_record_offset":40700,"ancestor_record_offset":40368,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$13::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":40952,"parent_record_offset":40916,"ancestor_record_offset":40368,"inlinee":5760,"source_line":2020,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$14::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":40368,"inlinee":5765,"source_line":2037,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$16::variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41492,"parent_record_offset":41468,"ancestor_record_offset":40368,"inlinee":5763,"source_line":2073,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$17::newtype_variant_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":10,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":40368,"inlinee":5771,"source_line":2089,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":10,"record_offset":40844,"parent_record_offset":40820,"ancestor_record_offset":40368,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":10,"record_offset":40780,"parent_record_offset":40752,"ancestor_record_offset":40368,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":57816,"parent_record_offset":57792,"ancestor_record_offset":56772,"inlinee":5773,"source_line":26,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":40880,"parent_record_offset":40588,"ancestor_record_offset":40368,"inlinee":5758,"source_line":70,"archive":"_pdb/B/modules/00010.jsonl.gz"}
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
// 首条位置: {"module":170,"record_offset":31568,"rva":4007920,"va":5372717040}
// 编译实例 RVAs: 0x3d27f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10170971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372717040,"raw_hex":"e85b329b00","size":5,"soft_fail":false,"target_va":5382888144,"va":5372717168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10175353","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372717040,"raw_hex":"e979439b00","size":5,"soft_fail":false,"target_va":5382892432,"va":5372717074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":31704,"parent_record_offset":31568,"ancestor_record_offset":31568,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34E3
// 首条位置: {"module":157,"inlinee":13539}
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
// 源码行: 26, 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":43628,"rva":256464,"va":5368965584}
// 编译实例 RVAs: 0x3e9d0, 0x3f900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42347123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368969472,"raw_hex":"e8732a8602","size":5,"soft_fail":false,"target_va":5411316864,"va":5368969736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: crash_payload.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42881333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368965584,"raw_hex":"e835518e02","size":5,"soft_fail":false,"target_va":5411847152,"va":5368965814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":10,"record_offset":43820,"parent_record_offset":43628,"ancestor_record_offset":43628,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":10,"record_offset":43848,"parent_record_offset":43820,"ancestor_record_offset":43628,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":10,"record_offset":43876,"parent_record_offset":43848,"ancestor_record_offset":43628,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":10,"record_offset":43904,"parent_record_offset":43876,"ancestor_record_offset":43628,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34E1
// 首条位置: {"module":157,"inlinee":13537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":43080,"parent_record_offset":42860,"ancestor_record_offset":42860,"inlinee":13528,"source_line":1730,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":42860,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":43756,"parent_record_offset":43732,"ancestor_record_offset":42860,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46976,"parent_record_offset":46952,"ancestor_record_offset":42860,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":43708,"parent_record_offset":43532,"ancestor_record_offset":42860,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46952,"parent_record_offset":46896,"ancestor_record_offset":42860,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":43472,"parent_record_offset":43444,"ancestor_record_offset":42860,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46924,"parent_record_offset":46896,"ancestor_record_offset":42860,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46896,"parent_record_offset":46872,"ancestor_record_offset":42860,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":43532,"parent_record_offset":43500,"ancestor_record_offset":42860,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":43500,"parent_record_offset":43472,"ancestor_record_offset":42860,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46872,"parent_record_offset":46848,"ancestor_record_offset":42860,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":43568,"parent_record_offset":43532,"ancestor_record_offset":42860,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":46024,"parent_record_offset":45984,"ancestor_record_offset":42860,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":43404,"parent_record_offset":43368,"ancestor_record_offset":42860,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":43368,"parent_record_offset":43296,"ancestor_record_offset":42860,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":43340,"parent_record_offset":43296,"ancestor_record_offset":42860,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":43444,"parent_record_offset":43404,"ancestor_record_offset":42860,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":43260,"parent_record_offset":43228,"ancestor_record_offset":42860,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":43296,"parent_record_offset":43260,"ancestor_record_offset":42860,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":42860,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":43616,"parent_record_offset":43592,"ancestor_record_offset":42860,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":43832,"parent_record_offset":43808,"ancestor_record_offset":42860,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":43664,"parent_record_offset":43640,"ancestor_record_offset":42860,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":46524,"parent_record_offset":46480,"ancestor_record_offset":42860,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":46480,"parent_record_offset":46024,"ancestor_record_offset":42860,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":46624,"parent_record_offset":46024,"ancestor_record_offset":42860,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":43640,"parent_record_offset":43616,"ancestor_record_offset":42860,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":45648,"parent_record_offset":44772,"ancestor_record_offset":42860,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":43808,"parent_record_offset":43404,"ancestor_record_offset":42860,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":46824,"parent_record_offset":45808,"ancestor_record_offset":42860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":46584,"parent_record_offset":46552,"ancestor_record_offset":42860,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":46552,"parent_record_offset":46524,"ancestor_record_offset":42860,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":46740,"parent_record_offset":46708,"ancestor_record_offset":42860,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46708,"parent_record_offset":46672,"ancestor_record_offset":42860,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46784,"parent_record_offset":46708,"ancestor_record_offset":42860,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45808,"parent_record_offset":45768,"ancestor_record_offset":42860,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45768,"parent_record_offset":45696,"ancestor_record_offset":42860,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":43160,"parent_record_offset":43124,"ancestor_record_offset":42860,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":43124,"parent_record_offset":43080,"ancestor_record_offset":42860,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":157,"record_offset":43196,"parent_record_offset":43160,"ancestor_record_offset":42860,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":157,"record_offset":43228,"parent_record_offset":43196,"ancestor_record_offset":42860,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":44736,"parent_record_offset":44664,"ancestor_record_offset":42860,"inlinee":13539,"source_line":17,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":44772,"parent_record_offset":44664,"ancestor_record_offset":42860,"inlinee":13541,"source_line":40,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":45696,"parent_record_offset":45648,"ancestor_record_offset":42860,"inlinee":13543,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34E5
// 首条位置: {"module":157,"inlinee":13541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":45648,"parent_record_offset":44772,"ancestor_record_offset":44772,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":48876,"parent_record_offset":44772,"ancestor_record_offset":44772,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":44844,"parent_record_offset":44772,"ancestor_record_offset":44772,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":44772,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":45508,"parent_record_offset":45484,"ancestor_record_offset":44772,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46976,"parent_record_offset":46952,"ancestor_record_offset":44772,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":45460,"parent_record_offset":45276,"ancestor_record_offset":44772,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46952,"parent_record_offset":46896,"ancestor_record_offset":44772,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":45200,"parent_record_offset":45164,"ancestor_record_offset":44772,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46924,"parent_record_offset":46896,"ancestor_record_offset":44772,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46896,"parent_record_offset":46872,"ancestor_record_offset":44772,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":45276,"parent_record_offset":45236,"ancestor_record_offset":44772,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":45236,"parent_record_offset":45200,"ancestor_record_offset":44772,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46872,"parent_record_offset":46848,"ancestor_record_offset":44772,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":45320,"parent_record_offset":45276,"ancestor_record_offset":44772,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":46024,"parent_record_offset":45984,"ancestor_record_offset":44772,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":45124,"parent_record_offset":45088,"ancestor_record_offset":44772,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":45088,"parent_record_offset":45016,"ancestor_record_offset":44772,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45060,"parent_record_offset":45016,"ancestor_record_offset":44772,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45164,"parent_record_offset":45124,"ancestor_record_offset":44772,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":44980,"parent_record_offset":44948,"ancestor_record_offset":44772,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45016,"parent_record_offset":44980,"ancestor_record_offset":44772,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":45344,"parent_record_offset":45320,"ancestor_record_offset":44772,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":45368,"parent_record_offset":45344,"ancestor_record_offset":44772,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":45584,"parent_record_offset":45560,"ancestor_record_offset":44772,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":45416,"parent_record_offset":45392,"ancestor_record_offset":44772,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":46524,"parent_record_offset":46480,"ancestor_record_offset":44772,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":46480,"parent_record_offset":46024,"ancestor_record_offset":44772,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":46624,"parent_record_offset":46024,"ancestor_record_offset":44772,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":44772,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":46672,"parent_record_offset":45808,"ancestor_record_offset":44772,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":45560,"parent_record_offset":45124,"ancestor_record_offset":44772,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":46824,"parent_record_offset":45808,"ancestor_record_offset":44772,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":46584,"parent_record_offset":46552,"ancestor_record_offset":44772,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":46552,"parent_record_offset":46524,"ancestor_record_offset":44772,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":46740,"parent_record_offset":46708,"ancestor_record_offset":44772,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46708,"parent_record_offset":46672,"ancestor_record_offset":44772,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46784,"parent_record_offset":46708,"ancestor_record_offset":44772,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45808,"parent_record_offset":45768,"ancestor_record_offset":44772,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45768,"parent_record_offset":45696,"ancestor_record_offset":44772,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":44880,"parent_record_offset":44844,"ancestor_record_offset":44772,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":47032,"parent_record_offset":45696,"ancestor_record_offset":44772,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":157,"record_offset":44916,"parent_record_offset":44880,"ancestor_record_offset":44772,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":157,"record_offset":44948,"parent_record_offset":44916,"ancestor_record_offset":44772,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":157,"record_offset":45696,"parent_record_offset":45648,"ancestor_record_offset":44772,"inlinee":13543,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34E7
// 首条位置: {"module":157,"inlinee":13543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":48556,"parent_record_offset":45696,"ancestor_record_offset":45696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$29::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45768,"parent_record_offset":45696,"ancestor_record_offset":45696,"inlinee":13518,"source_line":256,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":157,"record_offset":47032,"parent_record_offset":45696,"ancestor_record_offset":45696,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46404,"parent_record_offset":46380,"ancestor_record_offset":45696,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46428,"parent_record_offset":46404,"ancestor_record_offset":45696,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46976,"parent_record_offset":46952,"ancestor_record_offset":45696,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46380,"parent_record_offset":46196,"ancestor_record_offset":45696,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":46952,"parent_record_offset":46896,"ancestor_record_offset":45696,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46120,"parent_record_offset":46084,"ancestor_record_offset":45696,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46924,"parent_record_offset":46896,"ancestor_record_offset":45696,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46896,"parent_record_offset":46872,"ancestor_record_offset":45696,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46196,"parent_record_offset":46156,"ancestor_record_offset":45696,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46156,"parent_record_offset":46120,"ancestor_record_offset":45696,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46872,"parent_record_offset":46848,"ancestor_record_offset":45696,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":46240,"parent_record_offset":46196,"ancestor_record_offset":45696,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":46024,"parent_record_offset":45984,"ancestor_record_offset":45696,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":47312,"parent_record_offset":47276,"ancestor_record_offset":45696,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":45984,"parent_record_offset":45908,"ancestor_record_offset":45696,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45956,"parent_record_offset":45908,"ancestor_record_offset":45696,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":46084,"parent_record_offset":46024,"ancestor_record_offset":45696,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45868,"parent_record_offset":45808,"ancestor_record_offset":45696,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":45908,"parent_record_offset":45868,"ancestor_record_offset":45696,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":46264,"parent_record_offset":46240,"ancestor_record_offset":45696,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":46288,"parent_record_offset":46264,"ancestor_record_offset":45696,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":47772,"parent_record_offset":47748,"ancestor_record_offset":45696,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":46336,"parent_record_offset":46312,"ancestor_record_offset":45696,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":157,"record_offset":46524,"parent_record_offset":46480,"ancestor_record_offset":45696,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":157,"record_offset":46480,"parent_record_offset":46024,"ancestor_record_offset":45696,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":157,"record_offset":46624,"parent_record_offset":46024,"ancestor_record_offset":45696,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":45696,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":157,"record_offset":46672,"parent_record_offset":45808,"ancestor_record_offset":45696,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":47748,"parent_record_offset":47312,"ancestor_record_offset":45696,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":46824,"parent_record_offset":45808,"ancestor_record_offset":45696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":157,"record_offset":46584,"parent_record_offset":46552,"ancestor_record_offset":45696,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":157,"record_offset":46552,"parent_record_offset":46524,"ancestor_record_offset":45696,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":157,"record_offset":46740,"parent_record_offset":46708,"ancestor_record_offset":45696,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46708,"parent_record_offset":46672,"ancestor_record_offset":45696,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":157,"record_offset":46784,"parent_record_offset":46708,"ancestor_record_offset":45696,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$28::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":157,"record_offset":45808,"parent_record_offset":45768,"ancestor_record_offset":45696,"inlinee":13520,"source_line":181,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":47068,"parent_record_offset":47032,"ancestor_record_offset":45696,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":157,"record_offset":47104,"parent_record_offset":47068,"ancestor_record_offset":45696,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":157,"record_offset":47136,"parent_record_offset":47104,"ancestor_record_offset":45696,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x166B
// 首条位置: {"module":10,"inlinee":5739}
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
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A6
// 首条位置: {"module":16,"inlinee":5798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":102140,"parent_record_offset":102112,"ancestor_record_offset":102112,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":102164,"parent_record_offset":102140,"ancestor_record_offset":102112,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102188,"parent_record_offset":102164,"ancestor_record_offset":102112,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102212,"parent_record_offset":102188,"ancestor_record_offset":102112,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19DE
// 首条位置: {"module":16,"inlinee":6622}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":117112,"parent_record_offset":117084,"ancestor_record_offset":117084,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":117248,"parent_record_offset":117140,"ancestor_record_offset":117084,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":117328,"parent_record_offset":117304,"ancestor_record_offset":117084,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117352,"parent_record_offset":117328,"ancestor_record_offset":117084,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117376,"parent_record_offset":117352,"ancestor_record_offset":117084,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":117304,"parent_record_offset":117276,"ancestor_record_offset":117084,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":117192,"parent_record_offset":117140,"ancestor_record_offset":117084,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":117140,"parent_record_offset":117112,"ancestor_record_offset":117084,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":117216,"parent_record_offset":117192,"ancestor_record_offset":117084,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":117276,"parent_record_offset":117140,"ancestor_record_offset":117084,"inlinee":5802,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x166F
// 首条位置: {"module":10,"inlinee":5743}
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
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19DC
// 首条位置: {"module":16,"inlinee":6620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":16,"record_offset":117560,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":118500,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":118436,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":118696,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":116492,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":117588,"parent_record_offset":116424,"ancestor_record_offset":116424,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":118644,"parent_record_offset":118620,"ancestor_record_offset":116424,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":118620,"parent_record_offset":118596,"ancestor_record_offset":116424,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":118996,"parent_record_offset":118968,"ancestor_record_offset":116424,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":118596,"parent_record_offset":118572,"ancestor_record_offset":116424,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":118572,"parent_record_offset":118548,"ancestor_record_offset":116424,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":117248,"parent_record_offset":117140,"ancestor_record_offset":116424,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":117328,"parent_record_offset":117304,"ancestor_record_offset":116424,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":118524,"parent_record_offset":118500,"ancestor_record_offset":116424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117352,"parent_record_offset":117328,"ancestor_record_offset":116424,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117376,"parent_record_offset":117352,"ancestor_record_offset":116424,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":117304,"parent_record_offset":117276,"ancestor_record_offset":116424,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":118768,"parent_record_offset":118744,"ancestor_record_offset":116424,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":118744,"parent_record_offset":118720,"ancestor_record_offset":116424,"inlinee":5157,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":117056,"parent_record_offset":116544,"ancestor_record_offset":116424,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":117972,"parent_record_offset":117948,"ancestor_record_offset":116424,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":116772,"parent_record_offset":116636,"ancestor_record_offset":116424,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":117672,"parent_record_offset":117628,"ancestor_record_offset":116424,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":116636,"parent_record_offset":116544,"ancestor_record_offset":116424,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":116696,"parent_record_offset":116636,"ancestor_record_offset":116424,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":116544,"parent_record_offset":116492,"ancestor_record_offset":116424,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":117628,"parent_record_offset":117588,"ancestor_record_offset":116424,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":117140,"parent_record_offset":117112,"ancestor_record_offset":116424,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":117112,"parent_record_offset":117084,"ancestor_record_offset":116424,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":116796,"parent_record_offset":116772,"ancestor_record_offset":116424,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":116728,"parent_record_offset":116696,"ancestor_record_offset":116424,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":117084,"parent_record_offset":117056,"ancestor_record_offset":116424,"inlinee":6622,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":117276,"parent_record_offset":117140,"ancestor_record_offset":116424,"inlinee":5802,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19DF
// 首条位置: {"module":16,"inlinee":6623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":120312,"parent_record_offset":119616,"ancestor_record_offset":119616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":119692,"parent_record_offset":119616,"ancestor_record_offset":119616,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":119664,"parent_record_offset":119616,"ancestor_record_offset":119616,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":120488,"parent_record_offset":120464,"ancestor_record_offset":119616,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":120464,"parent_record_offset":120408,"ancestor_record_offset":119616,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":120436,"parent_record_offset":120408,"ancestor_record_offset":119616,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":120408,"parent_record_offset":120384,"ancestor_record_offset":119616,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":120384,"parent_record_offset":120360,"ancestor_record_offset":119616,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":120336,"parent_record_offset":120312,"ancestor_record_offset":119616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":120276,"parent_record_offset":119760,"ancestor_record_offset":119616,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":119968,"parent_record_offset":119844,"ancestor_record_offset":119616,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":119844,"parent_record_offset":119760,"ancestor_record_offset":119616,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":119896,"parent_record_offset":119844,"ancestor_record_offset":119616,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":119760,"parent_record_offset":119720,"ancestor_record_offset":119616,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":119992,"parent_record_offset":119968,"ancestor_record_offset":119616,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":119924,"parent_record_offset":119896,"ancestor_record_offset":119616,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19CB
// 首条位置: {"module":16,"inlinee":6603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":101952,"parent_record_offset":101924,"ancestor_record_offset":101924,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":102084,"parent_record_offset":101980,"ancestor_record_offset":101924,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":102164,"parent_record_offset":102140,"ancestor_record_offset":101924,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102188,"parent_record_offset":102164,"ancestor_record_offset":101924,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102212,"parent_record_offset":102188,"ancestor_record_offset":101924,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":102140,"parent_record_offset":102112,"ancestor_record_offset":101924,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":102028,"parent_record_offset":101980,"ancestor_record_offset":101924,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101980,"parent_record_offset":101952,"ancestor_record_offset":101924,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":102052,"parent_record_offset":102028,"ancestor_record_offset":101924,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":102112,"parent_record_offset":101980,"ancestor_record_offset":101924,"inlinee":5798,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1669
// 首条位置: {"module":10,"inlinee":5737}
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
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19C6
// 首条位置: {"module":16,"inlinee":6598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":16,"record_offset":102396,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":103332,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":103300,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":103420,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":101328,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":102424,"parent_record_offset":101260,"ancestor_record_offset":101260,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":102084,"parent_record_offset":101980,"ancestor_record_offset":101260,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":102164,"parent_record_offset":102140,"ancestor_record_offset":101260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102188,"parent_record_offset":102164,"ancestor_record_offset":101260,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":102212,"parent_record_offset":102188,"ancestor_record_offset":101260,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":102140,"parent_record_offset":102112,"ancestor_record_offset":101260,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":103492,"parent_record_offset":103468,"ancestor_record_offset":101260,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":103468,"parent_record_offset":103444,"ancestor_record_offset":101260,"inlinee":5746,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":101896,"parent_record_offset":101384,"ancestor_record_offset":101260,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101612,"parent_record_offset":101476,"ancestor_record_offset":101260,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":102508,"parent_record_offset":102464,"ancestor_record_offset":101260,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101476,"parent_record_offset":101384,"ancestor_record_offset":101260,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101536,"parent_record_offset":101476,"ancestor_record_offset":101260,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101384,"parent_record_offset":101328,"ancestor_record_offset":101260,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":102464,"parent_record_offset":102424,"ancestor_record_offset":101260,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":101980,"parent_record_offset":101952,"ancestor_record_offset":101260,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":101952,"parent_record_offset":101924,"ancestor_record_offset":101260,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":101636,"parent_record_offset":101612,"ancestor_record_offset":101260,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":101568,"parent_record_offset":101536,"ancestor_record_offset":101260,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":102808,"parent_record_offset":102784,"ancestor_record_offset":101260,"inlinee":5773,"source_line":40,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":102112,"parent_record_offset":101980,"ancestor_record_offset":101260,"inlinee":5798,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":101924,"parent_record_offset":101896,"ancestor_record_offset":101260,"inlinee":6603,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19CE
// 首条位置: {"module":16,"inlinee":6606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":103752,"parent_record_offset":103704,"ancestor_record_offset":103704,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":104024,"parent_record_offset":103780,"ancestor_record_offset":103704,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":103964,"parent_record_offset":103824,"ancestor_record_offset":103704,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":103824,"parent_record_offset":103780,"ancestor_record_offset":103704,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":103888,"parent_record_offset":103824,"ancestor_record_offset":103704,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":103780,"parent_record_offset":103752,"ancestor_record_offset":103704,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":103988,"parent_record_offset":103964,"ancestor_record_offset":103704,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":103920,"parent_record_offset":103888,"ancestor_record_offset":103704,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":104048,"parent_record_offset":104024,"ancestor_record_offset":103704,"inlinee":5773,"source_line":40,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A8
// 首条位置: {"module":16,"inlinee":5800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":112924,"parent_record_offset":112896,"ancestor_record_offset":112896,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":112948,"parent_record_offset":112924,"ancestor_record_offset":112896,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112972,"parent_record_offset":112948,"ancestor_record_offset":112896,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112996,"parent_record_offset":112972,"ancestor_record_offset":112896,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D9
// 首条位置: {"module":16,"inlinee":6617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":112736,"parent_record_offset":112708,"ancestor_record_offset":112708,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":112868,"parent_record_offset":112764,"ancestor_record_offset":112708,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":112948,"parent_record_offset":112924,"ancestor_record_offset":112708,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112972,"parent_record_offset":112948,"ancestor_record_offset":112708,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112996,"parent_record_offset":112972,"ancestor_record_offset":112708,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":112924,"parent_record_offset":112896,"ancestor_record_offset":112708,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112812,"parent_record_offset":112764,"ancestor_record_offset":112708,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112764,"parent_record_offset":112736,"ancestor_record_offset":112708,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":112836,"parent_record_offset":112812,"ancestor_record_offset":112708,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":112896,"parent_record_offset":112764,"ancestor_record_offset":112708,"inlinee":5800,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x166D
// 首条位置: {"module":10,"inlinee":5741}
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
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D7
// 首条位置: {"module":16,"inlinee":6615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":16,"record_offset":113180,"parent_record_offset":112072,"ancestor_record_offset":112072,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":113984,"parent_record_offset":112072,"ancestor_record_offset":112072,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":114048,"parent_record_offset":112072,"ancestor_record_offset":112072,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":112128,"parent_record_offset":112072,"ancestor_record_offset":112072,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":113208,"parent_record_offset":112072,"ancestor_record_offset":112072,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":112868,"parent_record_offset":112764,"ancestor_record_offset":112072,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":112948,"parent_record_offset":112924,"ancestor_record_offset":112072,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112972,"parent_record_offset":112948,"ancestor_record_offset":112072,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":112996,"parent_record_offset":112972,"ancestor_record_offset":112072,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":112924,"parent_record_offset":112896,"ancestor_record_offset":112072,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":114120,"parent_record_offset":114096,"ancestor_record_offset":112072,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":114096,"parent_record_offset":114072,"ancestor_record_offset":112072,"inlinee":6619,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":112680,"parent_record_offset":112176,"ancestor_record_offset":112072,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":113556,"parent_record_offset":113532,"ancestor_record_offset":112072,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112396,"parent_record_offset":112260,"ancestor_record_offset":112072,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":113268,"parent_record_offset":113236,"ancestor_record_offset":112072,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112260,"parent_record_offset":112176,"ancestor_record_offset":112072,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112320,"parent_record_offset":112260,"ancestor_record_offset":112072,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112176,"parent_record_offset":112128,"ancestor_record_offset":112072,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":113236,"parent_record_offset":113208,"ancestor_record_offset":112072,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":112764,"parent_record_offset":112736,"ancestor_record_offset":112072,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":112736,"parent_record_offset":112708,"ancestor_record_offset":112072,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":112420,"parent_record_offset":112396,"ancestor_record_offset":112072,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":112352,"parent_record_offset":112320,"ancestor_record_offset":112072,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":112896,"parent_record_offset":112764,"ancestor_record_offset":112072,"inlinee":5800,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":112708,"parent_record_offset":112680,"ancestor_record_offset":112072,"inlinee":6617,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19DA
// 首条位置: {"module":16,"inlinee":6618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":114264,"parent_record_offset":114220,"ancestor_record_offset":114220,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":114296,"parent_record_offset":114220,"ancestor_record_offset":114220,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":114856,"parent_record_offset":114340,"ancestor_record_offset":114220,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":114552,"parent_record_offset":114428,"ancestor_record_offset":114220,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":114428,"parent_record_offset":114340,"ancestor_record_offset":114220,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":114480,"parent_record_offset":114428,"ancestor_record_offset":114220,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":114340,"parent_record_offset":114296,"ancestor_record_offset":114220,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":114576,"parent_record_offset":114552,"ancestor_record_offset":114220,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":114508,"parent_record_offset":114480,"ancestor_record_offset":114220,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D5
// 首条位置: {"module":16,"inlinee":6613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":107108,"parent_record_offset":107080,"ancestor_record_offset":107080,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":107132,"parent_record_offset":107108,"ancestor_record_offset":107080,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107156,"parent_record_offset":107132,"ancestor_record_offset":107080,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107180,"parent_record_offset":107156,"ancestor_record_offset":107080,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D3
// 首条位置: {"module":16,"inlinee":6611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":106916,"parent_record_offset":106888,"ancestor_record_offset":106888,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":107052,"parent_record_offset":106944,"ancestor_record_offset":106888,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":107132,"parent_record_offset":107108,"ancestor_record_offset":106888,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107156,"parent_record_offset":107132,"ancestor_record_offset":106888,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107180,"parent_record_offset":107156,"ancestor_record_offset":106888,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":107108,"parent_record_offset":107080,"ancestor_record_offset":106888,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106996,"parent_record_offset":106944,"ancestor_record_offset":106888,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106944,"parent_record_offset":106916,"ancestor_record_offset":106888,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":107020,"parent_record_offset":106996,"ancestor_record_offset":106888,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":107080,"parent_record_offset":106944,"ancestor_record_offset":106888,"inlinee":6613,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1671
// 首条位置: {"module":10,"inlinee":5745}
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
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D1
// 首条位置: {"module":16,"inlinee":6609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":16,"record_offset":107364,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":108556,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":108268,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":108332,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":106296,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":107392,"parent_record_offset":106232,"ancestor_record_offset":106232,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":108760,"parent_record_offset":108736,"ancestor_record_offset":106232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":108736,"parent_record_offset":108668,"ancestor_record_offset":106232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":108704,"parent_record_offset":108668,"ancestor_record_offset":106232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":108668,"parent_record_offset":108640,"ancestor_record_offset":106232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":108640,"parent_record_offset":108612,"ancestor_record_offset":106232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":16,"record_offset":107052,"parent_record_offset":106944,"ancestor_record_offset":106232,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":107132,"parent_record_offset":107108,"ancestor_record_offset":106232,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":108584,"parent_record_offset":108556,"ancestor_record_offset":106232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107156,"parent_record_offset":107132,"ancestor_record_offset":106232,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":107180,"parent_record_offset":107156,"ancestor_record_offset":106232,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":107108,"parent_record_offset":107080,"ancestor_record_offset":106232,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":16,"record_offset":108404,"parent_record_offset":108380,"ancestor_record_offset":106232,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":108380,"parent_record_offset":108356,"ancestor_record_offset":106232,"inlinee":5157,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":106860,"parent_record_offset":106348,"ancestor_record_offset":106232,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":16,"record_offset":107776,"parent_record_offset":107752,"ancestor_record_offset":106232,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106576,"parent_record_offset":106440,"ancestor_record_offset":106232,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":107476,"parent_record_offset":107432,"ancestor_record_offset":106232,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106440,"parent_record_offset":106348,"ancestor_record_offset":106232,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106500,"parent_record_offset":106440,"ancestor_record_offset":106232,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106348,"parent_record_offset":106296,"ancestor_record_offset":106232,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":107432,"parent_record_offset":107392,"ancestor_record_offset":106232,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":16,"record_offset":106944,"parent_record_offset":106916,"ancestor_record_offset":106232,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":16,"record_offset":106916,"parent_record_offset":106888,"ancestor_record_offset":106232,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":106600,"parent_record_offset":106576,"ancestor_record_offset":106232,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":16,"record_offset":106532,"parent_record_offset":106500,"ancestor_record_offset":106232,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":107080,"parent_record_offset":106944,"ancestor_record_offset":106232,"inlinee":6613,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":16,"record_offset":106888,"parent_record_offset":106860,"ancestor_record_offset":106232,"inlinee":6611,"source_line":70,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19D6
// 首条位置: {"module":16,"inlinee":6614}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":110104,"parent_record_offset":109952,"ancestor_record_offset":109952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":16,"record_offset":110016,"parent_record_offset":109952,"ancestor_record_offset":109952,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":16,"record_offset":109988,"parent_record_offset":109952,"ancestor_record_offset":109952,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":110280,"parent_record_offset":110256,"ancestor_record_offset":109952,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":16,"record_offset":110256,"parent_record_offset":110200,"ancestor_record_offset":109952,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":110228,"parent_record_offset":110200,"ancestor_record_offset":109952,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":110200,"parent_record_offset":110176,"ancestor_record_offset":109952,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":16,"record_offset":110176,"parent_record_offset":110152,"ancestor_record_offset":109952,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":16,"record_offset":110128,"parent_record_offset":110104,"ancestor_record_offset":109952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16AA
// 首条位置: {"module":16,"inlinee":5802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":16,"record_offset":117304,"parent_record_offset":117276,"ancestor_record_offset":117276,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":16,"record_offset":117328,"parent_record_offset":117304,"ancestor_record_offset":117276,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117352,"parent_record_offset":117328,"ancestor_record_offset":117276,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":16,"record_offset":117376,"parent_record_offset":117352,"ancestor_record_offset":117276,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A2
// 首条位置: {"module":240,"inlinee":5794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":240,"record_offset":66016,"parent_record_offset":65968,"ancestor_record_offset":65968,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":66368,"parent_record_offset":66340,"ancestor_record_offset":65968,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":66396,"parent_record_offset":66368,"ancestor_record_offset":65968,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":240,"record_offset":66340,"parent_record_offset":66308,"ancestor_record_offset":65968,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":66244,"parent_record_offset":66212,"ancestor_record_offset":65968,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":66308,"parent_record_offset":66276,"ancestor_record_offset":65968,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":240,"record_offset":66276,"parent_record_offset":66244,"ancestor_record_offset":65968,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":240,"record_offset":66176,"parent_record_offset":66144,"ancestor_record_offset":65968,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":240,"record_offset":66144,"parent_record_offset":66112,"ancestor_record_offset":65968,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":240,"record_offset":66080,"parent_record_offset":66048,"ancestor_record_offset":65968,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":240,"record_offset":66112,"parent_record_offset":66080,"ancestor_record_offset":65968,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":240,"record_offset":66048,"parent_record_offset":66016,"ancestor_record_offset":65968,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":240,"record_offset":66212,"parent_record_offset":66176,"ancestor_record_offset":65968,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":240,"record_offset":66480,"parent_record_offset":66452,"ancestor_record_offset":65968,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":240,"record_offset":66452,"parent_record_offset":66176,"ancestor_record_offset":65968,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D57
// 首条位置: {"module":101,"inlinee":11607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":62556,"parent_record_offset":62520,"ancestor_record_offset":62520,"inlinee":8549,"source_line":2064,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F06
// 首条位置: {"module":253,"inlinee":16134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":253,"record_offset":1090628,"parent_record_offset":1090564,"ancestor_record_offset":1090564,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":1090744,"parent_record_offset":1090564,"ancestor_record_offset":1090564,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":253,"record_offset":1091620,"parent_record_offset":1090564,"ancestor_record_offset":1090564,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":253,"record_offset":1090792,"parent_record_offset":1090564,"ancestor_record_offset":1090564,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":1091240,"parent_record_offset":1091216,"ancestor_record_offset":1090564,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":253,"record_offset":1091216,"parent_record_offset":1091160,"ancestor_record_offset":1090564,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":253,"record_offset":1091984,"parent_record_offset":1091956,"ancestor_record_offset":1090564,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":253,"record_offset":1091188,"parent_record_offset":1091160,"ancestor_record_offset":1090564,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":253,"record_offset":1091160,"parent_record_offset":1091136,"ancestor_record_offset":1090564,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":253,"record_offset":1091136,"parent_record_offset":1091112,"ancestor_record_offset":1090564,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":253,"record_offset":1092012,"parent_record_offset":1091984,"ancestor_record_offset":1090564,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":253,"record_offset":1090652,"parent_record_offset":1090628,"ancestor_record_offset":1090564,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":253,"record_offset":1092036,"parent_record_offset":1092012,"ancestor_record_offset":1090564,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":1090764,"parent_record_offset":1090744,"ancestor_record_offset":1090564,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":253,"record_offset":1091784,"parent_record_offset":1091760,"ancestor_record_offset":1090564,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":253,"record_offset":1091832,"parent_record_offset":1091808,"ancestor_record_offset":1090564,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":253,"record_offset":1091808,"parent_record_offset":1091784,"ancestor_record_offset":1090564,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":1090680,"parent_record_offset":1090652,"ancestor_record_offset":1090564,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":253,"record_offset":1090708,"parent_record_offset":1090652,"ancestor_record_offset":1090564,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":253,"record_offset":1090896,"parent_record_offset":1090872,"ancestor_record_offset":1090564,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":253,"record_offset":1090944,"parent_record_offset":1090920,"ancestor_record_offset":1090564,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":253,"record_offset":1090920,"parent_record_offset":1090896,"ancestor_record_offset":1090564,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":253,"record_offset":1091760,"parent_record_offset":1091732,"ancestor_record_offset":1090564,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":253,"record_offset":1091732,"parent_record_offset":1091704,"ancestor_record_offset":1090564,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":253,"record_offset":1092572,"parent_record_offset":1092548,"ancestor_record_offset":1090564,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":253,"record_offset":1090872,"parent_record_offset":1090848,"ancestor_record_offset":1090564,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":253,"record_offset":1090848,"parent_record_offset":1090824,"ancestor_record_offset":1090564,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":253,"record_offset":1090824,"parent_record_offset":1090792,"ancestor_record_offset":1090564,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00253.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::BacktracePayload,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::BacktracePayload>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "tiny_glade::crash_pickup::crash_payload::BacktracePayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::BacktracePayload>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::BacktracePayload"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashMiscInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tiny_glade::crash_pickup::crash_payload::CrashMiscInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tiny_glade::crash_pickup::crash_payload::CrashMiscInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::CrashRecovered
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::CrashRecovered,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::CrashRecovered>", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "tiny_glade::crash_pickup::crash_payload::CrashRecovered"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::CrashRecovered>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::CrashRecovered"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSave
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$42::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde::ser::impls::impl$21::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >", "constructor": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "serde_json::de::VariantAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::RecoveredSave>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::crash_payload::RecoveredSession
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSession,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>", "constructor": "core::option::Option", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$16::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSession,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$16::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::RecoveredSession>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSession"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::crash_payload::SaveRecoveryError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >", "constructor": "core::result::Result", "arguments": ["tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >", "constructor": "serde::de::impls::impl$42::deserialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >", "constructor": "serde::ser::impls::impl$21::serialize", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >", "constructor": "serde::de::impls::impl$42::deserialize::impl$1::visit_enum", "arguments": ["tiny_glade::crash_pickup::crash_payload::RecoveredSave", "enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>", "serde_json::de::VariantAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 93; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>,bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::construct_playermade_stairs::magic_support::BuildingMagicSupports>,bevy_ecs::change_detection::Res<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>", "bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::construct_playermade_stairs::magic_support::BuildingMagicSupports>", "bevy_ecs::change_detection::Res<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::construct_playermade_stairs::magic_support::BuildingMagicSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineWallCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineWallCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsBracketsRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsBracketsRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsBracketsRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::CrashPickupMessage::set_context
// 信息 ID: 8c530d8d8d14748b6e7d1595
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=65968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize
// 信息 ID: d8afd2211430807ad043791b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=10, inline_record=57816; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=102808; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=103260; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=103444; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=103556; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=104048; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=104676; 不是独立调用边。
// [PDB 内联宿主] serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> >; module=188, inline_record=14224; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 8e99a644f63e0288b81bcb54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__FieldVisitor>; module=10, inline_record=30364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 37481a7ff81b58bec442c56a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=16, inline_record=98036; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=167056; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=171668; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=213364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 5c1d27661fb9c3b2c468e8a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor> @ 0x6d744 -> 0x3e9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x5e4c7 -> 0x3f900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 7281cf58dadb045fc7619e49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=16, inline_record=87952; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$5::deserialize<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Field,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=16, inline_record=97652; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=166864; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=171464; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=198564; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=213172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: a89aaa5f657155858e62cf8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=10, inline_record=29752; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=10, inline_record=30808; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=10, inline_record=31020; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=10, inline_record=31464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: c90b068a679c7710a59e8969
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=10, inline_record=40880; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=10, inline_record=41440; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=10, inline_record=57320; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >; module=10, inline_record=57896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 1ba735a38a6c25bea78f76f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=102112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: 07e943b61679575e37f3eb07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=117084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::expecting
// 信息 ID: 63d8a464f314718c9fdd9d90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=10, inline_record=31252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: 184e2b6d1a40c48ca1ce99ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=116424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_seq
// 信息 ID: dcfbb7849a77d22888b39a7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=119616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: fb1eb16de8e302f6e93cf881
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=101924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: 8915270fe6708c7771e52ce9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=10, inline_record=29984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 4d2043dc1a95923c38afdc15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=101260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: f694008db789eaa01af59dbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=103704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str
// 信息 ID: fdc54241b45c308dfb24d03d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=112896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: 564c605041f68f6e05cef2cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=112708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::expecting
// 信息 ID: 727b69d8dbf858708d31d7c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=10, inline_record=30596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: e5bc3824243bea9203d034f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=112072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_seq
// 信息 ID: b5758c24a97df9dd46a75048
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=114220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str
// 信息 ID: 83f412633cf2b3a8bb921b10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=107080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: 4a5e504d7f6897fb6799b36d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=106888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::expecting
// 信息 ID: c4eb53bc89eaf76f0b8e8ba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=10, inline_record=31696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: 4e56ade291b32df4069aaabb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=106232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_seq
// 信息 ID: a1dff8a5236218478b19fbc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=109952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str
// 信息 ID: 44b1c7a16a0bcce613f39e2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$14::struct_variant<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=16, inline_record=117276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 695851eef4f87f68345aaedb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=166220; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=170792; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=197856; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=212504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: f608f1819973738cfec30e50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=169048; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=176364; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=208204; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize::__Visitor>; module=16, inline_record=216652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
// 信息 ID: 6704162c18a370749cf985e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::SliceRead>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$42::deserialize::impl$1::visit_enum<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,serde_json::de::VariantAccess<serde_json::read::SliceRead> > @ 0x4a3bc8 -> 0x3e4c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::from_slice<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage> > @ 0x56f2f -> 0x3f4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 443a944c7662d182ba0506a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] tiny_glade::crash_pickup::crash_payload::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::BacktracePayload> @ 0x4c1788 -> 0x3e250 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x3114d0 -> 0x3e350 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,tiny_glade::crash_pickup::crash_payload::CrashRecovered> @ 0x4c0378 -> 0x3e7c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde::ser::impls::impl$21::serialize<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError>,ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x31153e -> 0x3eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<core::option::Option<tiny_glade::crash_pickup::crash_payload::RecoveredSession> > > @ 0x4bc2f2 -> 0x3ec80 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1> @ 0xda2da -> 0x3eda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::ClientHandle::send_message @ 0x15acc4 -> 0x3eda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$0::default
// 信息 ID: d3784929068fafe23f490e2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_minidump_created @ 0x2add40 -> 0x3cf340 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::crash_pickup::server::impl$1::on_message @ 0x2ae390 -> 0x3cf340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$10::default
// 信息 ID: 828dcd08075caa53000fa9ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=101, inline_record=62520; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=101, inline_record=64936; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=101, inline_record=67384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$2::drop
// 信息 ID: a91b332036b0fc0c82b83c3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::crash_pickup::crash_payload::ResolvedCrash>; module=253, inline_record=1090564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$5::fmt
// 信息 ID: 8fcd1ae45235a3e8e033e47b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$6::clone
// 信息 ID: a4af8a874e4743eee8e682e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=44736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$7::clone
// 信息 ID: c7364a267a2b1204e03c944f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=42860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$8::clone
// 信息 ID: 3f3346a93d01061775430313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=44772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::crash_payload::impl$9::clone
// 信息 ID: 851ee3be017d08630c64d0d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_report::on_game_crashed; module=157, inline_record=45696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
