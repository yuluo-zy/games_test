// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/history/wall_compound_edit.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1208eba1b8bc51da94fca24e03e97d768cd64522ef8962ae3e75483bde71f720
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_compound_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 38, 76, 144, 510, 520, 525, 535, 561, 564, 568, 569, 570, 574, 575, 577, 582, 1643, 1694, 1715, 1803, 1804, 1814, 1822, 1833
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use zmij; // namespace candidate
//   package identities: [{"name":"zmij","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.23"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 16, 22, 72, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15726
// 首条位置: {"module":1803,"inlinee":87846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":76864,"parent_record_offset":76840,"ancestor_record_offset":76840,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":87572,"parent_record_offset":87548,"ancestor_record_offset":87548,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":472332,"parent_record_offset":472312,"ancestor_record_offset":472312,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":472376,"parent_record_offset":472356,"ancestor_record_offset":472312,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":472356,"parent_record_offset":472332,"ancestor_record_offset":472312,"inlinee":87847,"source_line":16,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":76888,"parent_record_offset":76864,"ancestor_record_offset":76840,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":76912,"parent_record_offset":76888,"ancestor_record_offset":76840,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":87268,"parent_record_offset":87244,"ancestor_record_offset":87196,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 16, 22, 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15727
// 首条位置: {"module":1803,"inlinee":87847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":472376,"parent_record_offset":472356,"ancestor_record_offset":472356,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":87268,"parent_record_offset":87244,"ancestor_record_offset":87244,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 16, 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20A4
// 首条位置: {"module":76,"inlinee":8356}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_struct
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":18836,"parent_record_offset":18812,"ancestor_record_offset":18812,"inlinee":7390,"source_line":225,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$111::serialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":76,"record_offset":18860,"parent_record_offset":18836,"ancestor_record_offset":18812,"inlinee":8336,"source_line":468,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$8::serialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":76,"record_offset":18884,"parent_record_offset":18860,"ancestor_record_offset":18812,"inlinee":8338,"source_line":180,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":568,"record_offset":404,"rva":11820624,"va":5380529744}
// 编译实例 RVAs: 0xb45e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4287870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380529744,"raw_hex":"e87e6d4100","size":5,"soft_fail":false,"target_va":5384817664,"va":5380529789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$6::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D81
// 首条位置: {"module":1643,"inlinee":81281}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15B1B
// 首条位置: {"module":1833,"inlinee":88859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9106
// 首条位置: {"module":582,"inlinee":37126}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$16::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":582,"record_offset":101268,"parent_record_offset":101236,"ancestor_record_offset":101236,"inlinee":27063,"source_line":2041,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":582,"record_offset":101668,"parent_record_offset":101644,"ancestor_record_offset":101236,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":582,"record_offset":101692,"parent_record_offset":101668,"ancestor_record_offset":101236,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":582,"record_offset":101716,"parent_record_offset":101692,"ancestor_record_offset":101236,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":582,"record_offset":101644,"parent_record_offset":101464,"ancestor_record_offset":101236,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":582,"record_offset":101404,"parent_record_offset":101376,"ancestor_record_offset":101236,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":582,"record_offset":101464,"parent_record_offset":101432,"ancestor_record_offset":101236,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":582,"record_offset":101432,"parent_record_offset":101404,"ancestor_record_offset":101236,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":582,"record_offset":101500,"parent_record_offset":101464,"ancestor_record_offset":101236,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":582,"record_offset":101300,"parent_record_offset":101268,"ancestor_record_offset":101236,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":582,"record_offset":101332,"parent_record_offset":101300,"ancestor_record_offset":101236,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":582,"record_offset":101896,"parent_record_offset":101332,"ancestor_record_offset":101236,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":582,"record_offset":101376,"parent_record_offset":101332,"ancestor_record_offset":101236,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":582,"record_offset":101524,"parent_record_offset":101500,"ancestor_record_offset":101236,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":582,"record_offset":101604,"parent_record_offset":101548,"ancestor_record_offset":101236,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":582,"record_offset":101808,"parent_record_offset":101772,"ancestor_record_offset":101236,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":582,"record_offset":101772,"parent_record_offset":101332,"ancestor_record_offset":101236,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":582,"record_offset":101984,"parent_record_offset":101332,"ancestor_record_offset":101236,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":582,"record_offset":101548,"parent_record_offset":101524,"ancestor_record_offset":101236,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":582,"record_offset":101576,"parent_record_offset":101548,"ancestor_record_offset":101236,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":582,"record_offset":101856,"parent_record_offset":101832,"ancestor_record_offset":101236,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":582,"record_offset":101832,"parent_record_offset":101808,"ancestor_record_offset":101236,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":582,"record_offset":101924,"parent_record_offset":101896,"ancestor_record_offset":101236,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":582,"record_offset":101948,"parent_record_offset":101924,"ancestor_record_offset":101236,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=30
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7789
// 首条位置: {"module":520,"inlinee":30601}
// 编译实例 RVAs: 0xb8f7c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t294555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380831168,"raw_hex":"e89b7e0400","size":5,"soft_fail":false,"target_va":5381125776,"va":5380831216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":67476,"parent_record_offset":67444,"ancestor_record_offset":67444,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":520,"record_offset":67588,"parent_record_offset":67444,"ancestor_record_offset":67444,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":520,"record_offset":67532,"parent_record_offset":67444,"ancestor_record_offset":67444,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":520,"record_offset":67556,"parent_record_offset":67532,"ancestor_record_offset":67444,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":575,"record_offset":89800,"parent_record_offset":89776,"ancestor_record_offset":89600,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":575,"record_offset":89820,"parent_record_offset":89800,"ancestor_record_offset":89600,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":520,"record_offset":67500,"parent_record_offset":67476,"ancestor_record_offset":67444,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15928
// 首条位置: {"module":1822,"inlinee":88360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1822,"record_offset":40880,"parent_record_offset":40852,"ancestor_record_offset":40852,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":40852,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41496,"parent_record_offset":41472,"ancestor_record_offset":40852,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41332,"parent_record_offset":41308,"ancestor_record_offset":40852,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41356,"parent_record_offset":41332,"ancestor_record_offset":40852,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41520,"parent_record_offset":41496,"ancestor_record_offset":40852,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41448,"parent_record_offset":41176,"ancestor_record_offset":40852,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41308,"parent_record_offset":41176,"ancestor_record_offset":40852,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":41380,"parent_record_offset":41356,"ancestor_record_offset":40852,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41032,"parent_record_offset":41008,"ancestor_record_offset":40852,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41092,"parent_record_offset":40980,"ancestor_record_offset":40852,"inlinee":6565,"source_line":398,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41056,"parent_record_offset":41032,"ancestor_record_offset":40852,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41224,"parent_record_offset":41176,"ancestor_record_offset":40852,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41152,"parent_record_offset":41120,"ancestor_record_offset":40852,"inlinee":6567,"source_line":811,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41120,"parent_record_offset":41092,"ancestor_record_offset":40852,"inlinee":6566,"source_line":743,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":41176,"parent_record_offset":41152,"ancestor_record_offset":40852,"inlinee":6568,"source_line":827,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1822,"record_offset":41008,"parent_record_offset":40980,"ancestor_record_offset":40852,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1822,"record_offset":40944,"parent_record_offset":40912,"ancestor_record_offset":40852,"inlinee":6562,"source_line":1708,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1822,"record_offset":40980,"parent_record_offset":40944,"ancestor_record_offset":40852,"inlinee":6563,"source_line":1579,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1822,"record_offset":40912,"parent_record_offset":40880,"ancestor_record_offset":40852,"inlinee":6561,"source_line":4416,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1822,"record_offset":41248,"parent_record_offset":41224,"ancestor_record_offset":40852,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1822,"record_offset":41420,"parent_record_offset":41176,"ancestor_record_offset":40852,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=34
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x726B
// 首条位置: {"module":510,"inlinee":29291}
// 编译实例 RVAs: 0xb8f4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next_empty
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7785
// 首条位置: {"module":520,"inlinee":30597}
// 编译实例 RVAs: 0xb8f270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ae00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2180345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829808,"raw_hex":"e807bbdeff","size":5,"soft_fail":false,"target_va":5378649600,"va":5380829940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35521072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829808,"raw_hex":"e830021e02","size":5,"soft_fail":false,"target_va":5416350944,"va":5380829867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65056,"parent_record_offset":65024,"ancestor_record_offset":65024,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65084,"parent_record_offset":65024,"ancestor_record_offset":65024,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65192,"parent_record_offset":65160,"ancestor_record_offset":65024,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65240,"parent_record_offset":65216,"ancestor_record_offset":65024,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65216,"parent_record_offset":65192,"ancestor_record_offset":65024,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65160,"parent_record_offset":65116,"ancestor_record_offset":65024,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65116,"parent_record_offset":65084,"ancestor_record_offset":65024,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":520,"record_offset":65280,"parent_record_offset":65116,"ancestor_record_offset":65024,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::with_concurrent
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7786
// 首条位置: {"module":520,"inlinee":30598}
// 编译实例 RVAs: 0xb8f310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5151416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e84865b1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380830099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12107897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e8873f47ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380830084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a62c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2003325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e8836ee1ff","size":5,"soft_fail":false,"target_va":5378826944,"va":5380830264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8ff10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e84e0a0000","size":5,"soft_fail":false,"target_va":5380833040,"va":5380830397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1060390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e8262e1000","size":5,"soft_fail":false,"target_va":5381890720,"va":5380830325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35520561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e831001e02","size":5,"soft_fail":false,"target_va":5416350720,"va":5380830154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36226988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e8acc72802","size":5,"soft_fail":false,"target_va":5417057056,"va":5380830063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35790545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e8d11e2202","size":5,"soft_fail":false,"target_va":5416620769,"va":5380830219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35791394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380829968,"raw_hex":"e822222202","size":5,"soft_fail":false,"target_va":5416621600,"va":5380830201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":520,"record_offset":65968,"parent_record_offset":65504,"ancestor_record_offset":65504,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65548,"parent_record_offset":65504,"ancestor_record_offset":65504,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":520,"record_offset":65932,"parent_record_offset":65504,"ancestor_record_offset":65504,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":575,"record_offset":86924,"parent_record_offset":86736,"ancestor_record_offset":86736,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":66104,"parent_record_offset":66080,"ancestor_record_offset":65504,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":66128,"parent_record_offset":66104,"ancestor_record_offset":65504,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":66152,"parent_record_offset":66128,"ancestor_record_offset":65504,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":520,"record_offset":66080,"parent_record_offset":66048,"ancestor_record_offset":65504,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":520,"record_offset":66048,"parent_record_offset":66004,"ancestor_record_offset":65504,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":520,"record_offset":66004,"parent_record_offset":65968,"ancestor_record_offset":65504,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65656,"parent_record_offset":65632,"ancestor_record_offset":65504,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65704,"parent_record_offset":65680,"ancestor_record_offset":65504,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":520,"record_offset":65680,"parent_record_offset":65656,"ancestor_record_offset":65504,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":520,"record_offset":65796,"parent_record_offset":65772,"ancestor_record_offset":65504,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65632,"parent_record_offset":65584,"ancestor_record_offset":65504,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":65584,"parent_record_offset":65548,"ancestor_record_offset":65504,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":520,"record_offset":65772,"parent_record_offset":65584,"ancestor_record_offset":65504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":520,"record_offset":65744,"parent_record_offset":65584,"ancestor_record_offset":65504,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":520,"record_offset":65832,"parent_record_offset":65796,"ancestor_record_offset":65504,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":520,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65504,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":520,"record_offset":65888,"parent_record_offset":65796,"ancestor_record_offset":65504,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":87784,"rva":12121328,"va":5380830448}
// 编译实例 RVAs: 0xb8f4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5151816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e8b863b1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380830499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5152123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e88562b1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380830822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12108297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e8f73d47ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380830484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ae00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2181341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e823b7deff","size":5,"soft_fail":false,"target_va":5378649600,"va":5380830936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a62c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2003821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e8936ce1ff","size":5,"soft_fail":false,"target_va":5378826944,"va":5380830760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8ff10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e85d080000","size":5,"soft_fail":false,"target_va":5380833040,"va":5380830894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35520294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e826ff1d02","size":5,"soft_fail":false,"target_va":5416350944,"va":5380830645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36226508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e8ccc52802","size":5,"soft_fail":false,"target_va":5417057056,"va":5380830543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35790059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830448,"raw_hex":"e8eb1c2202","size":5,"soft_fail":false,"target_va":5416620769,"va":5380830705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":575,"record_offset":88408,"parent_record_offset":87784,"ancestor_record_offset":87784,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":575,"record_offset":87964,"parent_record_offset":87784,"ancestor_record_offset":87784,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":575,"record_offset":88152,"parent_record_offset":87784,"ancestor_record_offset":87784,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":88492,"parent_record_offset":87784,"ancestor_record_offset":87784,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":88720,"parent_record_offset":87784,"ancestor_record_offset":87784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88056,"parent_record_offset":88032,"ancestor_record_offset":87784,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88080,"parent_record_offset":88056,"ancestor_record_offset":87784,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88792,"parent_record_offset":88768,"ancestor_record_offset":87784,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88816,"parent_record_offset":88792,"ancestor_record_offset":87784,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88104,"parent_record_offset":88080,"ancestor_record_offset":87784,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88840,"parent_record_offset":88816,"ancestor_record_offset":87784,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88032,"parent_record_offset":87996,"ancestor_record_offset":87784,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":88768,"parent_record_offset":88744,"ancestor_record_offset":87784,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":575,"record_offset":87996,"parent_record_offset":87964,"ancestor_record_offset":87784,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":575,"record_offset":88188,"parent_record_offset":88152,"ancestor_record_offset":87784,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":575,"record_offset":88744,"parent_record_offset":88720,"ancestor_record_offset":87784,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":88596,"parent_record_offset":88564,"ancestor_record_offset":87784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":88644,"parent_record_offset":88620,"ancestor_record_offset":87784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":88620,"parent_record_offset":88596,"ancestor_record_offset":87784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":88432,"parent_record_offset":88408,"ancestor_record_offset":87784,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":88564,"parent_record_offset":88520,"ancestor_record_offset":87784,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":88456,"parent_record_offset":88432,"ancestor_record_offset":87784,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":88520,"parent_record_offset":88492,"ancestor_record_offset":87784,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":88372,"parent_record_offset":88188,"ancestor_record_offset":87784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":88684,"parent_record_offset":88520,"ancestor_record_offset":87784,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=26
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7787
// 首条位置: {"module":520,"inlinee":30599}
// 编译实例 RVAs: 0xb8f6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5152393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830960,"raw_hex":"e87761b1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380831092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a7df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1997241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830960,"raw_hex":"e84786e1ff","size":5,"soft_fail":false,"target_va":5378833904,"va":5380831140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t294725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380830960,"raw_hex":"e8457f0400","size":5,"soft_fail":false,"target_va":5381125776,"va":5380831046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":520,"record_offset":67384,"parent_record_offset":67352,"ancestor_record_offset":67352,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":24408,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":520,"record_offset":67444,"parent_record_offset":67352,"ancestor_record_offset":67352,"inlinee":30601,"source_line":29,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":24616,"parent_record_offset":24592,"ancestor_record_offset":24136,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":24640,"parent_record_offset":24616,"ancestor_record_offset":24136,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":24136,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":24592,"parent_record_offset":24504,"ancestor_record_offset":24136,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":24532,"parent_record_offset":24504,"ancestor_record_offset":24136,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":24504,"parent_record_offset":24480,"ancestor_record_offset":24136,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":24480,"parent_record_offset":24456,"ancestor_record_offset":24136,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":520,"record_offset":67476,"parent_record_offset":67444,"ancestor_record_offset":67352,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":520,"record_offset":67556,"parent_record_offset":67532,"ancestor_record_offset":67352,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":520,"record_offset":67588,"parent_record_offset":67444,"ancestor_record_offset":67352,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":520,"record_offset":67532,"parent_record_offset":67444,"ancestor_record_offset":67352,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":520,"record_offset":67408,"parent_record_offset":67384,"ancestor_record_offset":67352,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":24560,"parent_record_offset":24532,"ancestor_record_offset":24136,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":24432,"parent_record_offset":24408,"ancestor_record_offset":24136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":520,"record_offset":67500,"parent_record_offset":67476,"ancestor_record_offset":67352,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00520.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::sequence_len
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":86572,"rva":10567152,"va":5379276272}
// 编译实例 RVAs: 0xa13df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=6
// 源码行: 72, 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":38,"record_offset":3388,"rva":1653472,"va":5370362592}
// 编译实例 RVAs: 0x193ae0, 0x193b80, 0x193e70, 0x1c6b820, 0x1c6bd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$f32$u20$as$u20$zmij..private..Sealed$GT$::write_to_zmij_buffer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12893503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362592,"raw_hex":"e83fbdc400","size":5,"soft_fail":false,"target_va":5383256144,"va":5370362636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140196760
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8ec160000","size":5,"soft_fail":false,"target_va":5370373984,"va":5370368111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140465eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2957886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362752,"raw_hex":"e83e222d00","size":5,"soft_fail":false,"target_va":5373320880,"va":5370362989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140469060
//     owner=unresolved_namespace; count=21
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2969868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e80c512d00","size":5,"soft_fail":false,"target_va":5373333600,"va":5370363727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=46
//     target_aliases=3 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25166404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398509600,"raw_hex":"e8bcfd7ffe","size":5,"soft_fail":false,"target_va":5373343328,"va":5398509727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb520
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7760999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8676c7600","size":5,"soft_fail":false,"target_va":5378127136,"va":5370366132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b79e0
//     owner=unresolved_namespace; count=21
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3884066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510880,"raw_hex":"e8debbc4ff","size":5,"soft_fail":false,"target_va":5394627040,"va":5398511101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bd810
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3858690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398509600,"raw_hex":"e8fe1ec5ff","size":5,"soft_fail":false,"target_va":5394651152,"va":5398509837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c6fde0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510880,"raw_hex":"e8cd2e0000","size":5,"soft_fail":false,"target_va":5398527456,"va":5398515470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=140
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17842975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398509600,"raw_hex":"e81f431001","size":5,"soft_fail":false,"target_va":5416353040,"va":5398510060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46694369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362592,"raw_hex":"e8e17fc802","size":5,"soft_fail":false,"target_va":5417057056,"va":5370362682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18111961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398509600,"raw_hex":"e8d95d1401","size":5,"soft_fail":false,"target_va":5416622208,"va":5398510242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t18050174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398509600,"raw_hex":"e87e6c1301","size":5,"soft_fail":false,"target_va":5416560208,"va":5398510029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::end<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t5139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e813140000","size":5,"soft_fail":false,"target_va":5370374688,"va":5370369544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,bool>
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t7827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8931e0000","size":5,"soft_fail":false,"target_va":5370373888,"va":5370366056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::color_id::ColorId>
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t8006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8461f0000","size":5,"soft_fail":false,"target_va":5370373792,"va":5370365781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::history::wall_compound_edit::WallHeight>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t8801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e861220000","size":5,"soft_fail":false,"target_va":5370374512,"va":5370365706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::history::wall_compound_edit::_::impl$0::serialize::__SerializeWith>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t5635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e803160000","size":5,"soft_fail":false,"target_va":5370374304,"va":5370368664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e817240000","size":5,"soft_fail":false,"target_va":5370374416,"va":5370365172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallMaterial>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t4739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e883120000","size":5,"soft_fail":false,"target_va":5370373680,"va":5370368936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallState>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t10575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e84f290000","size":5,"soft_fail":false,"target_va":5370374368,"va":5370363788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WallStyle>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8e1230000","size":5,"soft_fail":false,"target_va":5370374640,"va":5370365450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::walls::inner_wall_state::WoodBoardLayout>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e819250000","size":5,"soft_fail":false,"target_va":5370373840,"va":5370364338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::WallHeightDecoratorBehavior>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t7316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8941c0000","size":5,"soft_fail":false,"target_va":5370373728,"va":5370366407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<core::option::Option<country_core::resources::walls::inner_wall_state::WallState> > >
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t8397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8cd200000","size":5,"soft_fail":false,"target_va":5370374144,"va":5370365742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::resources::history::wall_compound_edit::OperationType> >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t10322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e852280000","size":5,"soft_fail":false,"target_va":5370374080,"va":5370363753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::systems::wall::valid_enclosure::IsEnclosed> >
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e85d250000","size":5,"soft_fail":false,"target_va":5370374464,"va":5370364894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::systems::wall::valid_enclosure::ShapeParameters> >
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t6418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e812190000","size":5,"soft_fail":false,"target_va":5370374256,"va":5370367833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,f32>
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e863240000","size":5,"soft_fail":false,"target_va":5370373936,"va":5370364616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,glam::f32::affine2::Affine2>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t6865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8d11a0000","size":5,"soft_fail":false,"target_va":5370374192,"va":5370367322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,glam::f32::vec2::Vec2>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t7275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e86b1c0000","size":5,"soft_fail":false,"target_va":5370374576,"va":5370367296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t9967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370363504,"raw_hex":"e8ef260000","size":5,"soft_fail":false,"target_va":5370374032,"va":5370364060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t18049985","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5398509600,"raw_hex":"e9c16b1301","size":5,"soft_fail":false,"target_va":5416560208,"va":5398510218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::WallHeightDecoratorBehavior::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs
//     evidence={"module":1833,"record_offset":63472,"parent_record_offset":46708,"ancestor_record_offset":46708,"inlinee":37253,"source_line":42,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=git_dependency_definition; count=42
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":10124,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":7411,"source_line":738,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_newtype_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":3676,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":7390,"source_line":225,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=git_dependency_definition; count=50
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":4832,"parent_record_offset":4540,"ancestor_record_offset":4540,"inlinee":7409,"source_line":383,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":8112,"parent_record_offset":4540,"ancestor_record_offset":4540,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=138
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":5208,"parent_record_offset":5176,"ancestor_record_offset":4540,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":3388,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":4384,"parent_record_offset":4356,"ancestor_record_offset":3388,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":4356,"parent_record_offset":4324,"ancestor_record_offset":3388,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=138
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4540,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=138
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4540,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":4324,"parent_record_offset":4292,"ancestor_record_offset":3388,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=142
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3388,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=138
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":5176,"parent_record_offset":5008,"ancestor_record_offset":4540,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=142
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3388,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":4472,"parent_record_offset":4224,"ancestor_record_offset":3388,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":4264,"parent_record_offset":4224,"ancestor_record_offset":3388,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=142
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":38,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3388,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::classify
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":38,"record_offset":3772,"parent_record_offset":3732,"ancestor_record_offset":3388,"inlinee":8143,"source_line":725,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":38,"record_offset":6088,"parent_record_offset":6056,"ancestor_record_offset":4540,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":38,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":3388,"inlinee":7456,"source_line":612,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":38,"record_offset":4436,"parent_record_offset":4224,"ancestor_record_offset":3388,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":38,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3388,"inlinee":8147,"source_line":793,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":38,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":4540,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs
//     evidence={"module":1833,"record_offset":63496,"parent_record_offset":63472,"ancestor_record_offset":46708,"inlinee":27826,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$51::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":38,"record_offset":3704,"parent_record_offset":3676,"ancestor_record_offset":3388,"inlinee":7136,"source_line":11,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=90
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":4868,"parent_record_offset":4832,"ancestor_record_offset":4540,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=44
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":6124,"parent_record_offset":5928,"ancestor_record_offset":4540,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":7148,"parent_record_offset":5928,"ancestor_record_offset":4540,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_f32
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":3800,"parent_record_offset":3732,"ancestor_record_offset":3388,"inlinee":8144,"source_line":1691,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_f32
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":3388,"inlinee":8142,"source_line":155,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=44
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":6604,"parent_record_offset":5928,"ancestor_record_offset":4540,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=52
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":5348,"parent_record_offset":4832,"ancestor_record_offset":4540,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=44
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":7116,"parent_record_offset":5928,"ancestor_record_offset":4540,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=142
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3388,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=142
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":3856,"parent_record_offset":3800,"ancestor_record_offset":3388,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] zmij::Buffer::format_finite
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"zmij","version":"1.0.23","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zmij-1.0.23/src/lib.rs
//     evidence={"module":38,"record_offset":3828,"parent_record_offset":3800,"ancestor_record_offset":3388,"inlinee":8146,"source_line":1764,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":92972,"rva":29815536,"va":5398524656}
// 编译实例 RVAs: 0x1c6f2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17860786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398524656,"raw_hex":"e8b2881001","size":5,"soft_fail":false,"target_va":5416386464,"va":5398525673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1833,"record_offset":93180,"parent_record_offset":92972,"ancestor_record_offset":92972,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1833,"record_offset":93208,"parent_record_offset":93180,"ancestor_record_offset":92972,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":93236,"parent_record_offset":93208,"ancestor_record_offset":92972,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":93264,"parent_record_offset":93236,"ancestor_record_offset":92972,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1572F
// 首条位置: {"module":1803,"inlinee":87855}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":479836,"parent_record_offset":479808,"ancestor_record_offset":479808,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":480604,"parent_record_offset":480580,"ancestor_record_offset":479808,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":480628,"parent_record_offset":480604,"ancestor_record_offset":479808,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":480652,"parent_record_offset":480628,"ancestor_record_offset":479808,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":480580,"parent_record_offset":480500,"ancestor_record_offset":479808,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":480540,"parent_record_offset":480500,"ancestor_record_offset":479808,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":480500,"parent_record_offset":480464,"ancestor_record_offset":479808,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":480464,"parent_record_offset":480428,"ancestor_record_offset":479808,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":480024,"parent_record_offset":479996,"ancestor_record_offset":479808,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":480356,"parent_record_offset":479892,"ancestor_record_offset":479808,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":480052,"parent_record_offset":480024,"ancestor_record_offset":479808,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":480088,"parent_record_offset":480052,"ancestor_record_offset":479808,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":479996,"parent_record_offset":479948,"ancestor_record_offset":479808,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":479948,"parent_record_offset":479892,"ancestor_record_offset":479808,"inlinee":87856,"source_line":345,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":479892,"parent_record_offset":479864,"ancestor_record_offset":479808,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":479864,"parent_record_offset":479836,"ancestor_record_offset":479808,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156F3
// 首条位置: {"module":1803,"inlinee":87795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":358200,"parent_record_offset":358172,"ancestor_record_offset":358172,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":358224,"parent_record_offset":358200,"ancestor_record_offset":358172,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":358248,"parent_record_offset":358224,"ancestor_record_offset":358172,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":358276,"parent_record_offset":358248,"ancestor_record_offset":358172,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15709
// 首条位置: {"module":1803,"inlinee":87817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":417688,"parent_record_offset":417664,"ancestor_record_offset":417664,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":417964,"parent_record_offset":417940,"ancestor_record_offset":417664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":417988,"parent_record_offset":417964,"ancestor_record_offset":417664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":418012,"parent_record_offset":417988,"ancestor_record_offset":417664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":417940,"parent_record_offset":417884,"ancestor_record_offset":417664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":417912,"parent_record_offset":417884,"ancestor_record_offset":417664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":417884,"parent_record_offset":417860,"ancestor_record_offset":417664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":417860,"parent_record_offset":417836,"ancestor_record_offset":417664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":417788,"parent_record_offset":417760,"ancestor_record_offset":417664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":417760,"parent_record_offset":417736,"ancestor_record_offset":417664,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":417712,"parent_record_offset":417688,"ancestor_record_offset":417664,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":417736,"parent_record_offset":417712,"ancestor_record_offset":417664,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15755
// 首条位置: {"module":1804,"inlinee":87893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":51148,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":50892,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":52304,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":52724,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":50836,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":50928,"parent_record_offset":50772,"ancestor_record_offset":50772,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":51044,"parent_record_offset":51020,"ancestor_record_offset":50772,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":51020,"parent_record_offset":50984,"ancestor_record_offset":50772,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":51356,"parent_record_offset":51332,"ancestor_record_offset":50772,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":51404,"parent_record_offset":51380,"ancestor_record_offset":50772,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":52376,"parent_record_offset":52352,"ancestor_record_offset":50772,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":52492,"parent_record_offset":52468,"ancestor_record_offset":50772,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":52352,"parent_record_offset":52328,"ancestor_record_offset":50772,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":52400,"parent_record_offset":52376,"ancestor_record_offset":50772,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":50860,"parent_record_offset":50836,"ancestor_record_offset":50772,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":50956,"parent_record_offset":50928,"ancestor_record_offset":50772,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":51104,"parent_record_offset":51080,"ancestor_record_offset":50772,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":51080,"parent_record_offset":50984,"ancestor_record_offset":50772,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":51428,"parent_record_offset":51404,"ancestor_record_offset":50772,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":51692,"parent_record_offset":51668,"ancestor_record_offset":50772,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":50984,"parent_record_offset":50956,"ancestor_record_offset":50772,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":51380,"parent_record_offset":51356,"ancestor_record_offset":50772,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15779
// 首条位置: {"module":1804,"inlinee":87929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":192596,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":192344,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":193436,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":193800,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":192288,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":192376,"parent_record_offset":192224,"ancestor_record_offset":192224,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":192492,"parent_record_offset":192468,"ancestor_record_offset":192224,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":192468,"parent_record_offset":192432,"ancestor_record_offset":192224,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":192804,"parent_record_offset":192780,"ancestor_record_offset":192224,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":192852,"parent_record_offset":192828,"ancestor_record_offset":192224,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":193568,"parent_record_offset":193544,"ancestor_record_offset":192224,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":193544,"parent_record_offset":193520,"ancestor_record_offset":192224,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":193592,"parent_record_offset":193568,"ancestor_record_offset":192224,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":192312,"parent_record_offset":192288,"ancestor_record_offset":192224,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":192404,"parent_record_offset":192376,"ancestor_record_offset":192224,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":192552,"parent_record_offset":192528,"ancestor_record_offset":192224,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":192528,"parent_record_offset":192432,"ancestor_record_offset":192224,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":192876,"parent_record_offset":192852,"ancestor_record_offset":192224,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":192432,"parent_record_offset":192404,"ancestor_record_offset":192224,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":192828,"parent_record_offset":192804,"ancestor_record_offset":192224,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1578B
// 首条位置: {"module":1804,"inlinee":87947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":270392,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":270140,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":271232,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":271596,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":270084,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":270172,"parent_record_offset":270020,"ancestor_record_offset":270020,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":270288,"parent_record_offset":270264,"ancestor_record_offset":270020,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":270264,"parent_record_offset":270228,"ancestor_record_offset":270020,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":270600,"parent_record_offset":270576,"ancestor_record_offset":270020,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":270648,"parent_record_offset":270624,"ancestor_record_offset":270020,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":271364,"parent_record_offset":271340,"ancestor_record_offset":270020,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":271340,"parent_record_offset":271316,"ancestor_record_offset":270020,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":271388,"parent_record_offset":271364,"ancestor_record_offset":270020,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":270108,"parent_record_offset":270084,"ancestor_record_offset":270020,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":270200,"parent_record_offset":270172,"ancestor_record_offset":270020,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":270348,"parent_record_offset":270324,"ancestor_record_offset":270020,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":270324,"parent_record_offset":270228,"ancestor_record_offset":270020,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":270672,"parent_record_offset":270648,"ancestor_record_offset":270020,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":270228,"parent_record_offset":270200,"ancestor_record_offset":270020,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":270624,"parent_record_offset":270600,"ancestor_record_offset":270020,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156F2
// 首条位置: {"module":1803,"inlinee":87794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":358036,"parent_record_offset":358004,"ancestor_record_offset":358004,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":358656,"parent_record_offset":358632,"ancestor_record_offset":358004,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":358680,"parent_record_offset":358656,"ancestor_record_offset":358004,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":358704,"parent_record_offset":358680,"ancestor_record_offset":358004,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":358632,"parent_record_offset":358560,"ancestor_record_offset":358004,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":358596,"parent_record_offset":358560,"ancestor_record_offset":358004,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":358560,"parent_record_offset":358528,"ancestor_record_offset":358004,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":358528,"parent_record_offset":358496,"ancestor_record_offset":358004,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":358224,"parent_record_offset":358200,"ancestor_record_offset":358004,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":358432,"parent_record_offset":358132,"ancestor_record_offset":358004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":358248,"parent_record_offset":358224,"ancestor_record_offset":358004,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":358276,"parent_record_offset":358248,"ancestor_record_offset":358004,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":358200,"parent_record_offset":358172,"ancestor_record_offset":358004,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":358172,"parent_record_offset":358132,"ancestor_record_offset":358004,"inlinee":87795,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":358132,"parent_record_offset":358100,"ancestor_record_offset":358004,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":358068,"parent_record_offset":358036,"ancestor_record_offset":358004,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":358100,"parent_record_offset":358068,"ancestor_record_offset":358004,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1577D
// 首条位置: {"module":1804,"inlinee":87933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":209928,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":209672,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":210800,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":211320,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":209616,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":209708,"parent_record_offset":209552,"ancestor_record_offset":209552,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":209824,"parent_record_offset":209800,"ancestor_record_offset":209552,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":209800,"parent_record_offset":209764,"ancestor_record_offset":209552,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":210136,"parent_record_offset":210112,"ancestor_record_offset":209552,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":210184,"parent_record_offset":210160,"ancestor_record_offset":209552,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":210872,"parent_record_offset":210848,"ancestor_record_offset":209552,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":210988,"parent_record_offset":210964,"ancestor_record_offset":209552,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":210848,"parent_record_offset":210824,"ancestor_record_offset":209552,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":210896,"parent_record_offset":210872,"ancestor_record_offset":209552,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":209640,"parent_record_offset":209616,"ancestor_record_offset":209552,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":209736,"parent_record_offset":209708,"ancestor_record_offset":209552,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":209884,"parent_record_offset":209860,"ancestor_record_offset":209552,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":209860,"parent_record_offset":209764,"ancestor_record_offset":209552,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":210208,"parent_record_offset":210184,"ancestor_record_offset":209552,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":210440,"parent_record_offset":210416,"ancestor_record_offset":209552,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":209764,"parent_record_offset":209736,"ancestor_record_offset":209552,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":210160,"parent_record_offset":210136,"ancestor_record_offset":209552,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15749
// 首条位置: {"module":1804,"inlinee":87881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":16860,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":16604,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":17732,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":18252,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":16548,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":16640,"parent_record_offset":16484,"ancestor_record_offset":16484,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":16484,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":16732,"parent_record_offset":16696,"ancestor_record_offset":16484,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":16484,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":16484,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":16484,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":17920,"parent_record_offset":17896,"ancestor_record_offset":16484,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":17780,"parent_record_offset":17756,"ancestor_record_offset":16484,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":17828,"parent_record_offset":17804,"ancestor_record_offset":16484,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":16484,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":16668,"parent_record_offset":16640,"ancestor_record_offset":16484,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":16484,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":16792,"parent_record_offset":16696,"ancestor_record_offset":16484,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":16484,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":17372,"parent_record_offset":17348,"ancestor_record_offset":16484,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":16484,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":16484,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15750
// 首条位置: {"module":1804,"inlinee":87888}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":38172,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":37916,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":39012,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":39532,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37860,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37952,"parent_record_offset":37796,"ancestor_record_offset":37796,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":38068,"parent_record_offset":38044,"ancestor_record_offset":37796,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":38044,"parent_record_offset":38008,"ancestor_record_offset":37796,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":38380,"parent_record_offset":38356,"ancestor_record_offset":37796,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":38428,"parent_record_offset":38404,"ancestor_record_offset":37796,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":39036,"parent_record_offset":39012,"ancestor_record_offset":37796,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":39084,"parent_record_offset":39060,"ancestor_record_offset":37796,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":39060,"parent_record_offset":39036,"ancestor_record_offset":37796,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":39200,"parent_record_offset":39176,"ancestor_record_offset":37796,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":39108,"parent_record_offset":39084,"ancestor_record_offset":37796,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37884,"parent_record_offset":37860,"ancestor_record_offset":37796,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37980,"parent_record_offset":37952,"ancestor_record_offset":37796,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":37796,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":38104,"parent_record_offset":38008,"ancestor_record_offset":37796,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":38452,"parent_record_offset":38428,"ancestor_record_offset":37796,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":38008,"parent_record_offset":37980,"ancestor_record_offset":37796,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":38404,"parent_record_offset":38380,"ancestor_record_offset":37796,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156BF
// 首条位置: {"module":1803,"inlinee":87743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":253200,"parent_record_offset":253168,"ancestor_record_offset":253168,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":253224,"parent_record_offset":253200,"ancestor_record_offset":253168,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":253248,"parent_record_offset":253224,"ancestor_record_offset":253168,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":253276,"parent_record_offset":253248,"ancestor_record_offset":253168,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156BD
// 首条位置: {"module":1803,"inlinee":87741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":253024,"parent_record_offset":252992,"ancestor_record_offset":252992,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":253792,"parent_record_offset":253768,"ancestor_record_offset":252992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":253816,"parent_record_offset":253792,"ancestor_record_offset":252992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":253840,"parent_record_offset":253816,"ancestor_record_offset":252992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":253768,"parent_record_offset":253688,"ancestor_record_offset":252992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":253728,"parent_record_offset":253688,"ancestor_record_offset":252992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":253688,"parent_record_offset":253652,"ancestor_record_offset":252992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":253652,"parent_record_offset":253616,"ancestor_record_offset":252992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":253224,"parent_record_offset":253200,"ancestor_record_offset":252992,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":253544,"parent_record_offset":253120,"ancestor_record_offset":252992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":253248,"parent_record_offset":253224,"ancestor_record_offset":252992,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":253276,"parent_record_offset":253248,"ancestor_record_offset":252992,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":253200,"parent_record_offset":253168,"ancestor_record_offset":252992,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":253168,"parent_record_offset":253120,"ancestor_record_offset":252992,"inlinee":87743,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":253120,"parent_record_offset":253088,"ancestor_record_offset":252992,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":253056,"parent_record_offset":253024,"ancestor_record_offset":252992,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":253088,"parent_record_offset":253056,"ancestor_record_offset":252992,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15784
// 首条位置: {"module":1804,"inlinee":87940}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":233708,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":233452,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":234548,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":235068,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":233396,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":233488,"parent_record_offset":233332,"ancestor_record_offset":233332,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":233604,"parent_record_offset":233580,"ancestor_record_offset":233332,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":233580,"parent_record_offset":233544,"ancestor_record_offset":233332,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":233916,"parent_record_offset":233892,"ancestor_record_offset":233332,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":233964,"parent_record_offset":233940,"ancestor_record_offset":233332,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":234572,"parent_record_offset":234548,"ancestor_record_offset":233332,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":234620,"parent_record_offset":234596,"ancestor_record_offset":233332,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":234596,"parent_record_offset":234572,"ancestor_record_offset":233332,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":234736,"parent_record_offset":234712,"ancestor_record_offset":233332,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":234644,"parent_record_offset":234620,"ancestor_record_offset":233332,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":233420,"parent_record_offset":233396,"ancestor_record_offset":233332,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":233516,"parent_record_offset":233488,"ancestor_record_offset":233332,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":233664,"parent_record_offset":233640,"ancestor_record_offset":233332,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":233640,"parent_record_offset":233544,"ancestor_record_offset":233332,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":233988,"parent_record_offset":233964,"ancestor_record_offset":233332,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":233544,"parent_record_offset":233516,"ancestor_record_offset":233332,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":233940,"parent_record_offset":233916,"ancestor_record_offset":233332,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1574F
// 首条位置: {"module":1804,"inlinee":87887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":36648,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":77096,"parent_record_offset":75320,"ancestor_record_offset":75320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":36140,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":78544,"parent_record_offset":75320,"ancestor_record_offset":75320,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":36932,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37304,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":36084,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":36176,"parent_record_offset":36052,"ancestor_record_offset":36052,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":77360,"parent_record_offset":77336,"ancestor_record_offset":75320,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":77384,"parent_record_offset":77360,"ancestor_record_offset":75320,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":77408,"parent_record_offset":77384,"ancestor_record_offset":75320,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":77336,"parent_record_offset":77236,"ancestor_record_offset":75320,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":77268,"parent_record_offset":77236,"ancestor_record_offset":75320,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":77236,"parent_record_offset":77208,"ancestor_record_offset":75320,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":77208,"parent_record_offset":77180,"ancestor_record_offset":75320,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":36292,"parent_record_offset":36268,"ancestor_record_offset":36052,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":77304,"parent_record_offset":77268,"ancestor_record_offset":75320,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":36268,"parent_record_offset":36232,"ancestor_record_offset":36052,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":77124,"parent_record_offset":77096,"ancestor_record_offset":75320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":76840,"parent_record_offset":76816,"ancestor_record_offset":75320,"inlinee":87846,"source_line":72,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":75872,"parent_record_offset":75848,"ancestor_record_offset":75320,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":75920,"parent_record_offset":75896,"ancestor_record_offset":75320,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":76204,"parent_record_offset":76180,"ancestor_record_offset":75320,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":37004,"parent_record_offset":36980,"ancestor_record_offset":36052,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":76864,"parent_record_offset":76840,"ancestor_record_offset":75320,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":36980,"parent_record_offset":36956,"ancestor_record_offset":36052,"inlinee":83141,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":77004,"parent_record_offset":76980,"ancestor_record_offset":75320,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":78452,"parent_record_offset":78428,"ancestor_record_offset":75320,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":37028,"parent_record_offset":37004,"ancestor_record_offset":36052,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":36108,"parent_record_offset":36084,"ancestor_record_offset":36052,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":36204,"parent_record_offset":36176,"ancestor_record_offset":36052,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":36352,"parent_record_offset":36328,"ancestor_record_offset":36052,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":36328,"parent_record_offset":36232,"ancestor_record_offset":36052,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":75944,"parent_record_offset":75920,"ancestor_record_offset":75320,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":36232,"parent_record_offset":36204,"ancestor_record_offset":36052,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":75896,"parent_record_offset":75872,"ancestor_record_offset":75320,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1804,"record_offset":36572,"parent_record_offset":36548,"ancestor_record_offset":36052,"inlinee":82380,"source_line":67,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15765
// 首条位置: {"module":1804,"inlinee":87909}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":111496,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":111240,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":112336,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":112856,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":111184,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":111276,"parent_record_offset":111120,"ancestor_record_offset":111120,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":111392,"parent_record_offset":111368,"ancestor_record_offset":111120,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":111368,"parent_record_offset":111332,"ancestor_record_offset":111120,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":111704,"parent_record_offset":111680,"ancestor_record_offset":111120,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":111752,"parent_record_offset":111728,"ancestor_record_offset":111120,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":112360,"parent_record_offset":112336,"ancestor_record_offset":111120,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":112408,"parent_record_offset":112384,"ancestor_record_offset":111120,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":112384,"parent_record_offset":112360,"ancestor_record_offset":111120,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":112524,"parent_record_offset":112500,"ancestor_record_offset":111120,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":112432,"parent_record_offset":112408,"ancestor_record_offset":111120,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":111208,"parent_record_offset":111184,"ancestor_record_offset":111120,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":111304,"parent_record_offset":111276,"ancestor_record_offset":111120,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":111452,"parent_record_offset":111428,"ancestor_record_offset":111120,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":111428,"parent_record_offset":111332,"ancestor_record_offset":111120,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":111776,"parent_record_offset":111752,"ancestor_record_offset":111120,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":111332,"parent_record_offset":111304,"ancestor_record_offset":111120,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":111728,"parent_record_offset":111704,"ancestor_record_offset":111120,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1574A
// 首条位置: {"module":1804,"inlinee":87882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":19236,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":18980,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":20108,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":20628,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":18924,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":19016,"parent_record_offset":18860,"ancestor_record_offset":18860,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":18860,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":19108,"parent_record_offset":19072,"ancestor_record_offset":18860,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":19444,"parent_record_offset":19420,"ancestor_record_offset":18860,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":19492,"parent_record_offset":19468,"ancestor_record_offset":18860,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":20180,"parent_record_offset":20156,"ancestor_record_offset":18860,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":18860,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":20156,"parent_record_offset":20132,"ancestor_record_offset":18860,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":18860,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":18860,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":19044,"parent_record_offset":19016,"ancestor_record_offset":18860,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":19192,"parent_record_offset":19168,"ancestor_record_offset":18860,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":19168,"parent_record_offset":19072,"ancestor_record_offset":18860,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":19516,"parent_record_offset":19492,"ancestor_record_offset":18860,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":18860,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":19072,"parent_record_offset":19044,"ancestor_record_offset":18860,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":19468,"parent_record_offset":19444,"ancestor_record_offset":18860,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1575D
// 首条位置: {"module":1804,"inlinee":87901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":79788,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":79532,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":80828,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":81500,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":79476,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":79568,"parent_record_offset":79444,"ancestor_record_offset":79444,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":79684,"parent_record_offset":79660,"ancestor_record_offset":79444,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":79660,"parent_record_offset":79624,"ancestor_record_offset":79444,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":80400,"parent_record_offset":80348,"ancestor_record_offset":79444,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":79996,"parent_record_offset":79972,"ancestor_record_offset":79444,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":80044,"parent_record_offset":80020,"ancestor_record_offset":79444,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":80300,"parent_record_offset":80276,"ancestor_record_offset":79444,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":80348,"parent_record_offset":80324,"ancestor_record_offset":79444,"inlinee":82524,"source_line":184,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":80900,"parent_record_offset":80876,"ancestor_record_offset":79444,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":80876,"parent_record_offset":80852,"ancestor_record_offset":79444,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":80924,"parent_record_offset":80900,"ancestor_record_offset":79444,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":79500,"parent_record_offset":79476,"ancestor_record_offset":79444,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":79596,"parent_record_offset":79568,"ancestor_record_offset":79444,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":79744,"parent_record_offset":79720,"ancestor_record_offset":79444,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":79720,"parent_record_offset":79624,"ancestor_record_offset":79444,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":80068,"parent_record_offset":80044,"ancestor_record_offset":79444,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":80372,"parent_record_offset":80348,"ancestor_record_offset":79444,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":79624,"parent_record_offset":79596,"ancestor_record_offset":79444,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":80020,"parent_record_offset":79996,"ancestor_record_offset":79444,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":97936,"rva":29817120,"va":5398526240}
// 编译实例 RVAs: 0x1c6f920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1833,"record_offset":98164,"parent_record_offset":97936,"ancestor_record_offset":97936,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1833,"record_offset":98192,"parent_record_offset":98164,"ancestor_record_offset":97936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":98220,"parent_record_offset":98192,"ancestor_record_offset":97936,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":98248,"parent_record_offset":98220,"ancestor_record_offset":97936,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1576F
// 首条位置: {"module":1804,"inlinee":87919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":149080,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":148824,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":149920,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":150440,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":148768,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":148860,"parent_record_offset":148704,"ancestor_record_offset":148704,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":148976,"parent_record_offset":148952,"ancestor_record_offset":148704,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":148952,"parent_record_offset":148916,"ancestor_record_offset":148704,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":149288,"parent_record_offset":149264,"ancestor_record_offset":148704,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":149336,"parent_record_offset":149312,"ancestor_record_offset":148704,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":149944,"parent_record_offset":149920,"ancestor_record_offset":148704,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":149992,"parent_record_offset":149968,"ancestor_record_offset":148704,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":149968,"parent_record_offset":149944,"ancestor_record_offset":148704,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":150108,"parent_record_offset":150084,"ancestor_record_offset":148704,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":150016,"parent_record_offset":149992,"ancestor_record_offset":148704,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":148792,"parent_record_offset":148768,"ancestor_record_offset":148704,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":148888,"parent_record_offset":148860,"ancestor_record_offset":148704,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":149036,"parent_record_offset":149012,"ancestor_record_offset":148704,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":149012,"parent_record_offset":148916,"ancestor_record_offset":148704,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":149360,"parent_record_offset":149336,"ancestor_record_offset":148704,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":148916,"parent_record_offset":148888,"ancestor_record_offset":148704,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":149312,"parent_record_offset":149288,"ancestor_record_offset":148704,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$44::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1576E
// 首条位置: {"module":1804,"inlinee":87918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":146736,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":146480,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":147576,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":148096,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":146424,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":146516,"parent_record_offset":146360,"ancestor_record_offset":146360,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":146632,"parent_record_offset":146608,"ancestor_record_offset":146360,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":146608,"parent_record_offset":146572,"ancestor_record_offset":146360,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":146944,"parent_record_offset":146920,"ancestor_record_offset":146360,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":146992,"parent_record_offset":146968,"ancestor_record_offset":146360,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":147600,"parent_record_offset":147576,"ancestor_record_offset":146360,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":147648,"parent_record_offset":147624,"ancestor_record_offset":146360,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_enum
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":147624,"parent_record_offset":147600,"ancestor_record_offset":146360,"inlinee":7799,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":147764,"parent_record_offset":147740,"ancestor_record_offset":146360,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":147672,"parent_record_offset":147648,"ancestor_record_offset":146360,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":146448,"parent_record_offset":146424,"ancestor_record_offset":146360,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":146544,"parent_record_offset":146516,"ancestor_record_offset":146360,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":146692,"parent_record_offset":146668,"ancestor_record_offset":146360,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":146668,"parent_record_offset":146572,"ancestor_record_offset":146360,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":147016,"parent_record_offset":146992,"ancestor_record_offset":146360,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":146572,"parent_record_offset":146544,"ancestor_record_offset":146360,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":146968,"parent_record_offset":146944,"ancestor_record_offset":146360,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$45::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156C3
// 首条位置: {"module":1803,"inlinee":87747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":259164,"parent_record_offset":259128,"ancestor_record_offset":259128,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":259188,"parent_record_offset":259164,"ancestor_record_offset":259128,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":259212,"parent_record_offset":259188,"ancestor_record_offset":259128,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":259240,"parent_record_offset":259212,"ancestor_record_offset":259128,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$46::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156C1
// 首条位置: {"module":1803,"inlinee":87745}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":258972,"parent_record_offset":258940,"ancestor_record_offset":258940,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":259756,"parent_record_offset":259732,"ancestor_record_offset":258940,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":259780,"parent_record_offset":259756,"ancestor_record_offset":258940,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":259804,"parent_record_offset":259780,"ancestor_record_offset":258940,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":259732,"parent_record_offset":259652,"ancestor_record_offset":258940,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":259692,"parent_record_offset":259652,"ancestor_record_offset":258940,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":259652,"parent_record_offset":259616,"ancestor_record_offset":258940,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":259616,"parent_record_offset":259580,"ancestor_record_offset":258940,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":259188,"parent_record_offset":259164,"ancestor_record_offset":258940,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":259508,"parent_record_offset":259080,"ancestor_record_offset":258940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":259212,"parent_record_offset":259188,"ancestor_record_offset":258940,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":259240,"parent_record_offset":259212,"ancestor_record_offset":258940,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":259164,"parent_record_offset":259128,"ancestor_record_offset":258940,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$45::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":259128,"parent_record_offset":259080,"ancestor_record_offset":258940,"inlinee":87747,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":259080,"parent_record_offset":259044,"ancestor_record_offset":258940,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":259008,"parent_record_offset":258972,"ancestor_record_offset":258940,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":259044,"parent_record_offset":259008,"ancestor_record_offset":258940,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$47::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15783
// 首条位置: {"module":1804,"inlinee":87939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":231308,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":231052,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":232204,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":232724,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":230996,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":231088,"parent_record_offset":230932,"ancestor_record_offset":230932,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":231204,"parent_record_offset":231180,"ancestor_record_offset":230932,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":231180,"parent_record_offset":231144,"ancestor_record_offset":230932,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":231516,"parent_record_offset":231492,"ancestor_record_offset":230932,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":231564,"parent_record_offset":231540,"ancestor_record_offset":230932,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":232276,"parent_record_offset":232252,"ancestor_record_offset":230932,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":232252,"parent_record_offset":232228,"ancestor_record_offset":230932,"inlinee":7736,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":232392,"parent_record_offset":232368,"ancestor_record_offset":230932,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":232300,"parent_record_offset":232276,"ancestor_record_offset":230932,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":231020,"parent_record_offset":230996,"ancestor_record_offset":230932,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":231116,"parent_record_offset":231088,"ancestor_record_offset":230932,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":231264,"parent_record_offset":231240,"ancestor_record_offset":230932,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":231240,"parent_record_offset":231144,"ancestor_record_offset":230932,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":231848,"parent_record_offset":231824,"ancestor_record_offset":230932,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":231588,"parent_record_offset":231564,"ancestor_record_offset":230932,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":231144,"parent_record_offset":231116,"ancestor_record_offset":230932,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":231540,"parent_record_offset":231516,"ancestor_record_offset":230932,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1570B
// 首条位置: {"module":1803,"inlinee":87819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":420308,"parent_record_offset":420284,"ancestor_record_offset":420284,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":420584,"parent_record_offset":420560,"ancestor_record_offset":420284,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":420608,"parent_record_offset":420584,"ancestor_record_offset":420284,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":420632,"parent_record_offset":420608,"ancestor_record_offset":420284,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":420560,"parent_record_offset":420504,"ancestor_record_offset":420284,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":420532,"parent_record_offset":420504,"ancestor_record_offset":420284,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":420504,"parent_record_offset":420480,"ancestor_record_offset":420284,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":420480,"parent_record_offset":420456,"ancestor_record_offset":420284,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":420408,"parent_record_offset":420380,"ancestor_record_offset":420284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":420380,"parent_record_offset":420356,"ancestor_record_offset":420284,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":420332,"parent_record_offset":420308,"ancestor_record_offset":420284,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":420356,"parent_record_offset":420332,"ancestor_record_offset":420284,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$50::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1577F
// 首条位置: {"module":1804,"inlinee":87935}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":214060,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":215264,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":213804,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":215096,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":214956,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":216736,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":213748,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":213840,"parent_record_offset":213716,"ancestor_record_offset":213716,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":215500,"parent_record_offset":215476,"ancestor_record_offset":213716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":215524,"parent_record_offset":215500,"ancestor_record_offset":213716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":215548,"parent_record_offset":215524,"ancestor_record_offset":213716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":215476,"parent_record_offset":215376,"ancestor_record_offset":213716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":215408,"parent_record_offset":215376,"ancestor_record_offset":213716,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":215376,"parent_record_offset":215348,"ancestor_record_offset":213716,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":215348,"parent_record_offset":215320,"ancestor_record_offset":213716,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":213956,"parent_record_offset":213932,"ancestor_record_offset":213716,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":215444,"parent_record_offset":215408,"ancestor_record_offset":213716,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":213932,"parent_record_offset":213896,"ancestor_record_offset":213716,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":215292,"parent_record_offset":215264,"ancestor_record_offset":213716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":214268,"parent_record_offset":214244,"ancestor_record_offset":213716,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":214316,"parent_record_offset":214292,"ancestor_record_offset":213716,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":215028,"parent_record_offset":215004,"ancestor_record_offset":213716,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":215172,"parent_record_offset":215148,"ancestor_record_offset":213716,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":215004,"parent_record_offset":214980,"ancestor_record_offset":213716,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":215052,"parent_record_offset":215028,"ancestor_record_offset":213716,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":213772,"parent_record_offset":213748,"ancestor_record_offset":213716,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":213868,"parent_record_offset":213840,"ancestor_record_offset":213716,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":214016,"parent_record_offset":213992,"ancestor_record_offset":213716,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":213992,"parent_record_offset":213896,"ancestor_record_offset":213716,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":214340,"parent_record_offset":214316,"ancestor_record_offset":213716,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":213896,"parent_record_offset":213868,"ancestor_record_offset":213716,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":214292,"parent_record_offset":214268,"ancestor_record_offset":213716,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1804,"record_offset":214600,"parent_record_offset":214576,"ancestor_record_offset":213716,"inlinee":82446,"source_line":61,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$51::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156D5
// 首条位置: {"module":1803,"inlinee":87765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":287560,"parent_record_offset":287516,"ancestor_record_offset":287516,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":287588,"parent_record_offset":287560,"ancestor_record_offset":287516,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":287616,"parent_record_offset":287588,"ancestor_record_offset":287516,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":287648,"parent_record_offset":287616,"ancestor_record_offset":287516,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$52::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156D3
// 首条位置: {"module":1803,"inlinee":87763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":287356,"parent_record_offset":287324,"ancestor_record_offset":287324,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":288300,"parent_record_offset":288276,"ancestor_record_offset":287324,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":288324,"parent_record_offset":288300,"ancestor_record_offset":287324,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":288348,"parent_record_offset":288324,"ancestor_record_offset":287324,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":288276,"parent_record_offset":288188,"ancestor_record_offset":287324,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":288232,"parent_record_offset":288188,"ancestor_record_offset":287324,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":288188,"parent_record_offset":288148,"ancestor_record_offset":287324,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":288148,"parent_record_offset":288108,"ancestor_record_offset":287324,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":287588,"parent_record_offset":287560,"ancestor_record_offset":287324,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":288028,"parent_record_offset":287464,"ancestor_record_offset":287324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":287616,"parent_record_offset":287588,"ancestor_record_offset":287324,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":287648,"parent_record_offset":287616,"ancestor_record_offset":287324,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":287560,"parent_record_offset":287516,"ancestor_record_offset":287324,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$51::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":287516,"parent_record_offset":287464,"ancestor_record_offset":287324,"inlinee":87765,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":287464,"parent_record_offset":287428,"ancestor_record_offset":287324,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":287392,"parent_record_offset":287356,"ancestor_record_offset":287324,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":287428,"parent_record_offset":287392,"ancestor_record_offset":287324,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$53::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1578F
// 首条位置: {"module":1804,"inlinee":87951}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":284160,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":285656,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":283908,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":285488,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":285348,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":287268,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":283852,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":283940,"parent_record_offset":283820,"ancestor_record_offset":283820,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":285864,"parent_record_offset":285840,"ancestor_record_offset":283820,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":285888,"parent_record_offset":285864,"ancestor_record_offset":283820,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":285912,"parent_record_offset":285888,"ancestor_record_offset":283820,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":285840,"parent_record_offset":285752,"ancestor_record_offset":283820,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":285780,"parent_record_offset":285752,"ancestor_record_offset":283820,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":285752,"parent_record_offset":285728,"ancestor_record_offset":283820,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":285728,"parent_record_offset":285704,"ancestor_record_offset":283820,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":284056,"parent_record_offset":284032,"ancestor_record_offset":283820,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":285808,"parent_record_offset":285780,"ancestor_record_offset":283820,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":284032,"parent_record_offset":283996,"ancestor_record_offset":283820,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":285680,"parent_record_offset":285656,"ancestor_record_offset":283820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":284368,"parent_record_offset":284344,"ancestor_record_offset":283820,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":284416,"parent_record_offset":284392,"ancestor_record_offset":283820,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1804,"record_offset":284704,"parent_record_offset":284680,"ancestor_record_offset":283820,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1804,"record_offset":284992,"parent_record_offset":284968,"ancestor_record_offset":283820,"inlinee":34893,"source_line":574,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":285420,"parent_record_offset":285396,"ancestor_record_offset":283820,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":285564,"parent_record_offset":285540,"ancestor_record_offset":283820,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":285396,"parent_record_offset":285372,"ancestor_record_offset":283820,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":286312,"parent_record_offset":286288,"ancestor_record_offset":283820,"inlinee":34837,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":285444,"parent_record_offset":285420,"ancestor_record_offset":283820,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":283876,"parent_record_offset":283852,"ancestor_record_offset":283820,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":283968,"parent_record_offset":283940,"ancestor_record_offset":283820,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":284116,"parent_record_offset":284092,"ancestor_record_offset":283820,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":284092,"parent_record_offset":283996,"ancestor_record_offset":283820,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":284440,"parent_record_offset":284416,"ancestor_record_offset":283820,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":283996,"parent_record_offset":283968,"ancestor_record_offset":283820,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":284392,"parent_record_offset":284368,"ancestor_record_offset":283820,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1804,"record_offset":285272,"parent_record_offset":285248,"ancestor_record_offset":283820,"inlinee":82446,"source_line":61,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$54::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156CD
// 首条位置: {"module":1803,"inlinee":87757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":272868,"parent_record_offset":272836,"ancestor_record_offset":272836,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":272892,"parent_record_offset":272868,"ancestor_record_offset":272836,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":272916,"parent_record_offset":272892,"ancestor_record_offset":272836,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":272944,"parent_record_offset":272916,"ancestor_record_offset":272836,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$55::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156CB
// 首条位置: {"module":1803,"inlinee":87755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":272696,"parent_record_offset":272664,"ancestor_record_offset":272664,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":273460,"parent_record_offset":273436,"ancestor_record_offset":272664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":273484,"parent_record_offset":273460,"ancestor_record_offset":272664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":273508,"parent_record_offset":273484,"ancestor_record_offset":272664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":273436,"parent_record_offset":273356,"ancestor_record_offset":272664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":273396,"parent_record_offset":273356,"ancestor_record_offset":272664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":273356,"parent_record_offset":273320,"ancestor_record_offset":272664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":273320,"parent_record_offset":273284,"ancestor_record_offset":272664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":272892,"parent_record_offset":272868,"ancestor_record_offset":272664,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":273212,"parent_record_offset":272792,"ancestor_record_offset":272664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":272916,"parent_record_offset":272892,"ancestor_record_offset":272664,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":272944,"parent_record_offset":272916,"ancestor_record_offset":272664,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":272868,"parent_record_offset":272836,"ancestor_record_offset":272664,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$54::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":272836,"parent_record_offset":272792,"ancestor_record_offset":272664,"inlinee":87757,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":272792,"parent_record_offset":272760,"ancestor_record_offset":272664,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":272728,"parent_record_offset":272696,"ancestor_record_offset":272664,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":272760,"parent_record_offset":272728,"ancestor_record_offset":272664,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15771
// 首条位置: {"module":1804,"inlinee":87921}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":154428,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":155480,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":154172,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":156728,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":155340,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":156756,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":156840,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":154116,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":154208,"parent_record_offset":154084,"ancestor_record_offset":154084,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":155716,"parent_record_offset":155692,"ancestor_record_offset":154084,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":155740,"parent_record_offset":155716,"ancestor_record_offset":154084,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":155764,"parent_record_offset":155740,"ancestor_record_offset":154084,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":155692,"parent_record_offset":155592,"ancestor_record_offset":154084,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":155624,"parent_record_offset":155592,"ancestor_record_offset":154084,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":155592,"parent_record_offset":155564,"ancestor_record_offset":154084,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":155564,"parent_record_offset":155536,"ancestor_record_offset":154084,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":154324,"parent_record_offset":154300,"ancestor_record_offset":154084,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":155660,"parent_record_offset":155624,"ancestor_record_offset":154084,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":154300,"parent_record_offset":154264,"ancestor_record_offset":154084,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":155508,"parent_record_offset":155480,"ancestor_record_offset":154084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":155000,"parent_record_offset":154976,"ancestor_record_offset":154084,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":155264,"parent_record_offset":155240,"ancestor_record_offset":154084,"inlinee":87925,"source_line":81,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":154976,"parent_record_offset":154952,"ancestor_record_offset":154084,"inlinee":87923,"source_line":81,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":154636,"parent_record_offset":154612,"ancestor_record_offset":154084,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":154684,"parent_record_offset":154660,"ancestor_record_offset":154084,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":155412,"parent_record_offset":155388,"ancestor_record_offset":154084,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":155388,"parent_record_offset":155364,"ancestor_record_offset":154084,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":155436,"parent_record_offset":155412,"ancestor_record_offset":154084,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":154140,"parent_record_offset":154116,"ancestor_record_offset":154084,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":154236,"parent_record_offset":154208,"ancestor_record_offset":154084,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":154384,"parent_record_offset":154360,"ancestor_record_offset":154084,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":154360,"parent_record_offset":154264,"ancestor_record_offset":154084,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":154708,"parent_record_offset":154684,"ancestor_record_offset":154084,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":154264,"parent_record_offset":154236,"ancestor_record_offset":154084,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":154660,"parent_record_offset":154636,"ancestor_record_offset":154084,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15775
// 首条位置: {"module":1804,"inlinee":87925}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":155288,"parent_record_offset":155264,"ancestor_record_offset":155264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15773
// 首条位置: {"module":1804,"inlinee":87923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":155000,"parent_record_offset":154976,"ancestor_record_offset":154976,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$57::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15715
// 首条位置: {"module":1803,"inlinee":87829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":431988,"parent_record_offset":431956,"ancestor_record_offset":431956,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":432012,"parent_record_offset":431988,"ancestor_record_offset":431956,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":432036,"parent_record_offset":432012,"ancestor_record_offset":431956,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":432064,"parent_record_offset":432036,"ancestor_record_offset":431956,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$58::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15713
// 首条位置: {"module":1803,"inlinee":87827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":431812,"parent_record_offset":431780,"ancestor_record_offset":431780,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":432580,"parent_record_offset":432556,"ancestor_record_offset":431780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":432604,"parent_record_offset":432580,"ancestor_record_offset":431780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":432628,"parent_record_offset":432604,"ancestor_record_offset":431780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":432556,"parent_record_offset":432476,"ancestor_record_offset":431780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":432516,"parent_record_offset":432476,"ancestor_record_offset":431780,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":432476,"parent_record_offset":432440,"ancestor_record_offset":431780,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":432440,"parent_record_offset":432404,"ancestor_record_offset":431780,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":432012,"parent_record_offset":431988,"ancestor_record_offset":431780,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":432332,"parent_record_offset":431908,"ancestor_record_offset":431780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":432036,"parent_record_offset":432012,"ancestor_record_offset":431780,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":432064,"parent_record_offset":432036,"ancestor_record_offset":431780,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":431988,"parent_record_offset":431956,"ancestor_record_offset":431780,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$57::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1803,"record_offset":431956,"parent_record_offset":431908,"ancestor_record_offset":431780,"inlinee":87829,"source_line":81,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":431908,"parent_record_offset":431876,"ancestor_record_offset":431780,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":431844,"parent_record_offset":431812,"ancestor_record_offset":431780,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":431876,"parent_record_offset":431844,"ancestor_record_offset":431780,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$59::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1577B
// 首条位置: {"module":1804,"inlinee":87931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":202032,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":201780,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":202900,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":203244,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":201724,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":201812,"parent_record_offset":201644,"ancestor_record_offset":201644,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":201928,"parent_record_offset":201904,"ancestor_record_offset":201644,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":201904,"parent_record_offset":201868,"ancestor_record_offset":201644,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":202240,"parent_record_offset":202216,"ancestor_record_offset":201644,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":202288,"parent_record_offset":202264,"ancestor_record_offset":201644,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":203036,"parent_record_offset":203012,"ancestor_record_offset":201644,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":203152,"parent_record_offset":203128,"ancestor_record_offset":201644,"inlinee":7736,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":203012,"parent_record_offset":202988,"ancestor_record_offset":201644,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":203060,"parent_record_offset":203036,"ancestor_record_offset":201644,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":201748,"parent_record_offset":201724,"ancestor_record_offset":201644,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":201840,"parent_record_offset":201812,"ancestor_record_offset":201644,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":201988,"parent_record_offset":201964,"ancestor_record_offset":201644,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":201964,"parent_record_offset":201868,"ancestor_record_offset":201644,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":202572,"parent_record_offset":202548,"ancestor_record_offset":201644,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":202312,"parent_record_offset":202288,"ancestor_record_offset":201644,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":201868,"parent_record_offset":201840,"ancestor_record_offset":201644,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":202264,"parent_record_offset":202240,"ancestor_record_offset":201644,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15786
// 首条位置: {"module":1804,"inlinee":87942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":240720,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":243156,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":240464,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":242848,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":242876,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":243876,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":240408,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":240500,"parent_record_offset":240368,"ancestor_record_offset":240368,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":243412,"parent_record_offset":243388,"ancestor_record_offset":240368,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":243436,"parent_record_offset":243412,"ancestor_record_offset":240368,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":243460,"parent_record_offset":243436,"ancestor_record_offset":240368,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":243388,"parent_record_offset":243300,"ancestor_record_offset":240368,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":243328,"parent_record_offset":243300,"ancestor_record_offset":240368,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":243300,"parent_record_offset":243276,"ancestor_record_offset":240368,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":243276,"parent_record_offset":243252,"ancestor_record_offset":240368,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":240616,"parent_record_offset":240592,"ancestor_record_offset":240368,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":243356,"parent_record_offset":243328,"ancestor_record_offset":240368,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":240592,"parent_record_offset":240556,"ancestor_record_offset":240368,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":243180,"parent_record_offset":243156,"ancestor_record_offset":240368,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":242024,"parent_record_offset":241972,"ancestor_record_offset":240368,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":241364,"parent_record_offset":241312,"ancestor_record_offset":240368,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":241264,"parent_record_offset":241240,"ancestor_record_offset":240368,"inlinee":35724,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":241312,"parent_record_offset":241288,"ancestor_record_offset":240368,"inlinee":35659,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":242320,"parent_record_offset":242296,"ancestor_record_offset":240368,"inlinee":87846,"source_line":72,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":242368,"parent_record_offset":242344,"ancestor_record_offset":240368,"inlinee":87847,"source_line":72,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":240928,"parent_record_offset":240904,"ancestor_record_offset":240368,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":240976,"parent_record_offset":240952,"ancestor_record_offset":240368,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":241996,"parent_record_offset":241972,"ancestor_record_offset":240368,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":242948,"parent_record_offset":242924,"ancestor_record_offset":240368,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":262504,"parent_record_offset":262480,"ancestor_record_offset":261576,"inlinee":83141,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":242924,"parent_record_offset":242900,"ancestor_record_offset":240368,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":242972,"parent_record_offset":242948,"ancestor_record_offset":240368,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":240432,"parent_record_offset":240408,"ancestor_record_offset":240368,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":240528,"parent_record_offset":240500,"ancestor_record_offset":240368,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":240676,"parent_record_offset":240652,"ancestor_record_offset":240368,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":240652,"parent_record_offset":240556,"ancestor_record_offset":240368,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":241972,"parent_record_offset":241948,"ancestor_record_offset":240368,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":241924,"parent_record_offset":241900,"ancestor_record_offset":240368,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":241336,"parent_record_offset":241312,"ancestor_record_offset":240368,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":241000,"parent_record_offset":240976,"ancestor_record_offset":240368,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":241632,"parent_record_offset":241608,"ancestor_record_offset":240368,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":240556,"parent_record_offset":240528,"ancestor_record_offset":240368,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":240952,"parent_record_offset":240928,"ancestor_record_offset":240368,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":241948,"parent_record_offset":241924,"ancestor_record_offset":240368,"inlinee":85976,"source_line":317,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1804,"record_offset":262096,"parent_record_offset":262072,"ancestor_record_offset":261576,"inlinee":82380,"source_line":67,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$62::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1574B
// 首条位置: {"module":1804,"inlinee":87883}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":21612,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":21356,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":22508,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":23028,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":21300,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":21392,"parent_record_offset":21236,"ancestor_record_offset":21236,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":21236,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":21484,"parent_record_offset":21448,"ancestor_record_offset":21236,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":21236,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":21236,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":22580,"parent_record_offset":22556,"ancestor_record_offset":21236,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":21236,"inlinee":7736,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":22696,"parent_record_offset":22672,"ancestor_record_offset":21236,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":22604,"parent_record_offset":22580,"ancestor_record_offset":21236,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":21236,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":21236,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":21568,"parent_record_offset":21544,"ancestor_record_offset":21236,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":21544,"parent_record_offset":21448,"ancestor_record_offset":21236,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":22152,"parent_record_offset":22128,"ancestor_record_offset":21236,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":21236,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":21448,"parent_record_offset":21420,"ancestor_record_offset":21236,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":21236,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":96844,"rva":29816832,"va":5398525952}
// 编译实例 RVAs: 0x1c6f800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1833,"record_offset":97072,"parent_record_offset":96844,"ancestor_record_offset":96844,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1833,"record_offset":97096,"parent_record_offset":97072,"ancestor_record_offset":96844,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":97120,"parent_record_offset":97096,"ancestor_record_offset":96844,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":97148,"parent_record_offset":97120,"ancestor_record_offset":96844,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156D7
// 首条位置: {"module":1803,"inlinee":87767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":290708,"parent_record_offset":290684,"ancestor_record_offset":290684,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":290984,"parent_record_offset":290960,"ancestor_record_offset":290684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":291008,"parent_record_offset":290984,"ancestor_record_offset":290684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":291032,"parent_record_offset":291008,"ancestor_record_offset":290684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":290960,"parent_record_offset":290904,"ancestor_record_offset":290684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":290932,"parent_record_offset":290904,"ancestor_record_offset":290684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":290904,"parent_record_offset":290880,"ancestor_record_offset":290684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":290880,"parent_record_offset":290856,"ancestor_record_offset":290684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":290808,"parent_record_offset":290780,"ancestor_record_offset":290684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":290780,"parent_record_offset":290756,"ancestor_record_offset":290684,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":290732,"parent_record_offset":290708,"ancestor_record_offset":290684,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":290756,"parent_record_offset":290732,"ancestor_record_offset":290684,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1575F
// 首条位置: {"module":1804,"inlinee":87903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":85512,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":87664,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":85256,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":87356,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":87384,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":88384,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":85200,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":85292,"parent_record_offset":85160,"ancestor_record_offset":85160,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":87920,"parent_record_offset":87896,"ancestor_record_offset":85160,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":87944,"parent_record_offset":87920,"ancestor_record_offset":85160,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":87968,"parent_record_offset":87944,"ancestor_record_offset":85160,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":87896,"parent_record_offset":87808,"ancestor_record_offset":85160,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":87836,"parent_record_offset":87808,"ancestor_record_offset":85160,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":87808,"parent_record_offset":87784,"ancestor_record_offset":85160,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":87784,"parent_record_offset":87760,"ancestor_record_offset":85160,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":85408,"parent_record_offset":85384,"ancestor_record_offset":85160,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":87864,"parent_record_offset":87836,"ancestor_record_offset":85160,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":85384,"parent_record_offset":85348,"ancestor_record_offset":85160,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":87688,"parent_record_offset":87664,"ancestor_record_offset":85160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":86532,"parent_record_offset":86480,"ancestor_record_offset":85160,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":86156,"parent_record_offset":86104,"ancestor_record_offset":85160,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":86056,"parent_record_offset":86032,"ancestor_record_offset":85160,"inlinee":35724,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1804,"record_offset":86104,"parent_record_offset":86080,"ancestor_record_offset":85160,"inlinee":35659,"source_line":28,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":87196,"parent_record_offset":87172,"ancestor_record_offset":85160,"inlinee":87846,"source_line":72,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1804,"record_offset":87244,"parent_record_offset":87220,"ancestor_record_offset":85160,"inlinee":87847,"source_line":72,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":85720,"parent_record_offset":85696,"ancestor_record_offset":85160,"inlinee":32734,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::_::impl$0::deserialize::impl$0::visit_newtype_struct
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1804,"record_offset":85768,"parent_record_offset":85744,"ancestor_record_offset":85160,"inlinee":37905,"source_line":82,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1804,"record_offset":86504,"parent_record_offset":86480,"ancestor_record_offset":85160,"inlinee":82529,"source_line":50,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":87456,"parent_record_offset":87432,"ancestor_record_offset":85160,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_newtype_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":87432,"parent_record_offset":87408,"ancestor_record_offset":85160,"inlinee":35340,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":87480,"parent_record_offset":87456,"ancestor_record_offset":85160,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":85224,"parent_record_offset":85200,"ancestor_record_offset":85160,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":85320,"parent_record_offset":85292,"ancestor_record_offset":85160,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":85468,"parent_record_offset":85444,"ancestor_record_offset":85160,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":85444,"parent_record_offset":85348,"ancestor_record_offset":85160,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":86480,"parent_record_offset":86456,"ancestor_record_offset":85160,"inlinee":10077,"source_line":901,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":86432,"parent_record_offset":86408,"ancestor_record_offset":85160,"inlinee":9918,"source_line":920,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$70::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":86128,"parent_record_offset":86104,"ancestor_record_offset":85160,"inlinee":9914,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":85792,"parent_record_offset":85768,"ancestor_record_offset":85160,"inlinee":7858,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":87268,"parent_record_offset":87244,"ancestor_record_offset":85160,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":85348,"parent_record_offset":85320,"ancestor_record_offset":85160,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_newtype_struct
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":85744,"parent_record_offset":85720,"ancestor_record_offset":85160,"inlinee":35337,"source_line":351,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":86456,"parent_record_offset":86432,"ancestor_record_offset":85160,"inlinee":85976,"source_line":317,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":95964,"rva":29816640,"va":5398525760}
// 编译实例 RVAs: 0x1c6f740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1833,"record_offset":96192,"parent_record_offset":95964,"ancestor_record_offset":95964,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1833,"record_offset":96216,"parent_record_offset":96192,"ancestor_record_offset":95964,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":96240,"parent_record_offset":96216,"ancestor_record_offset":95964,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1833,"record_offset":96268,"parent_record_offset":96240,"ancestor_record_offset":95964,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":92368,"rva":29814688,"va":5398523808}
// 编译实例 RVAs: 0x1c6efa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141bae830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-788354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e87ef8f3ff","size":5,"soft_fail":false,"target_va":5397735472,"va":5398523821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baf300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-786361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e84700f4ff","size":5,"soft_fail":false,"target_va":5397738240,"va":5398524596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baf810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-785100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e83405f4ff","size":5,"soft_fail":false,"target_va":5397739536,"va":5398524631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bafb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-783826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e82e0af4ff","size":5,"soft_fail":false,"target_va":5397740320,"va":5398524141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bafc30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-783344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8100cf4ff","size":5,"soft_fail":false,"target_va":5397740592,"va":5398523931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baff10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-782678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8aa0ef4ff","size":5,"soft_fail":false,"target_va":5397741328,"va":5398524001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb0320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-782093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8f310f4ff","size":5,"soft_fail":false,"target_va":5397742368,"va":5398524456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb0910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-780153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e88718f4ff","size":5,"soft_fail":false,"target_va":5397743888,"va":5398524036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb0a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-780161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e87f18f4ff","size":5,"soft_fail":false,"target_va":5397744160,"va":5398524316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb0c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-779283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8ed1bf4ff","size":5,"soft_fail":false,"target_va":5397744688,"va":5398523966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-778558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8c21ef4ff","size":5,"soft_fail":false,"target_va":5397745728,"va":5398524281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e86c22f4ff","size":5,"soft_fail":false,"target_va":5397746736,"va":5398524351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8cd22f4ff","size":5,"soft_fail":false,"target_va":5397747008,"va":5398524526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8ba23f4ff","size":5,"soft_fail":false,"target_va":5397747280,"va":5398524561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-776629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e84b26f4ff","size":5,"soft_fail":false,"target_va":5397747552,"va":5398524176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-776346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e86627f4ff","size":5,"soft_fail":false,"target_va":5397748080,"va":5398524421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-775643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8252af4ff","size":5,"soft_fail":false,"target_va":5397748608,"va":5398524246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb1e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-775104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8402cf4ff","size":5,"soft_fail":false,"target_va":5397749392,"va":5398524491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb20a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-774191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8d12ff4ff","size":5,"soft_fail":false,"target_va":5397749920,"va":5398524106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb21b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-773884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e80431f4ff","size":5,"soft_fail":false,"target_va":5397750192,"va":5398524071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb25c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-772984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e88834f4ff","size":5,"soft_fail":false,"target_va":5397751232,"va":5398524211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb2bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-771607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398523808,"raw_hex":"e8e939f4ff","size":5,"soft_fail":false,"target_va":5397752784,"va":5398524386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":92628,"parent_record_offset":92368,"ancestor_record_offset":92368,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":92660,"parent_record_offset":92368,"ancestor_record_offset":92368,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1833,"record_offset":92600,"parent_record_offset":92368,"ancestor_record_offset":92368,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 81, 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1833,"record_offset":46316,"rva":29801232,"va":5398510352}
// 编译实例 RVAs: 0x1c6bb10, 0x1c6def0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t18049782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8f66a1301","size":5,"soft_fail":false,"target_va":5416560512,"va":5398510725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde_core..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15260094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8422617ff","size":5,"soft_fail":false,"target_va":5383250560,"va":5398510649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=6
//     target_aliases=16 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19454498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8de25d7fe","size":5,"soft_fail":false,"target_va":5379056256,"va":5398510749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bac330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-784510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e88207f4ff","size":5,"soft_fail":false,"target_va":5397726000,"va":5398510505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baf920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-770830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8f23cf4ff","size":5,"soft_fail":false,"target_va":5397739808,"va":5398510633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb24c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-759630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8b268f4ff","size":5,"soft_fail":false,"target_va":5397750976,"va":5398510601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141be72e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-542886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e85ab7f7ff","size":5,"soft_fail":false,"target_va":5397967584,"va":5398510465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bed000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-528226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398519536,"raw_hex":"e89ef0f7ff","size":5,"soft_fail":false,"target_va":5397991424,"va":5398519645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c6efa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_compound_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398519536,"raw_hex":"e817100000","size":5,"soft_fail":false,"target_va":5398523808,"va":5398519684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t18050478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398510352,"raw_hex":"e8ae6d1301","size":5,"soft_fail":false,"target_va":5416561184,"va":5398510701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1833,"record_offset":46512,"parent_record_offset":46316,"ancestor_record_offset":46316,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":46632,"parent_record_offset":46576,"ancestor_record_offset":46316,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1833,"record_offset":46668,"parent_record_offset":46576,"ancestor_record_offset":46316,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1833,"record_offset":46576,"parent_record_offset":46512,"ancestor_record_offset":46316,"inlinee":87958,"source_line":345,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1833,"record_offset":46604,"parent_record_offset":46576,"ancestor_record_offset":46316,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::serialize::impl$1::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20AB
// 首条位置: {"module":144,"inlinee":8363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::panic_deprecated_operation
// 记录形式: compiled_procedure; instances=1
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":89972,"rva":47681328,"va":5416390448}
// 编译实例 RVAs: 0x2d78f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t231707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390448,"raw_hex":"e81b890300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416390528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::WallHistoryEdit::update_compatibility
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A88
// 首条位置: {"module":564,"inlinee":35464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":564,"record_offset":14172,"parent_record_offset":14036,"ancestor_record_offset":14036,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":564,"record_offset":14116,"parent_record_offset":14036,"ancestor_record_offset":14036,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":564,"record_offset":14228,"parent_record_offset":14036,"ancestor_record_offset":14036,"inlinee":26087,"source_line":191,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":14036,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":14292,"parent_record_offset":14228,"ancestor_record_offset":14036,"inlinee":26088,"source_line":626,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":14264,"parent_record_offset":14228,"ancestor_record_offset":14036,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":564,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":14036,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::WallHistoryEdit::gather_wall_ids
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A9C
// 首条位置: {"module":564,"inlinee":35484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":564,"record_offset":38604,"parent_record_offset":38576,"ancestor_record_offset":38576,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":564,"record_offset":38628,"parent_record_offset":38604,"ancestor_record_offset":38576,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15730
// 首条位置: {"module":1803,"inlinee":87856}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":479996,"parent_record_offset":479948,"ancestor_record_offset":479948,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":480024,"parent_record_offset":479996,"ancestor_record_offset":479948,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":480052,"parent_record_offset":480024,"ancestor_record_offset":479948,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":480088,"parent_record_offset":480052,"ancestor_record_offset":479948,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15796
// 首条位置: {"module":1804,"inlinee":87958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":325860,"parent_record_offset":325800,"ancestor_record_offset":325800,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":325896,"parent_record_offset":325800,"ancestor_record_offset":325800,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":325832,"parent_record_offset":325800,"ancestor_record_offset":325800,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::wall_compound_edit::impl$4::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":1168,"rva":12080096,"va":5380789216}
// 编译实例 RVAs: 0xb853e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t4025758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380789216,"raw_hex":"e89e6d3d00","size":5,"soft_fail":false,"target_va":5384815120,"va":5380789357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::builders::DebugStruct>::field
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t4019754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380789216,"raw_hex":"e82a563d00","size":5,"soft_fail":false,"target_va":5384809152,"va":5380789393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::builders::DebugStruct>::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4019048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380789216,"raw_hex":"e868533d00","size":5,"soft_fail":false,"target_va":5384809552,"va":5380790499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::panic_deprecated_operation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t35599903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380789216,"raw_hex":"e81f361f02","size":5,"soft_fail":false,"target_va":5416390448,"va":5380790540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: wall_compound_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::CompoundEditBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::wall_compound_edit::CompoundEditBuilder>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::wall_compound_edit::CompoundEditBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::ConcurrentEdits
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "slice2$", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "ref$", "arguments": ["slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "ref$", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "assoc$", "arguments": ["Item", "ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >", "constructor": "core::iter::traits::iterator::Iterator", "arguments": ["assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >", "constructor": "dyn$", "arguments": ["core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>", "country_core::systems::history::replay_wall::impl$0::new::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$212::deserialize<country_core::resources::history::wall_compound_edit::ConcurrentEdits,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$212::deserialize", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<alloc::boxed::Box<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::alloc::Global> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["alloc::boxed::Box<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0> >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$16::clone<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$16::clone", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::resources::history::wall_compound_edit::ConcurrentEdits,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$>", "constructor": "core::result::Result", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::impl$171::advance_by<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::slice::iter::impl$171::advance_by", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::impl$171::nth<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::slice::iter::impl$171::nth", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::impl$171::next<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::slice::iter::impl$171::next", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::impl$171::size_hint<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::slice::iter::impl$171::size_hint", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::impl$1::advance_by<core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::rev::impl$1::advance_by", "arguments": ["core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::impl$1::nth<core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::rev::impl$1::nth", "arguments": ["core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::impl$1::next<core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::rev::impl$1::next", "arguments": ["core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::impl$1::size_hint<core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::rev::impl$1::size_hint", "arguments": ["core::slice::iter::Iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$10::next_element_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "serde_json::value::de::impl$10::next_element_seed", "arguments": ["core::marker::PhantomData<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::map::map_try_fold::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >", "country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "alloc::alloc::Global", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::OperationType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::wall_compound_edit::OperationType>", "constructor": "enum2$", "arguments": ["country_core::resources::history::wall_compound_edit::OperationType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,enum2$<country_core::resources::history::wall_compound_edit::OperationType> >", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "enum2$<country_core::resources::history::wall_compound_edit::OperationType>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::history::wall_compound_edit::OperationType> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::resources::history::wall_compound_edit::OperationType>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::WallHeight
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$10::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::history::wall_compound_edit::WallHeight>", "constructor": "serde_json::ser::impl$10::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::history::wall_compound_edit::WallHeight"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::WallHeight>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::history::wall_compound_edit::WallHeight"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$8::newtype_variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit> >", "constructor": "serde_json::value::de::impl$8::newtype_variant_seed", "arguments": ["core::marker::PhantomData<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::wall_compound_edit::WallHistoryEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 37
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "constructor": "enum2$", "arguments": ["country_core::resources::history::wall_compound_edit::WallHistoryEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "slice2$", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > >", "constructor": "ref$", "arguments": ["slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$212::deserialize<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$212::deserialize", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<alloc::boxed::Box<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["alloc::boxed::Box<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0> >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$1::partial_drop<alloc::vec::Vec<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,5>", "constructor": "core::array::iter::iter_inner::impl$1::partial_drop", "arguments": ["alloc::vec::Vec<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$216::deserialize", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >", "constructor": "alloc::sync::ArcInner", "arguments": ["enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "constructor": "alloc::vec::in_place_collect::from_iter_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "constructor": "core::slice::iter::Iter", "arguments": ["alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "constructor": "core::iter::adapters::map::map_try_fold::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::resources::history::wall_compound_edit::ConcurrentEdits", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >", "country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0", "alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::try_fold<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >", "constructor": "alloc::vec::into_iter::impl$6::try_fold", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>", "alloc::alloc::Global", "alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>", "core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >", "enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_compound_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/wall_compound_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/wall_compound_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/history/wall_compound_edit.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::history::wall_compound_edit::WallHistoryEdit", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_compound_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 25; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::TerrainStairsAudioState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::StairsCounter>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::TerrainStairsAudioState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::StairsCounter>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::TerrainStairsAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_stairs_audio::StairsCounter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
// 信息 ID: bc9c72772b1da49ae053c064
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group @ 0x934f84 -> 0xb8f6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3bd93 -> 0xb8f6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5aef3 -> 0xb8f6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7d988 -> 0xb8f6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_changes::StyleChanges::record @ 0x2145947 -> 0xb8f6f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=67352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::delete_focused::delete_focused; module=525, inline_record=146732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=40624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=48872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=55604; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=59064; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=63176; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=64640; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
// 信息 ID: 79cc4b22f1425e6b75eff5ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 27 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x314376 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327a9d -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327ba1 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::impl$0::add_wall_events @ 0x51d04c -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670085 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x6701e5 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67405b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674185 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67427b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67441b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67451e -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6742f8 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f851 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x721678 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x7239a3 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723a60 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group; module=510, inline_record=72640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=64556; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::delete_focused::delete_focused; module=525, inline_record=141992; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=39544; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=45988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=46776; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=55520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=58980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
// 信息 ID: 8b2c1d259f8274c1070cd953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3acfa -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3bd57 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::finish_wall::finish_wall @ 0xa3bbc3 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3cc19 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3d039 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3d4e7 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3d75c -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3d789 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::set_wall_style::set_wall_style @ 0xa3d9bb -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::deferred_wall_history::history_record_merge_walls @ 0xb48f5c -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_raise_shape::impl_raise_shape @ 0xb52092 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5aeae -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5a465 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62450 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7d945 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle @ 0xb80839 -> 0xb8f4f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next_empty
// 信息 ID: fbfe79af0aa9926b1714483a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group @ 0x934db8 -> 0xb8f270 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=65024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::delete_focused::delete_focused; module=525, inline_record=144292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::delete_focused::delete_focused; module=525, inline_record=145268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::sequence_len
// 信息 ID: 37f33e623546ebf950040da3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot @ 0x64091b -> 0xa13df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::asset_usage_snapshot::dump_asset_usage_snapshot @ 0x64092e -> 0xa13df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67217a -> 0xa13df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672766 -> 0xa13df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_changes::StyleChanges::record @ 0x214591b -> 0xa13df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::with_concurrent
// 信息 ID: 2b98daa3d6ab8c07b2374e55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group @ 0x934ebc -> 0xb8f310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::delete_focused::delete_focused @ 0x9ebedd -> 0xb8f310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::delete_focused::delete_focused @ 0x9ec069 -> 0xb8f310 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=65504; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
// 信息 ID: a99a1f5614ed2ac2571f7f8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2171204 -> 0xb8f7c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=520, inline_record=67444; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::delete_focused::delete_focused; module=525, inline_record=146824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=40716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::finish_wall::finish_wall; module=535, inline_record=48964; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=55696; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=59156; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=63268; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::set_wall_style::set_wall_style; module=535, inline_record=64732; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::WallHistoryEdit::gather_wall_ids
// 信息 ID: 0a76de6ebee5a472caf7a1a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::load_game; module=564, inline_record=38576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::replay_wall; module=1715, inline_record=2704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::replay_wall; module=1715, inline_record=4040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::WallHistoryEdit::update_compatibility
// 信息 ID: 1ae5aa422b37840f0c6e95b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=564, inline_record=14036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize
// 信息 ID: 2db19f1f7ec883d8a60bf3f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$10::next_element_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >; module=1803, inline_record=472312; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$8::newtype_variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit> >; module=1803, inline_record=535024; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=76840; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=87196; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=87548; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=242320; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=243040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_newtype_struct
// 信息 ID: 3a7439153786765a167790f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$10::next_element_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >; module=1803, inline_record=472356; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$8::newtype_variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit> >; module=1803, inline_record=535068; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=87244; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=242368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 6d85c64525c0b6c6f7e333e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=479948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 3b00ecd7e3c68be58568eb08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Field> > @ 0x1bae877 -> 0x1c6f2f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: b328330efc0ae2fbb8de9d74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=479808; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=508916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: de72603432fb2ae4b94daafc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=325800; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1833, inline_record=46576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 15c66e5f23642c617cf377fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=358172; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=369932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$10::deserialize
// 信息 ID: 7ed64ca6984fa080a060f010
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=417664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$11::visit_map
// 信息 ID: 6e29c686daa0699f37c7855d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=50772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$14::visit_map
// 信息 ID: 6a3c5de1a21273589ab38ff4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=192224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$17::visit_map
// 信息 ID: a1e0683a0635f0244f81d5a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=270020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 65de4ea4327465178e3e4163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=358004; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=369740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$20::visit_map
// 信息 ID: c4aae80d79f468699e647bf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=209552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$23::visit_map
// 信息 ID: 60061a1fb1bd419e29055515
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=16484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$26::visit_map
// 信息 ID: 8c772af8fc7b19dc56b4c078
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=37796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$27::visit_str
// 信息 ID: accef9bb4a6c92cd94f181a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=253168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$28::deserialize
// 信息 ID: 3133af2351eac301ab35f02a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=252992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$29::visit_map
// 信息 ID: 834cccbb0b36b23f0f6301d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=233332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 455521acc7429d68fadce531
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=36052; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=75320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$32::visit_map
// 信息 ID: eae8f099b08437431a2d5c91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=111120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$35::visit_map
// 信息 ID: fabf163c3010157c05d6e0d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=18860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$38::visit_map
// 信息 ID: c829bc0d1bd4ef849a3a5350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=79444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str<serde_json::error::Error>
// 信息 ID: 2e8d27ca2739ecd49dc088f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$3::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> > @ 0x1ba8d53 -> 0x1c6f920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$41::visit_map
// 信息 ID: 55642c99edd218e49c771c97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=148704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$44::visit_map
// 信息 ID: 77ac2fbd7828d09d9ceade9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=146360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$45::visit_str
// 信息 ID: ba5124017398f94838f9cb17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=259128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$46::deserialize
// 信息 ID: 28e65075854628314367ddf4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=258940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$47::visit_map
// 信息 ID: 11dbe5a9c3ecf23a72cd498e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=230932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$4::deserialize
// 信息 ID: f4183974b00c03e58c8a166c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=420284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$50::visit_map
// 信息 ID: cdbb219d051631af3d4bc1a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=213716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$51::visit_str
// 信息 ID: 51c2d9c72723b8b0b5c222b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=287516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$52::deserialize
// 信息 ID: 07c0314c44b0873a491192fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=287324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$53::visit_map
// 信息 ID: 83d0b18b23882abe24864953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=283820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$54::visit_str
// 信息 ID: 5e4588ec7ddb97df03697f05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=272836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$55::deserialize
// 信息 ID: ba123dc689845839915e722b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=272664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map
// 信息 ID: 4a4dac16b460e5c5c90cf4ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=154084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$0::deserialize
// 信息 ID: 156dfea176858195fa6330d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=155264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$56::visit_map::impl$1::deserialize
// 信息 ID: 15c7db8276b374c90f28267f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=154976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$57::visit_str
// 信息 ID: 131ef342e41853e7f48abcaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=431956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$58::deserialize
// 信息 ID: 8a4f84a7770f2ba62651111a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=431780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$59::visit_map
// 信息 ID: 354c2ce7eea171ac450305f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=201644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$5::visit_map
// 信息 ID: 672a36f073a5bed65c477312
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=240368; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=261576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$62::visit_map
// 信息 ID: 3f29cfda66d9bfb33afbab62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=21236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str<serde_json::error::Error>
// 信息 ID: a5d01b7cfc457d75e548f76f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$6::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> > @ 0x1ba21d3 -> 0x1c6f800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$7::deserialize
// 信息 ID: acd9929e186d9712a2033c4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=290684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$8::visit_map
// 信息 ID: 5747d01a1c34b82bdb85f8ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=85160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str<serde_json::error::Error>
// 信息 ID: 146c98d5c8344e3d24967e29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::impl$9::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Field> > @ 0x1ba8b33 -> 0x1c6f740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum<serde_json::value::de::EnumDeserializer>
// 信息 ID: d07296044cc881a9a7fa88ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum -> ["serde_json::value::de::EnumDeserializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::__Visitor> @ 0x1bed14f -> 0x1c6efa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> > @ 0x1c6df84 -> 0x1c6efa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 990b8706a601527a20352b87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$216::deserialize<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,enum2$<serde_json::value::Value> > @ 0xc8f488 -> 0x1c6def0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::serialize
// 信息 ID: 24b1c36cbb0e2b0279389367
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=76, inline_record=18812; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >; module=144, inline_record=2088; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >; module=144, inline_record=2196; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >; module=1694, inline_record=11032; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > >; module=1694, inline_record=11140; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1814, inline_record=40236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::serialize::impl$1::serialize
// 信息 ID: f66ec88e00bb3691bbb29089
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::_::impl$0::serialize::__SerializeWith>; module=144, inline_record=98824; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::_::impl$0::serialize::__SerializeWith>; module=1694, inline_record=36000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: ba386b639b5ef996b104fbec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::wall_compound_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount> | serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::WallHeight> @ 0x461cb2 -> 0x193ae0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::walls::inner_wall_state::PlasterPeelAmount> | serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::history::wall_compound_edit::WallHeight> @ 0x18bbd02 -> 0x193ae0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::history::wall_compound_edit::OperationType> > @ 0x4667d2 -> 0x193b80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > > > @ 0x45a438 -> 0x193e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > > > @ 0x45a47b -> 0x193e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::history::wall_compound_edit::OperationType> > @ 0x18b44b2 -> 0x1c6b820 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > > > @ 0x18b0658 -> 0x1c6bd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,ref$<slice2$<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > > > @ 0x18b069b -> 0x1c6bd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$0::new::closure$0
// 信息 ID: 2ddacc0ac204ad048119a776
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::history::wall_compound_edit::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0 <- alloc::boxed::iter::impl$13::from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0>,country_core::resources::history::wall_compound_edit::ConcurrentEdits>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0 <- alloc::vec::into_iter::impl$6::try_fold<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::vec::Vec<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> >,country_core::resources::history::wall_compound_edit::impl$0::new::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<country_core::resources::history::wall_compound_edit::ConcurrentEdits>,never$> > >; module=1822, inline_record=40852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$4::fmt
// 信息 ID: 66c57336d9ad880c63f0201c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$80::fmt<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> > @ 0xc8fdda -> 0xb853e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$5::fmt
// 信息 ID: 3b85f489bf3b553ae7786c45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$6::clone
// 信息 ID: 685b90732c2cfedf54ddb07e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<country_core::resources::history::EditType>,alloc::alloc::Global>; module=1643, inline_record=277192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$7::fmt
// 信息 ID: a9ef18278850f914766e7ff0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<country_core::resources::history::wall_compound_edit::ConcurrentEdits>; module=1833, inline_record=99340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::impl$8::clone
// 信息 ID: 4cb76fa2e0710d417a3c38cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$16::clone<country_core::resources::history::wall_compound_edit::ConcurrentEdits,alloc::alloc::Global>; module=582, inline_record=101236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::wall_compound_edit::panic_deprecated_operation
// 信息 ID: 47611ae5c9a5198c3c612123
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1> @ 0x19e1968 -> 0x2d78f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
