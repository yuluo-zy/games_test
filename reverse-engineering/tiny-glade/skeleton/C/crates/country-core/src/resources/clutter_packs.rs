// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/clutter_packs.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3b1a1de25893fb54dd36215446dd16d78b8533667f17c871801ab2459ef4ea51
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clutter_packs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 546, 559, 566, 586, 612, 614, 1249, 1639, 1643, 1645, 1650, 1658, 1665, 1677, 1682, 1687, 1812, 1813, 1822, 1844, 2873, 2897, 3393
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itoa; // namespace candidate
//   package identities: [{"name":"itoa","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.3"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::components::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1812,"record_offset":268,"rva":29398576,"va":5398107696}
// 编译实例 RVAs: 0x1c09630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18245003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398107696,"raw_hex":"e88b651601","size":5,"soft_fail":false,"target_va":5416353040,"va":5398108032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18949067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398107696,"raw_hex":"e8cb232101","size":5,"soft_fail":false,"target_va":5417057056,"va":5398107984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_core::ser::impls::impl$45::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":1812,"record_offset":528,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":12424,"source_line":11,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":268,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":268,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1108,"parent_record_offset":1072,"ancestor_record_offset":268,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":268,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":268,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":268,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1228,"parent_record_offset":968,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1012,"parent_record_offset":968,"ancestor_record_offset":268,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1812,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":268,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1812,"record_offset":736,"parent_record_offset":656,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1812,"record_offset":1192,"parent_record_offset":968,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] itoa::Buffer::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1812,"record_offset":628,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":7394,"source_line":86,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$18::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1812,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":9176,"source_line":125,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":592,"parent_record_offset":560,"ancestor_record_offset":268,"inlinee":12426,"source_line":1655,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":560,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":12425,"source_line":142,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1812,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":268,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1812,"record_offset":856,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 25, 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E16
// 首条位置: {"module":1645,"inlinee":81430}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":237760,"parent_record_offset":237736,"ancestor_record_offset":237736,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":237900,"parent_record_offset":237876,"ancestor_record_offset":237828,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":237876,"parent_record_offset":237852,"ancestor_record_offset":237828,"inlinee":7860,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":237924,"parent_record_offset":237900,"ancestor_record_offset":237828,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=5
// 源码行: 25, 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1812,"record_offset":1292,"rva":29398928,"va":5398108048}
// 编译实例 RVAs: 0x1c09630, 0x1c09790, 0x1c09970, 0x1c09b50, 0x1c09d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26281746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398109488,"raw_hex":"e8eef86efe","size":5,"soft_fail":false,"target_va":5371827984,"va":5398109725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b3010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3499997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398108048,"raw_hex":"e82398caff","size":5,"soft_fail":false,"target_va":5394608144,"va":5398108136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3496327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398109488,"raw_hex":"e879a6caff","size":5,"soft_fail":false,"target_va":5394613552,"va":5398109874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b66a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3487471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398109488,"raw_hex":"e811c9caff","size":5,"soft_fail":false,"target_va":5394622112,"va":5398109578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b87a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3478061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398108528,"raw_hex":"e8d3edcaff","size":5,"soft_fail":false,"target_va":5394630560,"va":5398108616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3469874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398109488,"raw_hex":"e8ce0dcbff","size":5,"soft_fail":false,"target_va":5394639776,"va":5398109645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bba80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3465517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398109008,"raw_hex":"e8d31ecbff","size":5,"soft_fail":false,"target_va":5394643584,"va":5398109096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18245003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398107696,"raw_hex":"e88b651601","size":5,"soft_fail":false,"target_va":5416353040,"va":5398108032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18949067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398107696,"raw_hex":"e8cb232101","size":5,"soft_fail":false,"target_va":5417057056,"va":5398107984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t18448121","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5398109488,"raw_hex":"e9f97e1901","size":5,"soft_fail":false,"target_va":5416557744,"va":5398109618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1812,"record_offset":10504,"parent_record_offset":9612,"ancestor_record_offset":9612,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_direct] serde_core::ser::impls::impl$49::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":1812,"record_offset":3992,"parent_record_offset":3720,"ancestor_record_offset":3720,"inlinee":10591,"source_line":11,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":1572,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":2148,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":2120,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":1292,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1840,"parent_record_offset":1812,"ancestor_record_offset":1292,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":1292,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1900,"parent_record_offset":1868,"ancestor_record_offset":1292,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1292,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":1292,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1812,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":1292,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":1292,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1940,"parent_record_offset":1772,"ancestor_record_offset":1292,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":1292,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":2060,"parent_record_offset":1772,"ancestor_record_offset":1292,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1812,"record_offset":1812,"parent_record_offset":1772,"ancestor_record_offset":1292,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1812,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":1292,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1812,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":1292,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1812,"record_offset":2776,"parent_record_offset":2748,"ancestor_record_offset":1292,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1812,"record_offset":2748,"parent_record_offset":2712,"ancestor_record_offset":1292,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1812,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":9612,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1812,"record_offset":4200,"parent_record_offset":4120,"ancestor_record_offset":3720,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1812,"record_offset":3164,"parent_record_offset":2956,"ancestor_record_offset":1292,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] itoa::Buffer::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1812,"record_offset":4092,"parent_record_offset":4056,"ancestor_record_offset":3720,"inlinee":7394,"source_line":86,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$18::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1812,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":3720,"inlinee":9176,"source_line":125,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":4056,"parent_record_offset":4024,"ancestor_record_offset":3720,"inlinee":12426,"source_line":1655,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":1292,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::end_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":2216,"parent_record_offset":2184,"ancestor_record_offset":1292,"inlinee":12493,"source_line":2028,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":1292,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":4024,"parent_record_offset":3992,"ancestor_record_offset":3720,"inlinee":12425,"source_line":142,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":2184,"parent_record_offset":2148,"ancestor_record_offset":1292,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::indent
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1812,"record_offset":2712,"parent_record_offset":2216,"ancestor_record_offset":1292,"inlinee":8104,"source_line":2276,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1812,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":1292,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1812,"record_offset":1660,"parent_record_offset":1628,"ancestor_record_offset":1292,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/01812.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::impl$12::eq
// 记录形式: inline_instance, inlinee_declaration; instances=42
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88BF
// 首条位置: {"module":559,"inlinee":35007}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":76452,"rva":13211440,"va":5381920560}
// 编译实例 RVAs: 0xc99730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f695b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2948731","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381920560,"raw_hex":"e97bfe2c00","size":5,"soft_fail":false,"target_va":5384869296,"va":5381920560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::ClutterPackObject::id
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":15140,"rva":13193120,"va":5381902240}
// 编译实例 RVAs: 0xc94fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6223694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902240,"raw_hex":"e8b208a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381902393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2892511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902240,"raw_hex":"e8df222c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5381902348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2057742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902240,"raw_hex":"e80e661f00","size":5,"soft_fail":false,"target_va":5383960112,"va":5381902365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::mod_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":612,"record_offset":15288,"parent_record_offset":15140,"ancestor_record_offset":15140,"inlinee":32533,"source_line":31,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":15580,"parent_record_offset":15556,"ancestor_record_offset":15140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":15604,"parent_record_offset":15580,"ancestor_record_offset":15140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":15140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":15556,"parent_record_offset":15500,"ancestor_record_offset":15140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":15324,"parent_record_offset":15288,"ancestor_record_offset":15140,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":15140,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":15140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":15140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":15140,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":15404,"parent_record_offset":15288,"ancestor_record_offset":15140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E45
// 首条位置: {"module":1645,"inlinee":81477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":237200,"parent_record_offset":237176,"ancestor_record_offset":237176,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":237224,"parent_record_offset":237200,"ancestor_record_offset":237176,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237248,"parent_record_offset":237224,"ancestor_record_offset":237176,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237276,"parent_record_offset":237248,"ancestor_record_offset":237176,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E44
// 首条位置: {"module":1645,"inlinee":81476}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":237024,"parent_record_offset":237000,"ancestor_record_offset":237000,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":237148,"parent_record_offset":237048,"ancestor_record_offset":237000,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":237224,"parent_record_offset":237200,"ancestor_record_offset":237000,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237248,"parent_record_offset":237224,"ancestor_record_offset":237000,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237276,"parent_record_offset":237248,"ancestor_record_offset":237000,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":237200,"parent_record_offset":237176,"ancestor_record_offset":237000,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1645,"record_offset":237176,"parent_record_offset":237048,"ancestor_record_offset":237000,"inlinee":81477,"source_line":117,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237092,"parent_record_offset":237048,"ancestor_record_offset":237000,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237048,"parent_record_offset":237024,"ancestor_record_offset":237000,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":237116,"parent_record_offset":237092,"ancestor_record_offset":237000,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9841
// 首条位置: {"module":1812,"inlinee":38977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E41
// 首条位置: {"module":1645,"inlinee":81473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":237344,"parent_record_offset":236844,"ancestor_record_offset":236844,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":383480,"parent_record_offset":382512,"ancestor_record_offset":382512,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":383448,"parent_record_offset":382512,"ancestor_record_offset":382512,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":237804,"parent_record_offset":236844,"ancestor_record_offset":236844,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":237972,"parent_record_offset":236844,"ancestor_record_offset":236844,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":236884,"parent_record_offset":236844,"ancestor_record_offset":236844,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":237376,"parent_record_offset":236844,"ancestor_record_offset":236844,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":383792,"parent_record_offset":383768,"ancestor_record_offset":382512,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":383816,"parent_record_offset":383792,"ancestor_record_offset":382512,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":383840,"parent_record_offset":383816,"ancestor_record_offset":382512,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":383768,"parent_record_offset":383704,"ancestor_record_offset":382512,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":383736,"parent_record_offset":383704,"ancestor_record_offset":382512,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":383704,"parent_record_offset":383676,"ancestor_record_offset":382512,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":383676,"parent_record_offset":383648,"ancestor_record_offset":382512,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":237148,"parent_record_offset":237048,"ancestor_record_offset":236844,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":383508,"parent_record_offset":383480,"ancestor_record_offset":382512,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":237224,"parent_record_offset":237200,"ancestor_record_offset":236844,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1645,"record_offset":384016,"parent_record_offset":383988,"ancestor_record_offset":382512,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":383536,"parent_record_offset":383508,"ancestor_record_offset":382512,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237248,"parent_record_offset":237224,"ancestor_record_offset":236844,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":237276,"parent_record_offset":237248,"ancestor_record_offset":236844,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":237200,"parent_record_offset":237176,"ancestor_record_offset":236844,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":1645,"record_offset":449888,"parent_record_offset":449864,"ancestor_record_offset":447360,"inlinee":81009,"source_line":630,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1645,"record_offset":237736,"parent_record_offset":237712,"ancestor_record_offset":236844,"inlinee":81430,"source_line":25,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1645,"record_offset":237176,"parent_record_offset":237048,"ancestor_record_offset":236844,"inlinee":81477,"source_line":117,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1645,"record_offset":237000,"parent_record_offset":236976,"ancestor_record_offset":236844,"inlinee":81476,"source_line":117,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":237900,"parent_record_offset":237876,"ancestor_record_offset":236844,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":449912,"parent_record_offset":449888,"ancestor_record_offset":447360,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":470148,"parent_record_offset":470124,"ancestor_record_offset":469172,"inlinee":34987,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":384224,"parent_record_offset":384200,"ancestor_record_offset":382512,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":449772,"parent_record_offset":449748,"ancestor_record_offset":447360,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":237876,"parent_record_offset":237852,"ancestor_record_offset":236844,"inlinee":7860,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":237924,"parent_record_offset":237900,"ancestor_record_offset":236844,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":236976,"parent_record_offset":236916,"ancestor_record_offset":236844,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":383408,"parent_record_offset":383384,"ancestor_record_offset":382512,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$203::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":470028,"parent_record_offset":470004,"ancestor_record_offset":469172,"inlinee":7843,"source_line":1515,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":237760,"parent_record_offset":237736,"ancestor_record_offset":236844,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":449400,"parent_record_offset":449376,"ancestor_record_offset":447360,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237092,"parent_record_offset":237048,"ancestor_record_offset":236844,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237444,"parent_record_offset":237404,"ancestor_record_offset":236844,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237484,"parent_record_offset":237444,"ancestor_record_offset":236844,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237524,"parent_record_offset":237484,"ancestor_record_offset":236844,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":236916,"parent_record_offset":236884,"ancestor_record_offset":236844,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237404,"parent_record_offset":237376,"ancestor_record_offset":236844,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":237048,"parent_record_offset":237024,"ancestor_record_offset":236844,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":237024,"parent_record_offset":237000,"ancestor_record_offset":236844,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":237116,"parent_record_offset":237092,"ancestor_record_offset":236844,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":237552,"parent_record_offset":237524,"ancestor_record_offset":236844,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 93, 117, 259, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E42
// 首条位置: {"module":1645,"inlinee":81474}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":451440,"parent_record_offset":451008,"ancestor_record_offset":451008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":238684,"parent_record_offset":238656,"ancestor_record_offset":238656,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":451616,"parent_record_offset":451592,"ancestor_record_offset":451008,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":451640,"parent_record_offset":451616,"ancestor_record_offset":451008,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":451664,"parent_record_offset":451640,"ancestor_record_offset":451008,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":451592,"parent_record_offset":451536,"ancestor_record_offset":451008,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":451564,"parent_record_offset":451536,"ancestor_record_offset":451008,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":451536,"parent_record_offset":451512,"ancestor_record_offset":451008,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":451512,"parent_record_offset":451488,"ancestor_record_offset":451008,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":451464,"parent_record_offset":451440,"ancestor_record_offset":451008,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1645,"record_offset":238780,"parent_record_offset":238756,"ancestor_record_offset":238656,"inlinee":81430,"source_line":25,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":238756,"parent_record_offset":238708,"ancestor_record_offset":238656,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":384544,"parent_record_offset":384520,"ancestor_record_offset":384404,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$203::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":470468,"parent_record_offset":470444,"ancestor_record_offset":470328,"inlinee":7843,"source_line":1515,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$73::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":238804,"parent_record_offset":238780,"ancestor_record_offset":238656,"inlinee":17603,"source_line":132,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":451176,"parent_record_offset":451152,"ancestor_record_offset":451008,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":238708,"parent_record_offset":238684,"ancestor_record_offset":238656,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::WorkshopSidecar::read
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":12976,"rva":13192640,"va":5381901760}
// 编译实例 RVAs: 0xc94dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2754423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381901760,"raw_hex":"e877072a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5381901812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-6223159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381901760,"raw_hex":"e8c90aa1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381901858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12563036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381901760,"raw_hex":"e8a44d40ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381901879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14164dc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10194376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381901760,"raw_hex":"e8c88d9b00","size":5,"soft_fail":false,"target_va":5392096288,"va":5381901907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2850098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381901760,"raw_hex":"e8327d2b00","size":5,"soft_fail":false,"target_va":5384751936,"va":5381901833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":14624,"parent_record_offset":12976,"ancestor_record_offset":12976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":13912,"parent_record_offset":12976,"ancestor_record_offset":12976,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":13164,"parent_record_offset":12976,"ancestor_record_offset":12976,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":13136,"parent_record_offset":12976,"ancestor_record_offset":12976,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":12976,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":12976,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":12976,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":13396,"parent_record_offset":13344,"ancestor_record_offset":12976,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":612,"record_offset":14168,"parent_record_offset":14140,"ancestor_record_offset":12976,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":12976,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":12976,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":13320,"parent_record_offset":13300,"ancestor_record_offset":12976,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":13200,"parent_record_offset":13164,"ancestor_record_offset":12976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::write_pack_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":39804,"rva":13201216,"va":5381910336}
// 编译实例 RVAs: 0xc96f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2743883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e84bde2900","size":5,"soft_fail":false,"target_va":5384656240,"va":5381912352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_strip_prefix
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2743706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e89add2900","size":5,"soft_fail":false,"target_va":5384654544,"va":5381910833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2742290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e812d82900","size":5,"soft_fail":false,"target_va":5384653152,"va":5381910857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3201751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8d7da3000","size":5,"soft_fail":false,"target_va":5385113168,"va":5381911412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6233114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8e6e3a0ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381911797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=26
//     evidence={"assembly":"\tcallq\t-6231840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8e0e8a0ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381910539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13189595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e825be36ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381911782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12643752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e858123fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5381914035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008d710
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12625295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8715a3fff","size":5,"soft_fail":false,"target_va":5369288464,"va":5381913754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12481792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8008b41ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5381913147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972ae0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3295962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e826b5cdff","size":5,"soft_fail":false,"target_va":5378616032,"va":5381911989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b069d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1640932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e81cf6e6ff","size":5,"soft_fail":false,"target_va":5380270544,"va":5381911471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1619470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8f249e7ff","size":5,"soft_fail":false,"target_va":5380291456,"va":5381910921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac4040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14863953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e851cee200","size":5,"soft_fail":false,"target_va":5396774976,"va":5381911018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c09790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16196148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e83422f700","size":5,"soft_fail":false,"target_va":5398108048,"va":5381911895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34473335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e877050e02","size":5,"soft_fail":false,"target_va":5416385392,"va":5381912052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd54f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34856781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e84ddf1302","size":5,"soft_fail":false,"target_va":5416768752,"va":5381911966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34858725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8e5e61302","size":5,"soft_fail":false,"target_va":5416769200,"va":5381910470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34708196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e8e49a1102","size":5,"soft_fail":false,"target_va":5416620794,"va":5381912593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3205426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e832e93000","size":5,"soft_fail":false,"target_va":5385116768,"va":5381911337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34709098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e86a9e1102","size":5,"soft_fail":false,"target_va":5416621632,"va":5381912529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::collect_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e87ed9ffff","size":5,"soft_fail":false,"target_va":5381900592,"va":5381910445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t34490001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e891460e02","size":5,"soft_fail":false,"target_va":5416402080,"va":5381912074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::is_safe_relative_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e81d0b0000","size":5,"soft_fail":false,"target_va":5381914048,"va":5381911198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2840678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e866582b00","size":5,"soft_fail":false,"target_va":5384751936,"va":5381911253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2840070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381910336,"raw_hex":"e806562b00","size":5,"soft_fail":false,"target_va":5384752464,"va":5381912389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":612,"record_offset":40856,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":7768,"source_line":651,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":41632,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":11201,"source_line":596,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":42552,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":42340,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":40020,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":40884,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":41276,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":41544,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":41488,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":40076,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":42512,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":41452,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":41600,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":40048,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":612,"record_offset":45328,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":8088,"source_line":2229,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":42484,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":47872,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":47844,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":41424,"parent_record_offset":39804,"ancestor_record_offset":39804,"inlinee":11153,"source_line":2710,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":45560,"parent_record_offset":45536,"ancestor_record_offset":39804,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":45584,"parent_record_offset":45560,"ancestor_record_offset":39804,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":40436,"parent_record_offset":40412,"ancestor_record_offset":39804,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":40460,"parent_record_offset":40436,"ancestor_record_offset":39804,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":45608,"parent_record_offset":45584,"ancestor_record_offset":39804,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":39804,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":45536,"parent_record_offset":45508,"ancestor_record_offset":39804,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":40412,"parent_record_offset":40348,"ancestor_record_offset":39804,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":45448,"parent_record_offset":45420,"ancestor_record_offset":39804,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":40380,"parent_record_offset":40348,"ancestor_record_offset":39804,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":40348,"parent_record_offset":40320,"ancestor_record_offset":39804,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":45508,"parent_record_offset":45476,"ancestor_record_offset":39804,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":45476,"parent_record_offset":45448,"ancestor_record_offset":39804,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":40320,"parent_record_offset":40292,"ancestor_record_offset":39804,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":612,"record_offset":41656,"parent_record_offset":41632,"ancestor_record_offset":39804,"inlinee":11203,"source_line":75,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":42640,"parent_record_offset":42588,"ancestor_record_offset":39804,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":42588,"parent_record_offset":42552,"ancestor_record_offset":39804,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":45392,"parent_record_offset":45328,"ancestor_record_offset":39804,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":45420,"parent_record_offset":45392,"ancestor_record_offset":39804,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":40104,"parent_record_offset":40076,"ancestor_record_offset":39804,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":44544,"parent_record_offset":44520,"ancestor_record_offset":39804,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":45028,"parent_record_offset":45004,"ancestor_record_offset":39804,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":42388,"parent_record_offset":42364,"ancestor_record_offset":39804,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":612,"record_offset":42668,"parent_record_offset":42588,"ancestor_record_offset":39804,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":612,"record_offset":41512,"parent_record_offset":41488,"ancestor_record_offset":39804,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":40696,"parent_record_offset":40656,"ancestor_record_offset":39804,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":40128,"parent_record_offset":40104,"ancestor_record_offset":39804,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":612,"record_offset":40908,"parent_record_offset":40884,"ancestor_record_offset":39804,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":41336,"parent_record_offset":41276,"ancestor_record_offset":39804,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":41364,"parent_record_offset":41276,"ancestor_record_offset":39804,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":41308,"parent_record_offset":41276,"ancestor_record_offset":39804,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":41388,"parent_record_offset":41364,"ancestor_record_offset":39804,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":42696,"parent_record_offset":42588,"ancestor_record_offset":39804,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":41568,"parent_record_offset":41544,"ancestor_record_offset":39804,"inlinee":23302,"source_line":2155,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":42412,"parent_record_offset":42388,"ancestor_record_offset":39804,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":42440,"parent_record_offset":42412,"ancestor_record_offset":39804,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":612,"record_offset":42364,"parent_record_offset":42340,"ancestor_record_offset":39804,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":612,"record_offset":45696,"parent_record_offset":45668,"ancestor_record_offset":39804,"inlinee":8091,"source_line":39,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":612,"record_offset":45720,"parent_record_offset":45696,"ancestor_record_offset":39804,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":612,"record_offset":45668,"parent_record_offset":45328,"ancestor_record_offset":39804,"inlinee":8089,"source_line":2197,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::write_pack_checksums::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143AC
// 首条位置: {"module":1682,"inlinee":82860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":175000,"parent_record_offset":174852,"ancestor_record_offset":174852,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":175644,"parent_record_offset":174852,"ancestor_record_offset":174852,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1682,"record_offset":174916,"parent_record_offset":174852,"ancestor_record_offset":174852,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1682,"record_offset":174880,"parent_record_offset":174852,"ancestor_record_offset":174852,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":175456,"parent_record_offset":175432,"ancestor_record_offset":174852,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":175480,"parent_record_offset":175456,"ancestor_record_offset":174852,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":175504,"parent_record_offset":175480,"ancestor_record_offset":174852,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":175432,"parent_record_offset":175276,"ancestor_record_offset":174852,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":175728,"parent_record_offset":175668,"ancestor_record_offset":174852,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":175776,"parent_record_offset":175752,"ancestor_record_offset":174852,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":175212,"parent_record_offset":175184,"ancestor_record_offset":174852,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":175276,"parent_record_offset":175240,"ancestor_record_offset":174852,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":175240,"parent_record_offset":175212,"ancestor_record_offset":174852,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":175320,"parent_record_offset":175276,"ancestor_record_offset":174852,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":175100,"parent_record_offset":175076,"ancestor_record_offset":174852,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":175124,"parent_record_offset":175100,"ancestor_record_offset":174852,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":175148,"parent_record_offset":175124,"ancestor_record_offset":174852,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":175076,"parent_record_offset":175052,"ancestor_record_offset":174852,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1682,"record_offset":175052,"parent_record_offset":175028,"ancestor_record_offset":174852,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":175184,"parent_record_offset":175148,"ancestor_record_offset":174852,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1682,"record_offset":175344,"parent_record_offset":175320,"ancestor_record_offset":174852,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":175392,"parent_record_offset":175368,"ancestor_record_offset":174852,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":175368,"parent_record_offset":175344,"ancestor_record_offset":174852,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1682,"record_offset":175028,"parent_record_offset":175000,"ancestor_record_offset":174852,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":175752,"parent_record_offset":175728,"ancestor_record_offset":174852,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":175560,"parent_record_offset":175148,"ancestor_record_offset":174852,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":175584,"parent_record_offset":175560,"ancestor_record_offset":174852,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1682,"record_offset":174964,"parent_record_offset":174940,"ancestor_record_offset":174852,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::write_pack_checksums::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":1682,"record_offset":175668,"parent_record_offset":175644,"ancestor_record_offset":174852,"inlinee":82862,"source_line":154,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1682,"record_offset":175692,"parent_record_offset":175668,"ancestor_record_offset":174852,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1682,"record_offset":174940,"parent_record_offset":174916,"ancestor_record_offset":174852,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::write_pack_checksums::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143AE
// 首条位置: {"module":1682,"inlinee":82862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":175728,"parent_record_offset":175668,"ancestor_record_offset":175668,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1682,"record_offset":175692,"parent_record_offset":175668,"ancestor_record_offset":175668,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1682,"record_offset":175776,"parent_record_offset":175752,"ancestor_record_offset":175668,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":175752,"parent_record_offset":175728,"ancestor_record_offset":175668,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::verify_pack_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":52396,"rva":13205360,"va":5381914480}
// 编译实例 RVAs: 0xc97f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2741683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8b3d52900","size":5,"soft_fail":false,"target_va":5384656240,"va":5381914552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3197561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e879ca3000","size":5,"soft_fail":false,"target_va":5385113168,"va":5381915602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-6236183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8e9d7a0ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381914882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12575835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8a51b40ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381914678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12573112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8482640ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5381916611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972ae0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3299952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e890a5cdff","size":5,"soft_fail":false,"target_va":5378616032,"va":5381915979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14164d6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10180246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e896569b00","size":5,"soft_fail":false,"target_va":5392094960,"va":5381914709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35141375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8ff361802","size":5,"soft_fail":false,"target_va":5417057040,"va":5381915660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t2880209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8d1f22b00","size":5,"soft_fail":false,"target_va":5384794864,"va":5381914650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3201230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8ced83000","size":5,"soft_fail":false,"target_va":5385116768,"va":5381915533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34705270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8768f1102","size":5,"soft_fail":false,"target_va":5416621600,"va":5381916325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34705317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8a58f1102","size":5,"soft_fail":false,"target_va":5416621632,"va":5381916310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::is_safe_relative_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8aafaffff","size":5,"soft_fail":false,"target_va":5381914048,"va":5381915409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2837362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914480,"raw_hex":"e8724b2b00","size":5,"soft_fail":false,"target_va":5384751936,"va":5381914569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::map::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":612,"record_offset":53480,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":8808,"source_line":1724,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::map::impl$22::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":612,"record_offset":53544,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":8812,"source_line":1733,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":52660,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":54728,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":55200,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":612,"record_offset":52744,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":52884,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":52604,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":52576,"parent_record_offset":52396,"ancestor_record_offset":52396,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":53088,"parent_record_offset":53064,"ancestor_record_offset":52396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":53112,"parent_record_offset":53088,"ancestor_record_offset":52396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":53136,"parent_record_offset":53112,"ancestor_record_offset":52396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":53064,"parent_record_offset":53032,"ancestor_record_offset":52396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":612,"record_offset":53032,"parent_record_offset":53004,"ancestor_record_offset":52396,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":612,"record_offset":53508,"parent_record_offset":53480,"ancestor_record_offset":52396,"inlinee":8809,"source_line":2698,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":612,"record_offset":53796,"parent_record_offset":53736,"ancestor_record_offset":52396,"inlinee":4906,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":612,"record_offset":54568,"parent_record_offset":54544,"ancestor_record_offset":52396,"inlinee":4926,"source_line":21,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::init_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53628,"parent_record_offset":53588,"ancestor_record_offset":52396,"inlinee":4912,"source_line":221,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53588,"parent_record_offset":53544,"ancestor_record_offset":52396,"inlinee":6173,"source_line":160,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::impl$22::next_unchecked::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53844,"parent_record_offset":53796,"ancestor_record_offset":52396,"inlinee":8814,"source_line":533,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":54216,"parent_record_offset":54120,"ancestor_record_offset":52396,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":54160,"parent_record_offset":54120,"ancestor_record_offset":52396,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":54428,"parent_record_offset":53844,"ancestor_record_offset":52396,"inlinee":7204,"source_line":1121,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53892,"parent_record_offset":53844,"ancestor_record_offset":52396,"inlinee":6176,"source_line":380,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":54120,"parent_record_offset":53844,"ancestor_record_offset":52396,"inlinee":4910,"source_line":715,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53736,"parent_record_offset":53588,"ancestor_record_offset":52396,"inlinee":7282,"source_line":532,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":54396,"parent_record_offset":54120,"ancestor_record_offset":52396,"inlinee":4911,"source_line":811,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":53928,"parent_record_offset":53892,"ancestor_record_offset":52396,"inlinee":4913,"source_line":893,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":53996,"parent_record_offset":53892,"ancestor_record_offset":52396,"inlinee":4917,"source_line":328,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::first_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":612,"record_offset":53660,"parent_record_offset":53628,"ancestor_record_offset":52396,"inlinee":4915,"source_line":630,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":53692,"parent_record_offset":53660,"ancestor_record_offset":52396,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":53960,"parent_record_offset":53928,"ancestor_record_offset":52396,"inlinee":4914,"source_line":287,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$16::ascend::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":612,"record_offset":54076,"parent_record_offset":54052,"ancestor_record_offset":52396,"inlinee":4919,"source_line":340,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":52708,"parent_record_offset":52684,"ancestor_record_offset":52396,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":55032,"parent_record_offset":55004,"ancestor_record_offset":52396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":55004,"parent_record_offset":54980,"ancestor_record_offset":52396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":54980,"parent_record_offset":54956,"ancestor_record_offset":52396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":54816,"parent_record_offset":54764,"ancestor_record_offset":52396,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":54764,"parent_record_offset":54728,"ancestor_record_offset":52396,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":612,"record_offset":55252,"parent_record_offset":55224,"ancestor_record_offset":52396,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":612,"record_offset":55280,"parent_record_offset":55252,"ancestor_record_offset":52396,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":55224,"parent_record_offset":55200,"ancestor_record_offset":52396,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":612,"record_offset":52628,"parent_record_offset":52604,"ancestor_record_offset":52396,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":612,"record_offset":54844,"parent_record_offset":54764,"ancestor_record_offset":52396,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":54024,"parent_record_offset":53996,"ancestor_record_offset":52396,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":54052,"parent_record_offset":53996,"ancestor_record_offset":52396,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":52684,"parent_record_offset":52660,"ancestor_record_offset":52396,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":54512,"parent_record_offset":53844,"ancestor_record_offset":52396,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":52916,"parent_record_offset":52884,"ancestor_record_offset":52396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":54872,"parent_record_offset":54764,"ancestor_record_offset":52396,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":55308,"parent_record_offset":55280,"ancestor_record_offset":52396,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":55336,"parent_record_offset":55308,"ancestor_record_offset":52396,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":612,"record_offset":54272,"parent_record_offset":54216,"ancestor_record_offset":52396,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":612,"record_offset":54296,"parent_record_offset":54272,"ancestor_record_offset":52396,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::is_safe_relative_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":50668,"rva":13204928,"va":5381914048}
// 编译实例 RVAs: 0xc97dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2916483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381914048,"raw_hex":"e883802c00","size":5,"soft_fail":false,"target_va":5384830608,"va":5381914120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":51308,"parent_record_offset":50668,"ancestor_record_offset":50668,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":612,"record_offset":50848,"parent_record_offset":50668,"ancestor_record_offset":50668,"inlinee":12765,"source_line":1367,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":612,"record_offset":50824,"parent_record_offset":50668,"ancestor_record_offset":50668,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51456,"parent_record_offset":51432,"ancestor_record_offset":50668,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51504,"parent_record_offset":51480,"ancestor_record_offset":50668,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51432,"parent_record_offset":51400,"ancestor_record_offset":50668,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":51368,"parent_record_offset":51332,"ancestor_record_offset":50668,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":51332,"parent_record_offset":51308,"ancestor_record_offset":50668,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":52116,"parent_record_offset":52088,"ancestor_record_offset":50668,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":612,"record_offset":52360,"parent_record_offset":51308,"ancestor_record_offset":50668,"inlinee":38925,"source_line":88,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":51032,"parent_record_offset":50932,"ancestor_record_offset":50668,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":51528,"parent_record_offset":51504,"ancestor_record_offset":50668,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$18::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":50932,"parent_record_offset":50904,"ancestor_record_offset":50668,"inlinee":12768,"source_line":382,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":50668,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":612,"record_offset":50904,"parent_record_offset":50876,"ancestor_record_offset":50668,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":612,"record_offset":52064,"parent_record_offset":52028,"ancestor_record_offset":50668,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":612,"record_offset":52148,"parent_record_offset":52088,"ancestor_record_offset":50668,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":612,"record_offset":52088,"parent_record_offset":52064,"ancestor_record_offset":50668,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":612,"record_offset":50960,"parent_record_offset":50932,"ancestor_record_offset":50668,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":612,"record_offset":50992,"parent_record_offset":50960,"ancestor_record_offset":50668,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":612,"record_offset":51996,"parent_record_offset":51968,"ancestor_record_offset":50668,"inlinee":22833,"source_line":657,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":612,"record_offset":51968,"parent_record_offset":51332,"ancestor_record_offset":50668,"inlinee":22809,"source_line":511,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":612,"record_offset":52028,"parent_record_offset":51996,"ancestor_record_offset":50668,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_contained_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":612,"record_offset":50876,"parent_record_offset":50848,"ancestor_record_offset":50668,"inlinee":12766,"source_line":583,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":612,"record_offset":51480,"parent_record_offset":51456,"ancestor_record_offset":50668,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::is_safe_relative_path::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":51400,"parent_record_offset":51368,"ancestor_record_offset":50668,"inlinee":38923,"source_line":240,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::is_safe_relative_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x980B
// 首条位置: {"module":612,"inlinee":38923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51432,"parent_record_offset":51400,"ancestor_record_offset":51400,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51456,"parent_record_offset":51432,"ancestor_record_offset":51400,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":51504,"parent_record_offset":51480,"ancestor_record_offset":51400,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":51528,"parent_record_offset":51504,"ancestor_record_offset":51400,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":51400,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":612,"record_offset":51480,"parent_record_offset":51456,"ancestor_record_offset":51400,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::collect_files
// 记录形式: compiled_procedure; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":9520,"rva":13191472,"va":5381900592}
// 编译实例 RVAs: 0xc94930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2788415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e83f8c2a00","size":5,"soft_fail":false,"target_va":5384689312,"va":5381900892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2867485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e81dc12b00","size":5,"soft_fail":false,"target_va":5384768304,"va":5381900814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2755444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e8740b2a00","size":5,"soft_fail":false,"target_va":5384656400,"va":5381900951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2863354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e8fab02b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5381901105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-6222321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e80f0ea1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381901020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12566728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e8383f40ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5381901731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35156356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e884711802","size":5,"soft_fail":false,"target_va":5417057056,"va":5381900695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34467030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e8d6ec0d02","size":5,"soft_fail":false,"target_va":5416368112,"va":5381901077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t221877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e8b5620300","size":5,"soft_fail":false,"target_va":5382122800,"va":5381900918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::collect_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e87afeffff","size":5,"soft_fail":false,"target_va":5381900592,"va":5381900977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2824501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381900592,"raw_hex":"e835192b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5381900646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":9740,"parent_record_offset":9520,"ancestor_record_offset":9520,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":10384,"parent_record_offset":9520,"ancestor_record_offset":9520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":10352,"parent_record_offset":9520,"ancestor_record_offset":9520,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":9672,"parent_record_offset":9520,"ancestor_record_offset":9520,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":9520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":9520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":10200,"parent_record_offset":10144,"ancestor_record_offset":9520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":9860,"parent_record_offset":9828,"ancestor_record_offset":9520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":10172,"parent_record_offset":10144,"ancestor_record_offset":9520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":10144,"parent_record_offset":10120,"ancestor_record_offset":9520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":9520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":9520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":9520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":612,"record_offset":10492,"parent_record_offset":10456,"ancestor_record_offset":9520,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":612,"record_offset":10456,"parent_record_offset":10432,"ancestor_record_offset":9520,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":9828,"parent_record_offset":9776,"ancestor_record_offset":9520,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":9776,"parent_record_offset":9740,"ancestor_record_offset":9520,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":9976,"parent_record_offset":9776,"ancestor_record_offset":9520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":10524,"parent_record_offset":10492,"ancestor_record_offset":9520,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":9948,"parent_record_offset":9776,"ancestor_record_offset":9520,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":9700,"parent_record_offset":9672,"ancestor_record_offset":9520,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":612,"record_offset":10552,"parent_record_offset":10492,"ancestor_record_offset":9520,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":612,"record_offset":10584,"parent_record_offset":10456,"ancestor_record_offset":9520,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":15980,"rva":13193360,"va":5381902480}
// 编译实例 RVAs: 0xc95090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2892207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8af212c00","size":5,"soft_fail":false,"target_va":5384795984,"va":5381903772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2742007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8f7d62900","size":5,"soft_fail":false,"target_va":5384644912,"va":5381902900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Components as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2872149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e855d32b00","size":5,"soft_fail":false,"target_va":5384775360,"va":5381903206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2753667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e883042a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5381902568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2749974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e816f62900","size":5,"soft_fail":false,"target_va":5384653152,"va":5381903173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2753466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8ba032a00","size":5,"soft_fail":false,"target_va":5384656912,"va":5381903441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2753771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8eb042a00","size":5,"soft_fail":false,"target_va":5384657024,"va":5381903248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2753771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8eb042a00","size":5,"soft_fail":false,"target_va":5384657312,"va":5381903536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2752614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e866002a00","size":5,"soft_fail":false,"target_va":5384656800,"va":5381904181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2752117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e875fe2900","size":5,"soft_fail":false,"target_va":5384656512,"va":5381904390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2860426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e88aa52b00","size":5,"soft_fail":false,"target_va":5384764464,"va":5381904033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6225013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e88b03a1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381903696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=42
//     evidence={"assembly":"\tcallq\t-6225449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8d701a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381904148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13181491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8cddd36ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381903678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12639830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8aa213fff","size":5,"soft_fail":false,"target_va":5369270288,"va":5381910113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008d710
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12620251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8256e3fff","size":5,"soft_fail":false,"target_va":5369288464,"va":5381908710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12575290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8c61d40ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5381910293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12567765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e82b3b40ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381906608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8936159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e821a577ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5381903802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ff6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3763180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e81494c6ff","size":5,"soft_fail":false,"target_va":5378143968,"va":5381907143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096cc40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3314301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8836dcdff","size":5,"soft_fail":false,"target_va":5378591808,"va":5381906104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3176973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8f385cfff","size":5,"soft_fail":false,"target_va":5378733248,"va":5381910216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3114516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8ec79d0ff","size":5,"soft_fail":false,"target_va":5378794960,"va":5381909471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b1d970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1540019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e84d80e8ff","size":5,"soft_fail":false,"target_va":5380364656,"va":5381904670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152d400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9006545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8d16d8900","size":5,"soft_fail":false,"target_va":5390914560,"va":5381908010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14164da40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10193075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8b3889b00","size":5,"soft_fail":false,"target_va":5392095808,"va":5381902728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14164dc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10190771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8b37f9b00","size":5,"soft_fail":false,"target_va":5392096288,"va":5381905512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141813480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12049962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e82adeb700","size":5,"soft_fail":false,"target_va":5393953920,"va":5381903953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34478627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8231a0e02","size":5,"soft_fail":false,"target_va":5416385392,"va":5381906760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34855893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8d5db1302","size":5,"soft_fail":false,"target_va":5416761616,"va":5381905718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34861789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8ddf21302","size":5,"soft_fail":false,"target_va":5416769200,"va":5381907406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35153613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8cd661802","size":5,"soft_fail":false,"target_va":5417057040,"va":5381903422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35153329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8b1651802","size":5,"soft_fail":false,"target_va":5417057056,"va":5381903722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2889178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8da152c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5381905681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34459969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e841d10d02","size":5,"soft_fail":false,"target_va":5416365072,"va":5381905098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34715331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8c3b61102","size":5,"soft_fail":false,"target_va":5416620794,"va":5381905458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t218728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e868560300","size":5,"soft_fail":false,"target_va":5382122800,"va":5381904067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-35636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8cc74ffff","size":5,"soft_fail":false,"target_va":5381872416,"va":5381908047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2926610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e812a82c00","size":5,"soft_fail":false,"target_va":5384830080,"va":5381903465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t34499403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e84b6b0e02","size":5,"soft_fail":false,"target_va":5416402080,"va":5381902672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter::closure$1
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t34499053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8ed690e02","size":5,"soft_fail":false,"target_va":5416401824,"va":5381902766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter::closure$4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t34496851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e853610e02","size":5,"soft_fail":false,"target_va":5416402336,"va":5381905480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter::closure$8
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t34494571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e86b580e02","size":5,"soft_fail":false,"target_va":5416402208,"va":5381907632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2849294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e80e7a2b00","size":5,"soft_fail":false,"target_va":5384751936,"va":5381902637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::canonicalize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2818745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8b9022b00","size":5,"soft_fail":false,"target_va":5384722976,"va":5381904226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2820199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e867082b00","size":5,"soft_fail":false,"target_va":5384724912,"va":5381904708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2821267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8930c2b00","size":5,"soft_fail":false,"target_va":5384725152,"va":5381903880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2818720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381902480,"raw_hex":"e8a0022b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5381907963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":23916,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":28160,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":612,"record_offset":18680,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":16496,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":21348,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":17240,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":17296,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":612,"record_offset":24000,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":17496,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":16567,"source_line":225,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":19480,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":19424,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":17832,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":18312,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":17588,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":23572,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":21292,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":17684,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":19544,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":16360,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":16400,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":612,"record_offset":19280,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":612,"record_offset":16524,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":612,"record_offset":20104,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":38921,"source_line":1570,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":20568,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":8115,"source_line":3035,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":21196,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":16304,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":612,"record_offset":19364,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":612,"record_offset":23384,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":17357,"source_line":1005,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":612,"record_offset":30220,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":16248,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":17468,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":11098,"source_line":2383,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":612,"record_offset":21320,"parent_record_offset":15980,"ancestor_record_offset":15980,"inlinee":29159,"source_line":47,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":15980,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":15980,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":20452,"parent_record_offset":20428,"ancestor_record_offset":15980,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":20476,"parent_record_offset":20452,"ancestor_record_offset":15980,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":15980,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":20500,"parent_record_offset":20476,"ancestor_record_offset":15980,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":19076,"parent_record_offset":18932,"ancestor_record_offset":15980,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":20428,"parent_record_offset":20372,"ancestor_record_offset":15980,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":15980,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":18872,"parent_record_offset":18844,"ancestor_record_offset":15980,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":20400,"parent_record_offset":20372,"ancestor_record_offset":15980,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":15980,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":18932,"parent_record_offset":18900,"ancestor_record_offset":15980,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":18900,"parent_record_offset":18872,"ancestor_record_offset":15980,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":15980,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":612,"record_offset":18964,"parent_record_offset":18932,"ancestor_record_offset":15980,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":18752,"parent_record_offset":18708,"ancestor_record_offset":15980,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":18780,"parent_record_offset":18752,"ancestor_record_offset":15980,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":18808,"parent_record_offset":18780,"ancestor_record_offset":15980,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":18708,"parent_record_offset":18680,"ancestor_record_offset":15980,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":612,"record_offset":28188,"parent_record_offset":28160,"ancestor_record_offset":15980,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":612,"record_offset":19772,"parent_record_offset":19736,"ancestor_record_offset":15980,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":612,"record_offset":19736,"parent_record_offset":19712,"ancestor_record_offset":15980,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":21376,"parent_record_offset":21348,"ancestor_record_offset":15980,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":18844,"parent_record_offset":18808,"ancestor_record_offset":15980,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":612,"record_offset":30640,"parent_record_offset":30616,"ancestor_record_offset":15980,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":24392,"parent_record_offset":24364,"ancestor_record_offset":15980,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":22556,"parent_record_offset":22528,"ancestor_record_offset":15980,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":612,"record_offset":22984,"parent_record_offset":22960,"ancestor_record_offset":15980,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":612,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":15980,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":612,"record_offset":17092,"parent_record_offset":17064,"ancestor_record_offset":15980,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":612,"record_offset":16992,"parent_record_offset":16788,"ancestor_record_offset":15980,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":612,"record_offset":17716,"parent_record_offset":17684,"ancestor_record_offset":15980,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":612,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":15980,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":612,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":15980,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":612,"record_offset":28252,"parent_record_offset":28216,"ancestor_record_offset":15980,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":612,"record_offset":19036,"parent_record_offset":19012,"ancestor_record_offset":15980,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":612,"record_offset":18540,"parent_record_offset":18456,"ancestor_record_offset":15980,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":612,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":15980,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":612,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":15980,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":612,"record_offset":21120,"parent_record_offset":21092,"ancestor_record_offset":15980,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":612,"record_offset":24440,"parent_record_offset":24416,"ancestor_record_offset":15980,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":612,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":15980,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":15980,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":15980,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::count::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":17552,"parent_record_offset":17520,"ancestor_record_offset":15980,"inlinee":16576,"source_line":232,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":17520,"parent_record_offset":17496,"ancestor_record_offset":15980,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":21092,"parent_record_offset":20848,"ancestor_record_offset":15980,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18456,"parent_record_offset":18432,"ancestor_record_offset":15980,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":612,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":15980,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":15980,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":26464,"parent_record_offset":26436,"ancestor_record_offset":15980,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":23128,"parent_record_offset":23096,"ancestor_record_offset":15980,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":612,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":15980,"inlinee":33968,"source_line":850,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":612,"record_offset":17120,"parent_record_offset":17064,"ancestor_record_offset":15980,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":612,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":15980,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":612,"record_offset":19508,"parent_record_offset":19480,"ancestor_record_offset":15980,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":612,"record_offset":16700,"parent_record_offset":16664,"ancestor_record_offset":15980,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":20948,"parent_record_offset":20924,"ancestor_record_offset":15980,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":23940,"parent_record_offset":23916,"ancestor_record_offset":15980,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":16820,"parent_record_offset":16788,"ancestor_record_offset":15980,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":612,"record_offset":25540,"parent_record_offset":25456,"ancestor_record_offset":15980,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":612,"record_offset":19204,"parent_record_offset":18808,"ancestor_record_offset":15980,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":19228,"parent_record_offset":19204,"ancestor_record_offset":15980,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=171
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":19572,"parent_record_offset":19544,"ancestor_record_offset":15980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":612,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":15980,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":612,"record_offset":25996,"parent_record_offset":25972,"ancestor_record_offset":15980,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":612,"record_offset":16908,"parent_record_offset":16788,"ancestor_record_offset":15980,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":612,"record_offset":25848,"parent_record_offset":25824,"ancestor_record_offset":15980,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":17380,"parent_record_offset":17296,"ancestor_record_offset":15980,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":15980,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":17408,"parent_record_offset":17296,"ancestor_record_offset":15980,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":17344,"parent_record_offset":17296,"ancestor_record_offset":15980,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":15980,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":15980,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":21444,"parent_record_offset":21376,"ancestor_record_offset":15980,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":612,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":15980,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":15980,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":612,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":15980,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":612,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":15980,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":612,"record_offset":28216,"parent_record_offset":28188,"ancestor_record_offset":15980,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$9::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":612,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":15980,"inlinee":33965,"source_line":316,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":612,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":15980,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":612,"record_offset":19832,"parent_record_offset":19772,"ancestor_record_offset":15980,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":612,"record_offset":19864,"parent_record_offset":19736,"ancestor_record_offset":15980,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":612,"record_offset":17948,"parent_record_offset":17920,"ancestor_record_offset":15980,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::discover_mod_clutter::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":17616,"parent_record_offset":17588,"ancestor_record_offset":15980,"inlinee":38916,"source_line":315,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::discover_mod_clutter::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":15980,"inlinee":38919,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::is_safe_clutter_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":15980,"inlinee":33963,"source_line":390,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::is_safe_clutter_stem::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":18588,"parent_record_offset":18564,"ancestor_record_offset":15980,"inlinee":33967,"source_line":394,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":612,"record_offset":25672,"parent_record_offset":25644,"ancestor_record_offset":15980,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":612,"record_offset":25704,"parent_record_offset":25672,"ancestor_record_offset":15980,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":612,"record_offset":25644,"parent_record_offset":25604,"ancestor_record_offset":15980,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":612,"record_offset":25484,"parent_record_offset":25456,"ancestor_record_offset":15980,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25972,"parent_record_offset":25368,"ancestor_record_offset":15980,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25824,"parent_record_offset":25604,"ancestor_record_offset":15980,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25456,"parent_record_offset":25400,"ancestor_record_offset":15980,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25604,"parent_record_offset":25576,"ancestor_record_offset":15980,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":26376,"parent_record_offset":26344,"ancestor_record_offset":15980,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25428,"parent_record_offset":25400,"ancestor_record_offset":15980,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25368,"parent_record_offset":25308,"ancestor_record_offset":15980,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25308,"parent_record_offset":25284,"ancestor_record_offset":15980,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":26344,"parent_record_offset":25308,"ancestor_record_offset":15980,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25340,"parent_record_offset":25308,"ancestor_record_offset":15980,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25400,"parent_record_offset":25368,"ancestor_record_offset":15980,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":26408,"parent_record_offset":26376,"ancestor_record_offset":15980,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25284,"parent_record_offset":25260,"ancestor_record_offset":15980,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":612,"record_offset":25576,"parent_record_offset":25368,"ancestor_record_offset":15980,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":612,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":15980,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":612,"record_offset":16552,"parent_record_offset":16524,"ancestor_record_offset":15980,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":612,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":15980,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":612,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":15980,"inlinee":14923,"source_line":1498,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":612,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":15980,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":612,"record_offset":16868,"parent_record_offset":16844,"ancestor_record_offset":15980,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":612,"record_offset":16728,"parent_record_offset":16700,"ancestor_record_offset":15980,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":612,"record_offset":17064,"parent_record_offset":16664,"ancestor_record_offset":15980,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":612,"record_offset":16580,"parent_record_offset":16552,"ancestor_record_offset":15980,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":612,"record_offset":23432,"parent_record_offset":23384,"ancestor_record_offset":15980,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":612,"record_offset":23464,"parent_record_offset":23432,"ancestor_record_offset":15980,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":612,"record_offset":23504,"parent_record_offset":23464,"ancestor_record_offset":15980,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":17644,"parent_record_offset":17616,"ancestor_record_offset":15980,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":20924,"parent_record_offset":20848,"ancestor_record_offset":15980,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":20896,"parent_record_offset":20848,"ancestor_record_offset":15980,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":20848,"parent_record_offset":20820,"ancestor_record_offset":15980,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$76::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":20820,"parent_record_offset":20792,"ancestor_record_offset":15980,"inlinee":34013,"source_line":3652,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::io::error::windows::decode_error_kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\io\error\windows.rs
//     evidence={"module":612,"record_offset":23540,"parent_record_offset":23384,"ancestor_record_offset":15980,"inlinee":17334,"source_line":13,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":612,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":15980,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":612,"record_offset":16756,"parent_record_offset":16728,"ancestor_record_offset":15980,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":612,"record_offset":16788,"parent_record_offset":16756,"ancestor_record_offset":15980,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":612,"record_offset":16664,"parent_record_offset":16636,"ancestor_record_offset":15980,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":612,"record_offset":16636,"parent_record_offset":16608,"ancestor_record_offset":15980,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":39268,"rva":47692960,"va":5416402080}
// 编译实例 RVAs: 0x2d7bca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47063318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402080,"raw_hex":"e8eade31fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416402161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402080,"raw_hex":"e887beffff","size":5,"soft_fail":false,"target_va":5416385392,"va":5416402148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":38552,"rva":47692704,"va":5416401824}
// 编译实例 RVAs: 0x2d7bba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-40723251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416401824,"raw_hex":"e8cd9c92fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416401950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47063118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416401824,"raw_hex":"e8b2df31fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416401961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47058568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416401824,"raw_hex":"e878f131fd","size":5,"soft_fail":false,"target_va":5369343504,"va":5416402067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416401824,"raw_hex":"e887bfffff","size":5,"soft_fail":false,"target_va":5416385392,"va":5416401892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":38708,"parent_record_offset":38552,"ancestor_record_offset":38552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":38872,"parent_record_offset":38848,"ancestor_record_offset":38552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":38896,"parent_record_offset":38872,"ancestor_record_offset":38552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":38920,"parent_record_offset":38896,"ancestor_record_offset":38552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":612,"record_offset":38848,"parent_record_offset":38820,"ancestor_record_offset":38552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":612,"record_offset":38820,"parent_record_offset":38796,"ancestor_record_offset":38552,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":612,"record_offset":38732,"parent_record_offset":38708,"ancestor_record_offset":38552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9804
// 首条位置: {"module":612,"inlinee":38916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":612,"record_offset":17644,"parent_record_offset":17616,"ancestor_record_offset":17616,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 323
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9807
// 首条位置: {"module":612,"inlinee":38919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::clutter_packs::is_safe_clutter_stem
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":18336,"inlinee":33963,"source_line":390,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":612,"record_offset":18540,"parent_record_offset":18456,"ancestor_record_offset":18336,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":612,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":18336,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":18336,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":18336,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":612,"record_offset":18456,"parent_record_offset":18432,"ancestor_record_offset":18336,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":612,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":18336,"inlinee":33968,"source_line":850,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":612,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":18336,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":612,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":18336,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$9::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":612,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":18336,"inlinee":33965,"source_line":316,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::is_safe_clutter_stem::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":612,"record_offset":18588,"parent_record_offset":18564,"ancestor_record_offset":18336,"inlinee":33967,"source_line":394,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":39564,"rva":47693216,"va":5416402336}
// 编译实例 RVAs: 0x2d7bda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t359208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402336,"raw_hex":"e8287b0500","size":5,"soft_fail":false,"target_va":5416761616,"va":5416402403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31607536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402336,"raw_hex":"e810b51dfe","size":5,"soft_fail":false,"target_va":5384794864,"va":5416402395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":39720,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":612,"record_offset":39768,"parent_record_offset":39744,"ancestor_record_offset":39564,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":612,"record_offset":39744,"parent_record_offset":39720,"ancestor_record_offset":39564,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1439F
// 首条位置: {"module":1682,"inlinee":82847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":132940,"parent_record_offset":132912,"ancestor_record_offset":132912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":133048,"parent_record_offset":133012,"ancestor_record_offset":132912,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":133012,"parent_record_offset":132988,"ancestor_record_offset":132912,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":132964,"parent_record_offset":132940,"ancestor_record_offset":132912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":133080,"parent_record_offset":133048,"ancestor_record_offset":132912,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":133108,"parent_record_offset":133048,"ancestor_record_offset":132912,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":133140,"parent_record_offset":133012,"ancestor_record_offset":132912,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 344
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143A0
// 首条位置: {"module":1682,"inlinee":82848}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1682,"record_offset":133492,"parent_record_offset":133464,"ancestor_record_offset":133464,"inlinee":29126,"source_line":1253,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1682,"record_offset":133640,"parent_record_offset":133612,"ancestor_record_offset":133464,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1682,"record_offset":133692,"parent_record_offset":133668,"ancestor_record_offset":133464,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1682,"record_offset":133668,"parent_record_offset":133640,"ancestor_record_offset":133464,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":1682,"record_offset":133576,"parent_record_offset":133548,"ancestor_record_offset":133464,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":1682,"record_offset":133612,"parent_record_offset":133576,"ancestor_record_offset":133464,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1682,"record_offset":133548,"parent_record_offset":133520,"ancestor_record_offset":133464,"inlinee":29087,"source_line":351,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1682,"record_offset":133520,"parent_record_offset":133492,"ancestor_record_offset":133464,"inlinee":29086,"source_line":317,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$8
// 记录形式: compiled_procedure; instances=1
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":612,"record_offset":39416,"rva":47693088,"va":5416402208}
// 编译实例 RVAs: 0x2d7bd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47063446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402208,"raw_hex":"e86ade31fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416402289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416402208,"raw_hex":"e807beffff","size":5,"soft_fail":false,"target_va":5416385392,"va":5416402276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::discover_mod_clutter::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B3F
// 首条位置: {"module":528,"inlinee":31551}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":528,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":33112,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":33160,"parent_record_offset":33136,"ancestor_record_offset":33112,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":528,"record_offset":33208,"parent_record_offset":33184,"ancestor_record_offset":33112,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":528,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":33112,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::is_safe_clutter_stem
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 390
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84AB
// 首条位置: {"module":546,"inlinee":33963}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":546,"record_offset":71884,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$9::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":546,"record_offset":71912,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":33965,"source_line":316,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":546,"record_offset":72068,"parent_record_offset":71984,"ancestor_record_offset":71856,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":546,"record_offset":71960,"parent_record_offset":71936,"ancestor_record_offset":71856,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":71936,"parent_record_offset":71912,"ancestor_record_offset":71856,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":72092,"parent_record_offset":72068,"ancestor_record_offset":71856,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":71984,"parent_record_offset":71960,"ancestor_record_offset":71856,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_lowercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":546,"record_offset":72140,"parent_record_offset":72116,"ancestor_record_offset":71856,"inlinee":33968,"source_line":850,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":546,"record_offset":72036,"parent_record_offset":72012,"ancestor_record_offset":71856,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":546,"record_offset":72012,"parent_record_offset":71984,"ancestor_record_offset":71856,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::is_safe_clutter_stem::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":546,"record_offset":72116,"parent_record_offset":72092,"ancestor_record_offset":71856,"inlinee":33967,"source_line":394,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::clutter_packs::is_safe_clutter_stem::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84AF
// 首条位置: {"module":546,"inlinee":33967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$6::is_ascii_lowercase
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":546,"record_offset":72140,"parent_record_offset":72116,"ancestor_record_offset":72116,"inlinee":33968,"source_line":850,"archive":"_pdb/C/modules/00546.jsonl.gz"}
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
// 完整结构及原始类型记录: clutter_packs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::ClutterManifest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::clutter_packs::ClutterManifest>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::clutter_packs::ClutterManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<country_core::resources::clutter_packs::ClutterManifest>", "constructor": "serde_json::de::from_slice", "arguments": ["country_core::resources::clutter_packs::ClutterManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::resources::clutter_packs::ClutterManifest,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::resources::clutter_packs::ClutterManifest", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::resources::clutter_packs::ClutterManifest,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::resources::clutter_packs::ClutterManifest,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::clutter_packs::ClutterManifest,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::resources::clutter_packs::ClutterManifest,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::ClutterManifestItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>", "tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >", "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >", "core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>", "tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<core::option::Option<enum2$<country_core::resources::clutter_diagnostics::ClutterItemStatus> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterManifestItem>,tiny_glade::gui::gui_mod_editor::item_statuses::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global,country_core::systems::mod_editor::impl$2::sync::closure_env$3>", "constructor": "alloc::vec::Vec::retain", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem", "alloc::alloc::Global", "country_core::systems::mod_editor::impl$2::sync::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global,country_core::systems::mod_editor::impl$2::sync::closure_env$3>", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem", "alloc::alloc::Global", "country_core::systems::mod_editor::impl$2::sync::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::clutter_packs::ClutterManifestItem>", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::clutter_packs::ClutterManifestItem> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::SliceRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::clutter_packs::ClutterManifestItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::clutter_packs::ClutterManifestItem,serde_json::de::SeqAccess<serde_json::read::SliceRead> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::clutter_packs::ClutterManifestItem", "serde_json::de::SeqAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::ClutterPackObject
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 28
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::clutter_packs::ClutterPackObject>", "constructor": "ref$", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "constructor": "tuple$", "arguments": ["ref$<country_core::resources::clutter_packs::ClutterPackObject>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["alloc::string::String", "ref$<country_core::resources::clutter_packs::ClutterPackObject>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::slice::iter::Iter<tg_mods::overlay::Mod>", "enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)> >", "constructor": "core::iter::adapters::flatten::Flatten", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::clutter_packs::ClutterPackObject>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>,alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>", "alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "constructor": "tuple$", "arguments": ["alloc::string::String", "ref$<country_core::resources::clutter_packs::ClutterPackObject>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$11::next<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)>,alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >", "constructor": "core::iter::adapters::flatten::impl$11::next", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)>", "alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$1>", "constructor": "alloc::vec::Vec::retain", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::alloc::Global", "country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>", "country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::clutter_packs::ClutterPackObject,core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global>,country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<std::path::PathBuf,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["std::path::PathBuf", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::clutter_packs::ClutterPackObject,core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "core::iter::adapters::flatten::Flatten<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>", "country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >", "constructor": "hashbrown::set::impl$10::extend::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "ref$<country_core::resources::clutter_packs::ClutterPackObject>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$6>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::set_clutter_ids<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$6> >", "constructor": "country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::set_clutter_ids", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::startup::load_clutter_packs::build_requests::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["clutter_pack_pipe::ClutterPackBuildRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["clutter_pack_pipe::ClutterPackBuildRequest", "alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<clutter_pack_pipe::ClutterPackBuildRequest,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["clutter_pack_pipe::ClutterPackBuildRequest", "core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::startup::load_clutter_packs::build_requests::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["alloc::string::String", "ref$<country_core::resources::clutter_packs::ClutterPackObject>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >", "core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>", "country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >", "tuple$<>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > >", "constructor": "core::iter::adapters::flatten::impl$20::fold::flatten::closure$0", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,ref_mut$<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6> >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "ref_mut$<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,ref_mut$<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6> >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> > > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,ref_mut$<country_core::startup::mod_hot_reload::reload_changed_clutter::closure_env$6> >", "enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_packs::ClutterPackObject,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::resources::clutter_packs::ClutterPackObject", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::ClutterPackRoot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::clutter_packs::ClutterPackRoot>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::clutter_packs::ClutterPackRoot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::PackChecksums
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<country_core::resources::clutter_packs::PackChecksums>", "constructor": "serde_json::de::from_slice", "arguments": ["country_core::resources::clutter_packs::PackChecksums"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::resources::clutter_packs::PackChecksums,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::resources::clutter_packs::PackChecksums", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::resources::clutter_packs::PackChecksums,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::resources::clutter_packs::PackChecksums,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::clutter_packs::PackChecksums,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::resources::clutter_packs::PackChecksums,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::WorkshopItemId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 39
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::string::String"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::path::PathBuf", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$62::default<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "alloc::sync::impl$62::default", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::path::PathBuf"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::resources::clutter_packs::WorkshopItemId", "enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::resources::clutter_packs::WorkshopItemId", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,tuple$<> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::string::String", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::path::PathBuf", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tg_mods::overlay::ModParts", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tg_mods::overlay::ModParts", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>", "constructor": "tuple$", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tg_mods::overlay::ModParts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tg_mods::overlay::ModParts", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>", "country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["country_core::systems::steam::workshop_subscriptions::WorkshopSubscription", "core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>", "country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<country_core::resources::clutter_packs::WorkshopItemId,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>", "country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::steam::workshop_subscriptions::WorkshopSubscription", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::resources::clutter_packs::WorkshopItemId>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$2>,country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>", "country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1>", "country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::clutter_packs::WorkshopItemId,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$1>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5> >", "constructor": "hashbrown::set::impl$10::extend::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<country_core::resources::clutter_packs::WorkshopItemId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5> > > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<steamworks::ugc::PublishedFileId,alloc::alloc::Global>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$5> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "enum2$<country_core::systems::steam::workshop_clutter::WorkshopModStatus>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "country_core::systems::steam::workshop_clutter::ProvidedWorkshopMod", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::path::PathBuf", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::clutter_packs::WorkshopItemId>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::clutter_packs::WorkshopItemId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "std::path::PathBuf", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<country_core::systems::steam::workshop_publish::LocalMod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,tuple$<>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> > > >", "constructor": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0", "arguments": ["country_core::systems::steam::workshop_publish::LocalMod", "tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>", "tuple$<>", "country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<country_core::systems::steam::workshop_publish::LocalMod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,tuple$<>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["country_core::systems::steam::workshop_publish::LocalMod", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<country_core::systems::steam::workshop_publish::LocalMod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,tuple$<>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,std::path::PathBuf,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global>,country_core::systems::steam::workshop_subscriptions::local_mod_sidecar_ids::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>", "alloc::alloc::Global", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<tuple$<country_core::resources::clutter_packs::WorkshopItemId,alloc::string::String>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::clutter_packs::WorkshopItemId", "tg_mods::overlay::ModParts", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > >", "constructor": "core::iter::adapters::filter_map::filter_map_fold::closure_env$0", "arguments": ["tg_mods::overlay::Mod", "tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>", "tuple$<>", "toolbar::menu::mods_workshop_tab::list_slot::closure_env$1", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<tg_mods::overlay::Mod,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > > >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["tg_mods::overlay::Mod", "alloc::alloc::Global", "tuple$<>", "core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tg_mods::overlay::Mod,tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,tuple$<>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_packs::WorkshopItemId,tg_mods::overlay::ModParts,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tg_mods::overlay::Mod,alloc::alloc::Global>,toolbar::menu::mods_workshop_tab::list_slot::closure_env$1> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::clutter_packs::WorkshopSidecar
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<country_core::resources::clutter_packs::WorkshopSidecar>", "constructor": "serde_json::de::from_slice", "arguments": ["country_core::resources::clutter_packs::WorkshopSidecar"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clutter_packs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clutter_packs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 51; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_publish::WorkshopPublishState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::components::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 04cc294091e7ac12a2f5ce00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::components::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::components::TreeStorageIndex> @ 0x18b9b12 -> 0x1c09630 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::clutter_packs::WorkshopItemId> @ 0x18bbbc9 -> 0x1c09630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::ClutterPackObject::id
// 信息 ID: 56ffa95a7bf21cde653acd61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e6a64c8cb4d0ae4c00079711: country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2dfc8 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2e4c3 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$1> @ 0x15f30fc -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$1> @ 0x15f33e0 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::set_clutter_ids<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install::closure_env$6> > @ 0x17fece3 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<clutter_pack_pipe::ClutterPackBuildRequest,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > > > @ 0x180dd09 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > > @ 0x1810c27 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > > @ 0x1810cb9 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > > @ 0x1816643 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > > @ 0x1816683 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > > @ 0x18166d3 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::InFlightPackBuild::poll @ 0x1adcf09 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1ade245 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1ade424 -> 0xc94fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::WorkshopSidecar::read
// 信息 ID: be6a693c3a1b3f402a7d0581
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa203a -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_mods::register_workshop_mod @ 0xafe112 -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods @ 0xafe869 -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup @ 0x1ae2adf -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::try_fold<tg_mods::overlay::Mod,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,core::iter::adapters::filter::filter_try_fold::closure_env$0<tg_mods::overlay::Mod,alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,never$> >,country_core::systems::steam::workshop_publish::scan_local_mods::closure_env$0,core::iter::adapters::map::map_try_fold::closure_env$0<tg_mods::overlay::Mod,country_core::systems::steam::workshop_publish::LocalMod,alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,never$> >,country_core::systems::steam::workshop_publish::scan_local_mods::closure_env$1,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::systems::steam::workshop_publish::LocalMod> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::systems::steam::workshop_publish::LocalMod>,never$> > > @ 0x1c36ff3 -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a456db -> 0xc94dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize
// 信息 ID: 91f576e4d4e09f32140a3d6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<country_core::resources::clutter_packs::PackChecksums>; module=1645, inline_record=10688; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::from_slice<country_core::resources::clutter_packs::ClutterManifest>; module=1645, inline_record=13156; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::from_slice<country_core::resources::clutter_packs::WorkshopSidecar>; module=1645, inline_record=14920; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=237736; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=237828; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=238780; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::clutter_packs::ClutterManifestItem,serde_json::de::SeqAccess<serde_json::read::SliceRead> >; module=1665, inline_record=25176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 8539cd440d1bba118d206613
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=237176; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=382836; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=447700; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=469492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: c961ff0e8bc16d54226cd3d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=237000; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=382660; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=447520; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=469316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 691139c8f0981d39fbabdde6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1812, inline_record=11952; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1812, inline_record=12168; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1812, inline_record=12384; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1812, inline_record=12600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 032d3f299ef3036ed3b316ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=236844; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=382512; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=447360; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=469172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 7c18e8d43ca956e97777e5da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=238656; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=384404; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=451008; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,country_core::resources::clutter_packs::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=470328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: ecdca020a6cb4d485337d156
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::clutter_packs::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::write_pack_checksums @ 0xc97557 -> 0x1c09790 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::components::TreeStorageIndex> @ 0x18b9b12 -> 0x1c09630 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::clutter_packs::WorkshopItemId> @ 0x18bbbc9 -> 0x1c09630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mod_editor::write_manifest @ 0xaa01ac -> 0x1c09970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::drive_workshop_publish @ 0xb01e45 -> 0x1c09b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,alloc::vec::Vec<country_core::resources::clutter_packs::ClutterManifestItem,alloc::alloc::Global> > @ 0x18b8a0a -> 0x1c09d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::collect_files
// 信息 ID: eaa7ca99199c208606d296a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::collect_files @ 0xc94ab1 -> 0xc94930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::clutter_packs::write_pack_checksums @ 0xc96fad -> 0xc94930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter
// 信息 ID: 0bab7ac34aed18a25b929b3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa33cc -> 0xc95090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::discover_clutter_part @ 0x1add434 -> 0xc95090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a422af -> 0xc95090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$0
// 信息 ID: 4a54efa4bcb7d1f16f9c1c5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc95150 -> 0x2d7bca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::clutter_packs::write_pack_checksums @ 0xc9760a -> 0x2d7bca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$1
// 信息 ID: a2b2207dde2996bd8614fafd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc951ae -> 0x2d7bba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc95c89 -> 0x2d7bba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$10
// 信息 ID: da265d3d5008bedb194c5638
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::clutter_packs::discover_mod_clutter; module=612, inline_record=17616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$3
// 信息 ID: 86d8280feb06eb60fa43f829
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::clutter_packs::discover_mod_clutter; module=612, inline_record=18336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$4
// 信息 ID: 291ce2b02ce2b9979dbfcbe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc95c48 -> 0x2d7bda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$5
// 信息 ID: e8b21ca8c9584b6293283195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5 <- core::iter::adapters::map::impl$2::try_fold<std::ffi::os_str::OsString,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::ffi::os_str::OsString,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$6>,enum2$<core::ops::control_flow::ControlFlow<std::ffi::os_str::OsString,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::ffi::os_str::OsString,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::ffi::os_str::OsString,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$6>,enum2$<core::ops::control_flow::ControlFlow<std::ffi::os_str::OsString,tuple$<> > > >; module=1682, inline_record=132912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$6
// 信息 ID: 6ee5da5c6f8ed53f66e4152d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5008ae74692ef3f813781806: enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$6 <- core::iter::adapters::map::impl$2::try_fold<std::ffi::os_str::OsString,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::ffi::os_str::OsString,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$6>,enum2$<core::ops::control_flow::ControlFlow<std::ffi::os_str::OsString,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<std::ffi::os_str::OsString,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)>,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<std::ffi::os_str::OsString,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$6>,enum2$<core::ops::control_flow::ControlFlow<std::ffi::os_str::OsString,tuple$<> > > >; module=1682, inline_record=133464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$8
// 信息 ID: 187c61b148974267b8e15e81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc964e1 -> 0x2d7bd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::clutter_packs::discover_mod_clutter @ 0xc964b0 -> 0x2d7bd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::discover_mod_clutter::closure$9
// 信息 ID: 18abad2791da1b92b446d38e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "country_core::resources::clutter_packs::discover_mod_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::shared::pivot::median3_rec<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::stable::driftsort_main<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9>,alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::stable::drift::sort<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9 <- core::slice::sort::stable::quicksort::quicksort<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=528, inline_record=33112; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=528, inline_record=33288; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=528, inline_record=33464; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=1639, inline_record=17124; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=1639, inline_record=17292; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=1639, inline_record=17608; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=1639, inline_record=17816; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::clutter_packs::ClutterPackObject,alloc::slice::impl$0::sort_by::closure_env$0<country_core::resources::clutter_packs::ClutterPackObject,country_core::resources::clutter_packs::discover_mod_clutter::closure_env$9> >; module=1639, inline_record=18192; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::impl$0::fmt
// 信息 ID: 62ff351f23a5f7d9cb8dc1f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::impl$12::eq
// 信息 ID: f800b1a4213bff8028e3c095
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods; module=559, inline_record=27688; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions::title; module=566, inline_record=49108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions; module=566, inline_record=58956; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions; module=566, inline_record=64180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_subscriptions::refresh_workshop_subscriptions::closure$6; module=566, inline_record=84860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::status; module=586, inline_record=21396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install; module=586, inline_record=27564; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install; module=586, inline_record=32004; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::is_safe_clutter_stem
// 信息 ID: 31acea48dcb49e96d2271230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::mod_editor::mod_editor_tick; module=546, inline_record=71856; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::clutter_packs::discover_mod_clutter; module=612, inline_record=18360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::is_safe_clutter_stem::closure$0
// 信息 ID: 23531fce42638c868dbbd8c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::clutter_packs::is_safe_clutter_stem", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::mod_editor::mod_editor_tick; module=546, inline_record=72116; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::clutter_packs::discover_mod_clutter; module=612, inline_record=18588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::is_safe_relative_path
// 信息 ID: 37495178d28d3e44ba7a0a2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::clutter_packs::write_pack_checksums @ 0xc9729e -> 0xc97dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::clutter_packs::verify_pack_checksums @ 0xc98311 -> 0xc97dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::is_safe_relative_path::closure$0
// 信息 ID: a10e205f229471593d4daa45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::clutter_packs::is_safe_relative_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::clutter_packs::is_safe_relative_path; module=612, inline_record=51400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::verify_pack_checksums
// 信息 ID: a447fa98ca649b951740d98b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::workshop_mods::register_workshop_mod @ 0xafd937 -> 0xc97f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::write_pack_checksums
// 信息 ID: 433cc0b38b3aa450d5ee7a96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a42438 -> 0xc96f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::write_pack_checksums::closure$0
// 信息 ID: 492a3ef8554e321c217d499f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eae8d4217b8f03d2eaf2d900: enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::clutter_packs::write_pack_checksums", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::clutter_packs::write_pack_checksums::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,alloc::string::String,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$26::from_iter::closure_env$0<alloc::string::String,anyhow::Error,alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>,core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0> >,alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::write_pack_checksums::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,alloc::string::String,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$26::from_iter::closure_env$0<alloc::string::String,anyhow::Error,alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>,core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0> >,alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::write_pack_checksums::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::write_pack_checksums::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<alloc::string::String,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)>,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::clutter_packs::write_pack_checksums::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<alloc::string::String,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)>,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<alloc::string::String,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)>,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=174852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::clutter_packs::write_pack_checksums::closure$0::closure$0
// 信息 ID: 6397c56d55c1fb8bb7fe6dc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eae8d4217b8f03d2eaf2d900: enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::clutter_packs::write_pack_checksums", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<alloc::string::String,anyhow::Error> >,std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<std::path::Components,country_core::resources::clutter_packs::write_pack_checksums::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<alloc::string::String,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > (*)(alloc::string::String)>,enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<alloc::string::String,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=175668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
