// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e058ff031dda1669b6f3289ffbda970004478cdf49fc8994a654422f561b2d00
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1805, 2150, 3377, 3383, 3576, 3613, 3615, 3625, 4501, 4603, 4949
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"},{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3377,"record_offset":9400,"rva":40174064,"va":5408883184}
// 编译实例 RVAs: 0x26501f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::alloc_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24099280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408883184,"raw_hex":"e8304690fe","size":5,"soft_fail":false,"target_va":5384784192,"va":5408883467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::capacity_overflow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24099256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408883184,"raw_hex":"e8484690fe","size":5,"soft_fail":false,"target_va":5384784144,"va":5408883395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-33204758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408883184,"raw_hex":"e8ea5505fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5408883441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40161241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408883184,"raw_hex":"e827309bfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5408883428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8173563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408883184,"raw_hex":"e8fbb77c00","size":5,"soft_fail":false,"target_va":5417057088,"va":5408883520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3377,"record_offset":10236,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::control::tag::TagSliceExt::fill_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3377,"record_offset":10264,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":12598,"source_line":73,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::RawTableInner::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3377,"record_offset":9768,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":12563,"source_line":1596,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::capacity_to_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3377,"record_offset":9572,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":12594,"source_line":105,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3377,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":9400,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3377,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9400,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3377,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":9400,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3377,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":9400,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3377,"record_offset":9668,"parent_record_offset":9608,"ancestor_record_offset":9400,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3377,"record_offset":9968,"parent_record_offset":9820,"ancestor_record_offset":9400,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3377,"record_offset":9608,"parent_record_offset":9572,"ancestor_record_offset":9400,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3377,"record_offset":9700,"parent_record_offset":9572,"ancestor_record_offset":9400,"inlinee":12595,"source_line":3806,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3377,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":9400,"inlinee":12596,"source_line":3775,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3377,"record_offset":9636,"parent_record_offset":9608,"ancestor_record_offset":9400,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3377,"record_offset":10176,"parent_record_offset":9768,"ancestor_record_offset":9400,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3377,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":9400,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3377,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":9400,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3377,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":9400,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.17.1/src/raw.rs
//     evidence={"module":3377,"record_offset":9820,"parent_record_offset":9768,"ancestor_record_offset":9400,"inlinee":4368,"source_line":216,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::alloc::inner::do_alloc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\alloc.rs
//     evidence={"module":3377,"record_offset":10028,"parent_record_offset":9768,"ancestor_record_offset":9400,"inlinee":12568,"source_line":18,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3377,"record_offset":10204,"parent_record_offset":9768,"ancestor_record_offset":9400,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/03377.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_any<serde::private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":11632,"rva":44542320,"va":5413251440}
// 编译实例 RVAs: 0x2a7a970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1427223e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3508130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e85e78caff","size":5,"soft_fail":false,"target_va":5409743840,"va":5413251965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e892edffff","size":5,"soft_fail":false,"target_va":5413248000,"va":5413252713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_any_struct<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8cfe7ffff","size":5,"soft_fail":false,"target_va":5413246464,"va":5413252652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_struct_after_name<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8beeeffff","size":5,"soft_fail":false,"target_va":5413247200,"va":5413251613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_byte_buf<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t27766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8766c0000","size":5,"soft_fail":false,"target_va":5413280832,"va":5413253061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_char<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-42435242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8567d78fd","size":5,"soft_fail":false,"target_va":5370817840,"va":5413253077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_map<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e83f120000","size":5,"soft_fail":false,"target_va":5413257696,"va":5413253020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_seq<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e826150000","size":5,"soft_fail":false,"target_va":5413258400,"va":5413252981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_string<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-42423662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e892aa78fd","size":5,"soft_fail":false,"target_va":5370829232,"va":5413252889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_tuple<serde::private::de::content::ContentVisitor>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e87c200000","size":5,"soft_fail":false,"target_va":5413260000,"va":5413251679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::any_number
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30095874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8fec534fe","size":5,"soft_fail":false,"target_va":5383156912,"va":5413252781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30089296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8b0df34fe","size":5,"soft_fail":false,"target_va":5383162208,"va":5413251499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_ident
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30085662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8e2ed34fe","size":5,"soft_fail":false,"target_va":5383166128,"va":5413251785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_struct_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30071240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8382635fe","size":5,"soft_fail":false,"target_va":5383180336,"va":5413251571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30061913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8a74a35fe","size":5,"soft_fail":false,"target_va":5383190144,"va":5413252052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30084931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8bdf034fe","size":5,"soft_fail":false,"target_va":5383166960,"va":5413251886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30080646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e87a0135fe","size":5,"soft_fail":false,"target_va":5383171072,"va":5413251713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30084943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8b1f034fe","size":5,"soft_fail":false,"target_va":5383166592,"va":5413251530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30072543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8212135fe","size":5,"soft_fail":false,"target_va":5383180192,"va":5413252730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30075123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e80d1735fe","size":5,"soft_fail":false,"target_va":5383177488,"va":5413252606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30057612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8745b35fe","size":5,"soft_fail":false,"target_va":5383194256,"va":5413251863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30063966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e8a24235fe","size":5,"soft_fail":false,"target_va":5383189072,"va":5413253033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::value::number::Number::visit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2577710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251440,"raw_hex":"e82e552700","size":5,"soft_fail":false,"target_va":5415830528,"va":5413252813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11888,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11924,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4501,"record_offset":12820,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_option
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":12008,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":10036,"source_line":500,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":11828,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":10034,"source_line":536,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$21::visit_bool
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":11952,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":10035,"source_line":320,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$21::visit_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":12568,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":10037,"source_line":383,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$21::visit_f64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":12596,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":10038,"source_line":390,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$21::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":12540,"parent_record_offset":11632,"ancestor_record_offset":11632,"inlinee":9921,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4501,"record_offset":12892,"parent_record_offset":12868,"ancestor_record_offset":11632,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":12316,"parent_record_offset":12008,"ancestor_record_offset":11632,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":12384,"parent_record_offset":12008,"ancestor_record_offset":11632,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":12196,"parent_record_offset":12008,"ancestor_record_offset":11632,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":12508,"parent_record_offset":12008,"ancestor_record_offset":11632,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4501,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":11632,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4501,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":11632,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4501,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":11632,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4501,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":11632,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":4501,"record_offset":12252,"parent_record_offset":12008,"ancestor_record_offset":11632,"inlinee":10013,"source_line":696,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":4501,"record_offset":12280,"parent_record_offset":12252,"ancestor_record_offset":11632,"inlinee":10015,"source_line":696,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$21::visit_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":11856,"parent_record_offset":11828,"ancestor_record_offset":11632,"inlinee":9921,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_byte_buf<serde::private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":52488,"rva":44571712,"va":5413280832}
// 编译实例 RVAs: 0x2a81c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_core..de..impls..$LT$impl$u20$serde_core..de..Deserialize$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$..deserialize..VecVisitor$LT$T$GT$$u20$as$u20$serde_core..de..Visitor$GT$::visit_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t306149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e8e5ab0400","size":5,"soft_fail":false,"target_va":5413587120,"va":5413280966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-37602690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e87e3ac2fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413281389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::byte_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30116772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e85c7434fe","size":5,"soft_fail":false,"target_va":5383164352,"va":5413281119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30118678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e8ea6c34fe","size":5,"soft_fail":false,"target_va":5383162208,"va":5413280881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30113951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e8617f34fe","size":5,"soft_fail":false,"target_va":5383166960,"va":5413280906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30086789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413280832,"raw_hex":"e87be934fe","size":5,"soft_fail":false,"target_va":5383194256,"va":5413281040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":53720,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":53756,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$21::visit_byte_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":53692,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":10113,"source_line":439,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":52668,"parent_record_offset":52488,"ancestor_record_offset":52488,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":53068,"parent_record_offset":53044,"ancestor_record_offset":52488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":53092,"parent_record_offset":53068,"ancestor_record_offset":52488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":53116,"parent_record_offset":53092,"ancestor_record_offset":52488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":53044,"parent_record_offset":52988,"ancestor_record_offset":52488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":53016,"parent_record_offset":52988,"ancestor_record_offset":52488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":52988,"parent_record_offset":52964,"ancestor_record_offset":52488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":52964,"parent_record_offset":52940,"ancestor_record_offset":52488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":52776,"parent_record_offset":52700,"ancestor_record_offset":52488,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":52852,"parent_record_offset":52700,"ancestor_record_offset":52488,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4501,"record_offset":52916,"parent_record_offset":52700,"ancestor_record_offset":52488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":52880,"parent_record_offset":52700,"ancestor_record_offset":52488,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":52808,"parent_record_offset":52700,"ancestor_record_offset":52488,"inlinee":9928,"source_line":804,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":52700,"parent_record_offset":52668,"ancestor_record_offset":52488,"inlinee":10129,"source_line":611,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":48744,"rva":44567824,"va":5413276944}
// 编译实例 RVAs: 0x2a80d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3276749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8cdff3100","size":5,"soft_fail":false,"target_va":5416555360,"va":5413278606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1990
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31324603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8450622fe","size":5,"soft_fail":false,"target_va":5381953936,"va":5413278534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7b010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8a79dffff","size":5,"soft_fail":false,"target_va":5413253136,"va":5413278308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8cdb6ffff","size":5,"soft_fail":false,"target_va":5413258848,"va":5413277582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30124524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8145634fe","size":5,"soft_fail":false,"target_va":5383152768,"va":5413277287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::is_serde_content::type_id_of_untagged_enum_default_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30125318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8fa5234fe","size":5,"soft_fail":false,"target_va":5383152928,"va":5413278241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::is_serde_tag_or_content::type_id_of_internally_tagged_enum_default_tag_or_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30123496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8185a34fe","size":5,"soft_fail":false,"target_va":5383153840,"va":5413277331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30126871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8e94c34fe","size":5,"soft_fail":false,"target_va":5383152992,"va":5413279858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30088286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8a2e334fe","size":5,"soft_fail":false,"target_va":5383190144,"va":5413278425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30110192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8108e34fe","size":5,"soft_fail":false,"target_va":5383166960,"va":5413277147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30095481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e887c734fe","size":5,"soft_fail":false,"target_va":5383181600,"va":5413277076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30114973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8637b34fe","size":5,"soft_fail":false,"target_va":5383162464,"va":5413277432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30082860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8d4f834fe","size":5,"soft_fail":false,"target_va":5383194256,"va":5413277111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2551211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413276944,"raw_hex":"e8abed2600","size":5,"soft_fail":false,"target_va":5415830720,"va":5413279504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":48932,"parent_record_offset":48744,"ancestor_record_offset":48744,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":52456,"parent_record_offset":48744,"ancestor_record_offset":48744,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49004,"parent_record_offset":48744,"ancestor_record_offset":48744,"inlinee":9953,"source_line":242,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4501,"record_offset":49832,"parent_record_offset":49804,"ancestor_record_offset":48744,"inlinee":129600,"source_line":70,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4501,"record_offset":49680,"parent_record_offset":49652,"ancestor_record_offset":48744,"inlinee":129617,"source_line":5,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4501,"record_offset":49232,"parent_record_offset":49004,"ancestor_record_offset":48744,"inlinee":129618,"source_line":5,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":4501,"record_offset":49552,"parent_record_offset":49528,"ancestor_record_offset":48744,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":49580,"parent_record_offset":49552,"ancestor_record_offset":48744,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4501,"record_offset":49888,"parent_record_offset":49864,"ancestor_record_offset":48744,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":49172,"parent_record_offset":49004,"ancestor_record_offset":48744,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":50156,"parent_record_offset":49332,"ancestor_record_offset":48744,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4501,"record_offset":50676,"parent_record_offset":49232,"ancestor_record_offset":48744,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4501,"record_offset":51236,"parent_record_offset":50752,"ancestor_record_offset":48744,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":50120,"parent_record_offset":49332,"ancestor_record_offset":48744,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":52220,"parent_record_offset":49004,"ancestor_record_offset":48744,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":49492,"parent_record_offset":49332,"ancestor_record_offset":48744,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":52284,"parent_record_offset":49004,"ancestor_record_offset":48744,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4501,"record_offset":49912,"parent_record_offset":49888,"ancestor_record_offset":48744,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4501,"record_offset":49940,"parent_record_offset":49912,"ancestor_record_offset":48744,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4501,"record_offset":49864,"parent_record_offset":49832,"ancestor_record_offset":48744,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4501,"record_offset":51308,"parent_record_offset":51284,"ancestor_record_offset":48744,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49204,"parent_record_offset":49004,"ancestor_record_offset":48744,"inlinee":9928,"source_line":804,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":49708,"parent_record_offset":49680,"ancestor_record_offset":48744,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":52244,"parent_record_offset":52220,"ancestor_record_offset":48744,"inlinee":9930,"source_line":257,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49736,"parent_record_offset":49708,"ancestor_record_offset":48744,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49332,"parent_record_offset":49292,"ancestor_record_offset":48744,"inlinee":9956,"source_line":852,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":50752,"parent_record_offset":50704,"ancestor_record_offset":48744,"inlinee":9908,"source_line":873,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":50924,"parent_record_offset":50752,"ancestor_record_offset":48744,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49528,"parent_record_offset":49332,"ancestor_record_offset":48744,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49292,"parent_record_offset":49232,"ancestor_record_offset":48744,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":50704,"parent_record_offset":49232,"ancestor_record_offset":48744,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49804,"parent_record_offset":49736,"ancestor_record_offset":48744,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":49652,"parent_record_offset":49332,"ancestor_record_offset":48744,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":51072,"parent_record_offset":51048,"ancestor_record_offset":48744,"inlinee":83339,"source_line":1515,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4501,"record_offset":51640,"parent_record_offset":49232,"ancestor_record_offset":48744,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4501,"record_offset":51688,"parent_record_offset":51664,"ancestor_record_offset":48744,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4501,"record_offset":51664,"parent_record_offset":51640,"ancestor_record_offset":48744,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":11152,"rva":44542000,"va":5413251120}
// 编译实例 RVAs: 0x2a7a830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142cefb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2577041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251120,"raw_hex":"e891522700","size":5,"soft_fail":false,"target_va":5415828256,"va":5413251210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30098375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251120,"raw_hex":"e839bc34fe","size":5,"soft_fail":false,"target_va":5383152768,"va":5413251138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30061136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413251120,"raw_hex":"e8b04d35fe","size":5,"soft_fail":false,"target_va":5383190144,"va":5413251275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":11348,"parent_record_offset":11152,"ancestor_record_offset":11152,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":11512,"parent_record_offset":11152,"ancestor_record_offset":11152,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11320,"parent_record_offset":11152,"ancestor_record_offset":11152,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11600,"parent_record_offset":11152,"ancestor_record_offset":11152,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":11380,"parent_record_offset":11152,"ancestor_record_offset":11152,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11464,"parent_record_offset":11436,"ancestor_record_offset":11152,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":11436,"parent_record_offset":11408,"ancestor_record_offset":11152,"inlinee":10024,"source_line":404,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$64::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":11408,"parent_record_offset":11380,"ancestor_record_offset":11152,"inlinee":7684,"source_line":132,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$5::next_element_seed<serde::private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":10660,"rva":44541504,"va":5413250624}
// 编译实例 RVAs: 0x2a7a640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413250624,"raw_hex":"e8c6020000","size":5,"soft_fail":false,"target_va":5413251440,"va":5413250725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413250624,"raw_hex":"e81af4ffff","size":5,"soft_fail":false,"target_va":5413248000,"va":5413251041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30097897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413250624,"raw_hex":"e817be34fe","size":5,"soft_fail":false,"target_va":5383152768,"va":5413250660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30060676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413250624,"raw_hex":"e87c4f35fe","size":5,"soft_fail":false,"target_va":5383190144,"va":5413250815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":10876,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":11000,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":10840,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":11064,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":10908,"parent_record_offset":10660,"ancestor_record_offset":10660,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":10932,"parent_record_offset":10908,"ancestor_record_offset":10660,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<core::marker::PhantomData<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":7220,"rva":44539376,"va":5413248496}
// 编译实例 RVAs: 0x2a79df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$ron..de..id..Deserializer$u20$as$u20$serde_core..de..Deserializer$GT$::deserialize_string
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t252085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e8b5d80300","size":5,"soft_fail":false,"target_va":5413500864,"va":5413248774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-37570373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e8bbb8c2fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413249056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44526850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e8fe9258fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413249037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3807969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e8e11a3a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413249082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3371883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e86b733300","size":5,"soft_fail":false,"target_va":5416620794,"va":5413248906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30095757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e873c634fe","size":5,"soft_fail":false,"target_va":5383152768,"va":5413248520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::is_serde_tag_or_content::type_id_of_internally_tagged_enum_default_tag_or_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30094718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e882ca34fe","size":5,"soft_fail":false,"target_va":5383153840,"va":5413248553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30055028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413248496,"raw_hex":"e88c6535fe","size":5,"soft_fail":false,"target_va":5383193824,"va":5413248847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":7540,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":8692,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":8656,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":7416,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":7600,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::id::Deserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":7572,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":9971,"source_line":11,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":7448,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":7700,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7220,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":7220,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":7220,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":8428,"parent_record_offset":8276,"ancestor_record_offset":7220,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7220,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":8276,"parent_record_offset":8240,"ancestor_record_offset":7220,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":8240,"parent_record_offset":8212,"ancestor_record_offset":7220,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":8316,"parent_record_offset":8276,"ancestor_record_offset":7220,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":8096,"parent_record_offset":8068,"ancestor_record_offset":7220,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":8124,"parent_record_offset":8096,"ancestor_record_offset":7220,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":7220,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":8068,"parent_record_offset":8040,"ancestor_record_offset":7220,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4501,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":7220,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4501,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":7220,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4501,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":7220,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":4501,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":7220,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":7220,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7220,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7220,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4501,"record_offset":8556,"parent_record_offset":8152,"ancestor_record_offset":7220,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4501,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":7220,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":7948,"parent_record_offset":7908,"ancestor_record_offset":7220,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":7220,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":7876,"parent_record_offset":7844,"ancestor_record_offset":7220,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":7984,"parent_record_offset":7908,"ancestor_record_offset":7220,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":8012,"parent_record_offset":7984,"ancestor_record_offset":7220,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":7748,"parent_record_offset":7700,"ancestor_record_offset":7220,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<serde::private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":8724,"rva":44540048,"va":5413249168}
// 编译实例 RVAs: 0x2a7a090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249168,"raw_hex":"e84d070000","size":5,"soft_fail":false,"target_va":5413251440,"va":5413249566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30096423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249168,"raw_hex":"e8d9c334fe","size":5,"soft_fail":false,"target_va":5383152768,"va":5413249186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::is_serde_tag_or_content::type_id_of_internally_tagged_enum_default_tag_or_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30095388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249168,"raw_hex":"e8e4c734fe","size":5,"soft_fail":false,"target_va":5383153840,"va":5413249223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30086860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249168,"raw_hex":"e834e934fe","size":5,"soft_fail":false,"target_va":5383162464,"va":5413249319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30055655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249168,"raw_hex":"e8196335fe","size":5,"soft_fail":false,"target_va":5383193824,"va":5413249474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":9036,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":9756,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":9624,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":8908,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":9272,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":8944,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":9068,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":4501,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":8724,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":8724,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":9216,"parent_record_offset":9180,"ancestor_record_offset":8724,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":8724,"inlinee":10021,"source_line":45,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":9152,"parent_record_offset":9124,"ancestor_record_offset":8724,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":9180,"parent_record_offset":9152,"ancestor_record_offset":8724,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":8724,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":8724,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<serde::private::de::content::ContentVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4501,"record_offset":9788,"rva":44540752,"va":5413249872}
// 编译实例 RVAs: 0x2a7a350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142a7a970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e864050000","size":5,"soft_fail":false,"target_va":5413251440,"va":5413250055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<serde_core::private::content::Content> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e863f6ffff","size":5,"soft_fail":false,"target_va":5413248000,"va":5413250456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::is_serde_content::type_id_of_untagged_enum_default_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30097066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e856c134fe","size":5,"soft_fail":false,"target_va":5383152928,"va":5413249989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30060005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e81b5235fe","size":5,"soft_fail":false,"target_va":5383190144,"va":5413250144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30082989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e853f834fe","size":5,"soft_fail":false,"target_va":5383166960,"va":5413249944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30055670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e80a6335fe","size":5,"soft_fail":false,"target_va":5383194256,"va":5413249921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30056536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413249872,"raw_hex":"e8a85f35fe","size":5,"soft_fail":false,"target_va":5383193824,"va":5413250355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":10120,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":10244,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":9972,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":10336,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":10028,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$20::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":4501,"record_offset":10152,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":7661,"source_line":304,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":4501,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":9788,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":10080,"parent_record_offset":10052,"ancestor_record_offset":9788,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":10544,"parent_record_offset":10508,"ancestor_record_offset":9788,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":9788,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":9788,"inlinee":9609,"source_line":39,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9788,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Deserializer::__deserialize_content_v1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":9788,"inlinee":7663,"source_line":1260,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":13848,"rva":47119136,"va":5415828256}
// 编译实例 RVAs: 0x2cefb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-32632860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8e40f0efe","size":5,"soft_fail":false,"target_va":5383197376,"va":5415830231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-32632566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e80a110efe","size":5,"soft_fail":false,"target_va":5383196976,"va":5415829537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e804000000","size":5,"soft_fail":false,"target_va":5415830336,"va":5415830327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t792718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e88e180c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415828653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32662526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8029c0dfe","size":5,"soft_fail":false,"target_va":5383166960,"va":5415829481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32657464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8c8af0dfe","size":5,"soft_fail":false,"target_va":5383171072,"va":5415828531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32662087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8b99d0dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415828674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32632699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e885100efe","size":5,"soft_fail":false,"target_va":5383196000,"va":5415828694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32645858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e81edd0dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415828381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32661683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e84d9f0dfe","size":5,"soft_fail":false,"target_va":5383167056,"va":5415828734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e824e5ffff","size":5,"soft_fail":false,"target_va":5415822128,"va":5415828999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8d0e5ffff","size":5,"soft_fail":false,"target_va":5415822416,"va":5415829115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e81cecffff","size":5,"soft_fail":false,"target_va":5415824160,"va":5415829247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e825eaffff","size":5,"soft_fail":false,"target_va":5415822992,"va":5415828582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8f9e4ffff","size":5,"soft_fail":false,"target_va":5415822704,"va":5415829618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e82de1ffff","size":5,"soft_fail":false,"target_va":5415821840,"va":5415829726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e88de8ffff","size":5,"soft_fail":false,"target_va":5415823872,"va":5415829870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e88eecffff","size":5,"soft_fail":false,"target_va":5415824448,"va":5415829421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32648131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e83dd40dfe","size":5,"soft_fail":false,"target_va":5383180192,"va":5415828318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32652550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8fac20dfe","size":5,"soft_fail":false,"target_va":5383177488,"va":5415830033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32650995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e80dc90dfe","size":5,"soft_fail":false,"target_va":5383177376,"va":5415828366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32639233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415828256,"raw_hex":"e8fff60dfe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415828300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":14908,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":15620,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":14664,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":13992,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":14992,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":14020,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":14604,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":14636,"parent_record_offset":13848,"ancestor_record_offset":13848,"inlinee":15547,"source_line":126,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":14176,"parent_record_offset":14136,"ancestor_record_offset":13848,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":14432,"parent_record_offset":14408,"ancestor_record_offset":13848,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":14408,"parent_record_offset":14356,"ancestor_record_offset":13848,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":14136,"parent_record_offset":14108,"ancestor_record_offset":13848,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":13848,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":14300,"parent_record_offset":14272,"ancestor_record_offset":13848,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4949,"record_offset":14244,"parent_record_offset":14216,"ancestor_record_offset":13848,"inlinee":15552,"source_line":115,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4949,"record_offset":14272,"parent_record_offset":14244,"ancestor_record_offset":13848,"inlinee":8299,"source_line":689,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$13::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4949,"record_offset":14216,"parent_record_offset":14020,"ancestor_record_offset":13848,"inlinee":15550,"source_line":746,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":15148,"parent_record_offset":15112,"ancestor_record_offset":13848,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":15112,"parent_record_offset":15080,"ancestor_record_offset":13848,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":14108,"parent_record_offset":14076,"ancestor_record_offset":13848,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":14076,"parent_record_offset":14020,"ancestor_record_offset":13848,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":14356,"parent_record_offset":14328,"ancestor_record_offset":13848,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":14492,"parent_record_offset":14356,"ancestor_record_offset":13848,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":14464,"parent_record_offset":14356,"ancestor_record_offset":13848,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":15656,"parent_record_offset":15620,"ancestor_record_offset":13848,"inlinee":15554,"source_line":439,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":3312,"rva":47113008,"va":5415822128}
// 编译实例 RVAs: 0x2cee330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t799013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e825310c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415822358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32660067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e89da50dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415822270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32655576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e828b70dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415822163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32639726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e812f50dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415822249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e8230f0000","size":5,"soft_fail":false,"target_va":5415826272,"va":5415822392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32633225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822128,"raw_hex":"e8770e0efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415822292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3436,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":3556,"parent_record_offset":3524,"ancestor_record_offset":3312,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":3312,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3496,"parent_record_offset":3464,"ancestor_record_offset":3312,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":3312,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":3604,"rva":47113296,"va":5415822416}
// 编译实例 RVAs: 0x2cee450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t798725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e805300c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415822646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32660355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e87da40dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415822558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32655864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e808b60dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415822451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32640014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e8f2f30dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415822537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e8130c0000","size":5,"soft_fail":false,"target_va":5415825776,"va":5415822680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32633513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822416,"raw_hex":"e8570d0efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415822580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3728,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":3604,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":3604,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3788,"parent_record_offset":3756,"ancestor_record_offset":3604,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3756,"parent_record_offset":3728,"ancestor_record_offset":3604,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":6528,"rva":47115040,"va":5415824160}
// 编译实例 RVAs: 0x2ceeb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t796981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e835290c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415824390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32662099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e8ad9d0dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415824302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32657608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e838af0dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415824195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32641758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e822ed0dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415824281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e843030000","size":5,"soft_fail":false,"target_va":5415825264,"va":5415824424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32635257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824160,"raw_hex":"e887060efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415824324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6652,"parent_record_offset":6528,"ancestor_record_offset":6528,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":6772,"parent_record_offset":6740,"ancestor_record_offset":6528,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":6740,"parent_record_offset":6712,"ancestor_record_offset":6528,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6712,"parent_record_offset":6680,"ancestor_record_offset":6528,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":6528,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":4188,"rva":47113872,"va":5415822992}
// 编译实例 RVAs: 0x2cee690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t797565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822992,"raw_hex":"e87d2b0c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415823806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32660935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822992,"raw_hex":"e839a20dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415823138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32656442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822992,"raw_hex":"e8c6b30dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415823029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32640594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822992,"raw_hex":"e8aef10dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415823117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32634099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822992,"raw_hex":"e80d0b0efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415823166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4316,"parent_record_offset":4188,"ancestor_record_offset":4188,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":4484,"parent_record_offset":4188,"ancestor_record_offset":4188,"inlinee":15513,"source_line":236,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":5360,"parent_record_offset":5328,"ancestor_record_offset":4188,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":5292,"parent_record_offset":5236,"ancestor_record_offset":4188,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":4188,"inlinee":15520,"source_line":523,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":4188,"inlinee":15516,"source_line":803,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4188,"inlinee":15523,"source_line":663,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":5444,"parent_record_offset":5416,"ancestor_record_offset":4188,"inlinee":15521,"source_line":2498,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":4188,"inlinee":15517,"source_line":2710,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$0::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":4188,"inlinee":15524,"source_line":2604,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":4440,"parent_record_offset":4404,"ancestor_record_offset":4188,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":4188,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4188,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4188,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":4744,"parent_record_offset":4484,"ancestor_record_offset":4188,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":4188,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":4188,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":4976,"parent_record_offset":4912,"ancestor_record_offset":4188,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":4540,"parent_record_offset":4484,"ancestor_record_offset":4188,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":4188,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":4404,"parent_record_offset":4376,"ancestor_record_offset":4188,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":4884,"parent_record_offset":4848,"ancestor_record_offset":4188,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":4848,"parent_record_offset":4808,"ancestor_record_offset":4188,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4376,"parent_record_offset":4344,"ancestor_record_offset":4188,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":4188,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":4188,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":5060,"parent_record_offset":4912,"ancestor_record_offset":4188,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":5032,"parent_record_offset":4912,"ancestor_record_offset":4188,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":4680,"parent_record_offset":4484,"ancestor_record_offset":4188,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":5328,"parent_record_offset":4808,"ancestor_record_offset":4188,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$11::checked_add_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":5392,"parent_record_offset":4808,"ancestor_record_offset":4188,"inlinee":15519,"source_line":1394,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$11::checked_mul_ext
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":5212,"parent_record_offset":4808,"ancestor_record_offset":4188,"inlinee":15515,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$11::checked_sub_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":6112,"parent_record_offset":5512,"ancestor_record_offset":4188,"inlinee":15522,"source_line":1404,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":3896,"rva":47113584,"va":5415822704}
// 编译实例 RVAs: 0x2cee570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t798437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e8e52e0c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415822934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32660643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e85da30dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415822846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32656152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e8e8b40dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415822739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32640302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e8d2f20dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415822825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e8c3100000","size":5,"soft_fail":false,"target_va":5415827264,"va":5415822968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32633801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415822704,"raw_hex":"e8370c0efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415822868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4020,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3896,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3896,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4080,"parent_record_offset":4048,"ancestor_record_offset":3896,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3896,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":3020,"rva":47112720,"va":5415821840}
// 编译实例 RVAs: 0x2cee210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t799301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e845320c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415822070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32659779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e8bda60dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415821982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32655288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e848b80dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415821875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32639438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e832f60dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415821961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e813160000","size":5,"soft_fail":false,"target_va":5415827760,"va":5415822104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32632937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415821840,"raw_hex":"e8970f0efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415822004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3144,"parent_record_offset":3020,"ancestor_record_offset":3020,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":3264,"parent_record_offset":3232,"ancestor_record_offset":3020,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":3232,"parent_record_offset":3204,"ancestor_record_offset":3020,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3204,"parent_record_offset":3172,"ancestor_record_offset":3020,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":3020,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":6236,"rva":47114752,"va":5415823872}
// 编译实例 RVAs: 0x2ceea00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t797269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e8552a0c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415824102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32661811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e8cd9e0dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415824014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32657320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e858b00dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415823907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32641470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e842ee0dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415823993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e853020000","size":5,"soft_fail":false,"target_va":5415824736,"va":5415824136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32634969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415823872,"raw_hex":"e8a7070efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415824036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6360,"parent_record_offset":6236,"ancestor_record_offset":6236,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":6480,"parent_record_offset":6448,"ancestor_record_offset":6236,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":6236,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6420,"parent_record_offset":6388,"ancestor_record_offset":6236,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6388,"parent_record_offset":6360,"ancestor_record_offset":6236,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":6820,"rva":47115328,"va":5415824448}
// 编译实例 RVAs: 0x2ceec40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t796693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e815280c00","size":5,"soft_fail":false,"target_va":5416621376,"va":5415824678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32662387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e88d9c0dfe","size":5,"soft_fail":false,"target_va":5383162208,"va":5415824590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32657896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e818ae0dfe","size":5,"soft_fail":false,"target_va":5383166592,"va":5415824483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32642046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e802ec0dfe","size":5,"soft_fail":false,"target_va":5383182528,"va":5415824569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer_digits<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e813080000","size":5,"soft_fail":false,"target_va":5415826784,"va":5415824712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32635545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415824448,"raw_hex":"e867050efe","size":5,"soft_fail":false,"target_va":5383189072,"va":5415824612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6944,"parent_record_offset":6820,"ancestor_record_offset":6820,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4949,"record_offset":7064,"parent_record_offset":7032,"ancestor_record_offset":6820,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":7032,"parent_record_offset":7004,"ancestor_record_offset":6820,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":7004,"parent_record_offset":6972,"ancestor_record_offset":6820,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4949,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":6820,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<i16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":9996,"rva":47117152,"va":5415826272}
// 编译实例 RVAs: 0x2cef360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415826272,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415826597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":10892,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":10148,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":10288,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":10828,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":10764,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$12::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":10644,"parent_record_offset":9996,"ancestor_record_offset":9996,"inlinee":15538,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":9996,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":10728,"parent_record_offset":10672,"ancestor_record_offset":9996,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$1::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":10672,"parent_record_offset":10644,"ancestor_record_offset":9996,"inlinee":15539,"source_line":803,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$1::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":10700,"parent_record_offset":10672,"ancestor_record_offset":9996,"inlinee":15540,"source_line":2710,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":10244,"parent_record_offset":10220,"ancestor_record_offset":9996,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9996,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":9996,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":9996,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":9996,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":10408,"parent_record_offset":10348,"ancestor_record_offset":9996,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":10172,"parent_record_offset":10148,"ancestor_record_offset":9996,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":9996,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":9996,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":10492,"parent_record_offset":10348,"ancestor_record_offset":9996,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":10464,"parent_record_offset":10348,"ancestor_record_offset":9996,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<i32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":9036,"rva":47116656,"va":5415825776}
// 编译实例 RVAs: 0x2cef170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415825776,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415826098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":9932,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":9188,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":9328,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":9868,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":9804,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$13::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":9684,"parent_record_offset":9036,"ancestor_record_offset":9036,"inlinee":15529,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":9836,"parent_record_offset":9804,"ancestor_record_offset":9036,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":9768,"parent_record_offset":9712,"ancestor_record_offset":9036,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":9712,"parent_record_offset":9684,"ancestor_record_offset":9036,"inlinee":15530,"source_line":803,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":9740,"parent_record_offset":9712,"ancestor_record_offset":9036,"inlinee":15531,"source_line":2710,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":9036,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":9036,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":9036,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":9036,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":9036,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":9448,"parent_record_offset":9388,"ancestor_record_offset":9036,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":9036,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":9360,"parent_record_offset":9328,"ancestor_record_offset":9036,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":9388,"parent_record_offset":9360,"ancestor_record_offset":9036,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":9532,"parent_record_offset":9388,"ancestor_record_offset":9036,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":9504,"parent_record_offset":9388,"ancestor_record_offset":9036,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<i64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":8076,"rva":47116144,"va":5415825264}
// 编译实例 RVAs: 0x2ceef70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415825264,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415825604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":8972,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":8228,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":8368,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":8908,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":8844,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$14::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":8724,"parent_record_offset":8076,"ancestor_record_offset":8076,"inlinee":15533,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":8876,"parent_record_offset":8844,"ancestor_record_offset":8076,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":8808,"parent_record_offset":8752,"ancestor_record_offset":8076,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":8076,"inlinee":6813,"source_line":803,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4949,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":8076,"inlinee":6814,"source_line":2710,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":8076,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":8076,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":8076,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":8076,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":8076,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":8488,"parent_record_offset":8428,"ancestor_record_offset":8076,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":8076,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":8400,"parent_record_offset":8368,"ancestor_record_offset":8076,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":8428,"parent_record_offset":8400,"ancestor_record_offset":8076,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":8572,"parent_record_offset":8428,"ancestor_record_offset":8076,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":8544,"parent_record_offset":8428,"ancestor_record_offset":8076,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<u16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":11920,"rva":47118144,"va":5415827264}
// 编译实例 RVAs: 0x2cef740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415827264,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415827586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":12820,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":12072,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":12212,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":12756,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":12692,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$16::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":12572,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":15526,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":12724,"parent_record_offset":12692,"ancestor_record_offset":11920,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":12656,"parent_record_offset":12600,"ancestor_record_offset":11920,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":12600,"parent_record_offset":12572,"ancestor_record_offset":11920,"inlinee":15527,"source_line":1160,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":12628,"parent_record_offset":12600,"ancestor_record_offset":11920,"inlinee":15528,"source_line":3050,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":11920,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":11920,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":11920,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":11920,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11920,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":12336,"parent_record_offset":12272,"ancestor_record_offset":11920,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":11920,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":12244,"parent_record_offset":12212,"ancestor_record_offset":11920,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":11920,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":12420,"parent_record_offset":12272,"ancestor_record_offset":11920,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":12392,"parent_record_offset":12272,"ancestor_record_offset":11920,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":12884,"rva":47118640,"va":5415827760}
// 编译实例 RVAs: 0x2cef930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415827760,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415828080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":13784,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":13036,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":13176,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":13720,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":13656,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$17::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":13536,"parent_record_offset":12884,"ancestor_record_offset":12884,"inlinee":15532,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":12884,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":13620,"parent_record_offset":13564,"ancestor_record_offset":12884,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":13564,"parent_record_offset":13536,"ancestor_record_offset":12884,"inlinee":13733,"source_line":1160,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":13592,"parent_record_offset":13564,"ancestor_record_offset":12884,"inlinee":13734,"source_line":3050,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":13132,"parent_record_offset":13108,"ancestor_record_offset":12884,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":12884,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":12884,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12884,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":13108,"parent_record_offset":13084,"ancestor_record_offset":12884,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":13300,"parent_record_offset":13236,"ancestor_record_offset":12884,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12884,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":13208,"parent_record_offset":13176,"ancestor_record_offset":12884,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":13236,"parent_record_offset":13208,"ancestor_record_offset":12884,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":13384,"parent_record_offset":13236,"ancestor_record_offset":12884,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":13356,"parent_record_offset":13236,"ancestor_record_offset":12884,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":7112,"rva":47115616,"va":5415824736}
// 编译实例 RVAs: 0x2ceed60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415824736,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415825076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":8012,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":7264,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":7404,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":7948,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":7884,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$18::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":7764,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":15541,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":7916,"parent_record_offset":7884,"ancestor_record_offset":7112,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":7848,"parent_record_offset":7792,"ancestor_record_offset":7112,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":7792,"parent_record_offset":7764,"ancestor_record_offset":7112,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":7112,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":7112,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":7112,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":7552,"parent_record_offset":7528,"ancestor_record_offset":7112,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":7112,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":7112,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":7528,"parent_record_offset":7464,"ancestor_record_offset":7112,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":7112,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":7436,"parent_record_offset":7404,"ancestor_record_offset":7112,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":7112,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":7612,"parent_record_offset":7464,"ancestor_record_offset":7112,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":7584,"parent_record_offset":7464,"ancestor_record_offset":7112,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer_digits<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4949,"record_offset":10956,"rva":47117664,"va":5415826784}
// 编译实例 RVAs: 0x2cef560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*168(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5415826784,"raw_hex":"ff9424a8000000","size":7,"soft_fail":false,"va":5415827100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4949,"record_offset":11856,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":11104,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":11244,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":11792,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":15525,"source_line":140,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::decode_hex
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":11728,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":15518,"source_line":1204,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::impl$15::checked_mul_ext
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/parse.rs
//     evidence={"module":4949,"record_offset":11608,"parent_record_offset":10956,"ancestor_record_offset":10956,"inlinee":15535,"source_line":1384,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":4949,"record_offset":11760,"parent_record_offset":11728,"ancestor_record_offset":10956,"inlinee":12686,"source_line":1230,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4949,"record_offset":11692,"parent_record_offset":11636,"ancestor_record_offset":10956,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":11636,"parent_record_offset":11608,"ancestor_record_offset":10956,"inlinee":15536,"source_line":1160,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4949,"record_offset":11664,"parent_record_offset":11636,"ancestor_record_offset":10956,"inlinee":15537,"source_line":3050,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4949,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10956,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":11536,"parent_record_offset":11512,"ancestor_record_offset":10956,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4949,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":10956,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4949,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":10956,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4949,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":10956,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4949,"record_offset":11372,"parent_record_offset":11308,"ancestor_record_offset":10956,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4949,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":10956,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4949,"record_offset":11280,"parent_record_offset":11244,"ancestor_record_offset":10956,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":10956,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":11456,"parent_record_offset":11308,"ancestor_record_offset":10956,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4949,"record_offset":11428,"parent_record_offset":11308,"ancestor_record_offset":10956,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=3
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3613,"record_offset":5132,"rva":41020224,"va":5409729344}
// 编译实例 RVAs: 0x271eb40, 0x271ebd0, 0x271ee10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6825363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729344,"raw_hex":"e893256800","size":5,"soft_fail":false,"target_va":5416554848,"va":5409729480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_value
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6825419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729344,"raw_hex":"e8cb256800","size":5,"soft_fail":false,"target_va":5416554864,"va":5409729440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271d600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729344,"raw_hex":"e888eaffff","size":5,"soft_fail":false,"target_va":5409723904,"va":5409729395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271e520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409730064,"raw_hex":"e8d2f6ffff","size":5,"soft_fail":false,"target_va":5409727776,"va":5409730121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271e9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729488,"raw_hex":"e8c2fdffff","size":5,"soft_fail":false,"target_va":5409728976,"va":5409729545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e0dfa0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7271404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729344,"raw_hex":"e8ecf36e00","size":5,"soft_fail":false,"target_va":5417000864,"va":5409729455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e0e0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7271383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409729488,"raw_hex":"e8d7f36e00","size":5,"soft_fail":false,"target_va":5417001136,"va":5409729748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":5360,"parent_record_offset":5132,"ancestor_record_offset":5132,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":6012,"parent_record_offset":5572,"ancestor_record_offset":5572,"inlinee":129599,"source_line":168,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3613,"record_offset":5536,"parent_record_offset":5132,"ancestor_record_offset":5132,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":5132,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":5132,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":5132,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":5132,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":5388,"parent_record_offset":5360,"ancestor_record_offset":5132,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":6064,"parent_record_offset":6012,"ancestor_record_offset":5572,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":6204,"parent_record_offset":6012,"ancestor_record_offset":5572,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":5572,"inlinee":6489,"source_line":2266,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$52::unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":6100,"parent_record_offset":6012,"ancestor_record_offset":5572,"inlinee":6485,"source_line":2849,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":5572,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":5572,"inlinee":6487,"source_line":32,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":9560,"rva":41065664,"va":5409774784}
// 编译实例 RVAs: 0x2729cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26548287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8c1e76afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409774842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34096444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8c4baf7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409775143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9656084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8eca86cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409775391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8cb0c0000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409775456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6750684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8dc016700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409774991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9656618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e8d6a66cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409775109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::to_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t3813893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774784,"raw_hex":"e805323a00","size":5,"soft_fail":false,"target_va":5413588864,"va":5409774966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":9788,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":10348,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":10320,"parent_record_offset":9560,"ancestor_record_offset":9560,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":9560,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":9560,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":9560,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":10716,"parent_record_offset":10628,"ancestor_record_offset":9560,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10176,"parent_record_offset":10144,"ancestor_record_offset":9560,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":9560,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":9560,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":9560,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":9560,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":9560,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":10144,"parent_record_offset":10096,"ancestor_record_offset":9560,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":10064,"parent_record_offset":10000,"ancestor_record_offset":9560,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":10096,"parent_record_offset":10064,"ancestor_record_offset":9560,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":10404,"parent_record_offset":10376,"ancestor_record_offset":9560,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":9820,"parent_record_offset":9788,"ancestor_record_offset":9560,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":9928,"parent_record_offset":9820,"ancestor_record_offset":9560,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":10684,"parent_record_offset":10656,"ancestor_record_offset":9560,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":10376,"parent_record_offset":10348,"ancestor_record_offset":9560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":9892,"parent_record_offset":9864,"ancestor_record_offset":9560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":10264,"parent_record_offset":10096,"ancestor_record_offset":9560,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":9864,"parent_record_offset":9820,"ancestor_record_offset":9560,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":9964,"parent_record_offset":9928,"ancestor_record_offset":9560,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":10000,"parent_record_offset":9964,"ancestor_record_offset":9560,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":11112,"rva":41066368,"va":5409775488}
// 编译实例 RVAs: 0x2729f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26548991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e801e56afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409775546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34097164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e8f4b7f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409775863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9656804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e81ca66cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409776111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e8fb090000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409776176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6749964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e80cff6600","size":5,"soft_fail":false,"target_va":5416525680,"va":5409775711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9657338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e806a46cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409775829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::to_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t3813893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409775488,"raw_hex":"e805323a00","size":5,"soft_fail":false,"target_va":5413589584,"va":5409775686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":11340,"parent_record_offset":11112,"ancestor_record_offset":11112,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":11900,"parent_record_offset":11112,"ancestor_record_offset":11112,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":11872,"parent_record_offset":11112,"ancestor_record_offset":11112,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":11112,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":11112,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":11112,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":12268,"parent_record_offset":12180,"ancestor_record_offset":11112,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":11728,"parent_record_offset":11696,"ancestor_record_offset":11112,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":12208,"parent_record_offset":12180,"ancestor_record_offset":11112,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":11112,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":11112,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":11752,"parent_record_offset":11728,"ancestor_record_offset":11112,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":11112,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":11696,"parent_record_offset":11648,"ancestor_record_offset":11112,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":11616,"parent_record_offset":11552,"ancestor_record_offset":11112,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":11648,"parent_record_offset":11616,"ancestor_record_offset":11112,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":11956,"parent_record_offset":11928,"ancestor_record_offset":11112,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":11372,"parent_record_offset":11340,"ancestor_record_offset":11112,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":11480,"parent_record_offset":11372,"ancestor_record_offset":11112,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":11112,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":11928,"parent_record_offset":11900,"ancestor_record_offset":11112,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":11444,"parent_record_offset":11416,"ancestor_record_offset":11112,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":11816,"parent_record_offset":11648,"ancestor_record_offset":11112,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":11416,"parent_record_offset":11372,"ancestor_record_offset":11112,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":11516,"parent_record_offset":11480,"ancestor_record_offset":11112,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":11552,"parent_record_offset":11516,"ancestor_record_offset":11112,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":1920,"rva":41061792,"va":5409770912}
// 编译实例 RVAs: 0x2728da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26543457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e89ffa6afe","size":5,"soft_fail":false,"target_va":5383227792,"va":5409771244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26544415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e8e1f66afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409770970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34092748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e834c9f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409771447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9652517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e8dbb66cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409771824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271c990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-50619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e8453affff","size":5,"soft_fail":false,"target_va":5409720720,"va":5409771334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e8eb1b0000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409771584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c0d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e83a320000","size":5,"soft_fail":false,"target_va":5409784016,"va":5409771153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6754401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e861106700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409771274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9652922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770912,"raw_hex":"e846b56cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409771413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":2176,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":2904,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":2876,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1920,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":1920,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":1920,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":3272,"parent_record_offset":3184,"ancestor_record_offset":1920,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":2576,"parent_record_offset":2544,"ancestor_record_offset":1920,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":1920,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":1920,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1920,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1920,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":1920,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":2544,"parent_record_offset":2496,"ancestor_record_offset":1920,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":2460,"parent_record_offset":2392,"ancestor_record_offset":1920,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":2496,"parent_record_offset":2460,"ancestor_record_offset":1920,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":2960,"parent_record_offset":2932,"ancestor_record_offset":1920,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":3625,"record_offset":2808,"parent_record_offset":2772,"ancestor_record_offset":1920,"inlinee":98374,"source_line":154,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":2212,"parent_record_offset":2176,"ancestor_record_offset":1920,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":2320,"parent_record_offset":2212,"ancestor_record_offset":1920,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":3240,"parent_record_offset":3212,"ancestor_record_offset":1920,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":2932,"parent_record_offset":2904,"ancestor_record_offset":1920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":1920,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":2664,"parent_record_offset":2496,"ancestor_record_offset":1920,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":2256,"parent_record_offset":2212,"ancestor_record_offset":1920,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":2356,"parent_record_offset":2320,"ancestor_record_offset":1920,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":2736,"parent_record_offset":2700,"ancestor_record_offset":1920,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":2392,"parent_record_offset":2356,"ancestor_record_offset":1920,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":2700,"parent_record_offset":2392,"ancestor_record_offset":1920,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":16840,"rva":41068912,"va":5409778032}
// 编译实例 RVAs: 0x272a970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26551533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e813db6afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409778088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34099692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e814aef7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409778391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9659330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e83e9c6cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409778637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271cd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-56615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e8d922ffff","size":5,"soft_fail":false,"target_va":5409721600,"va":5409778210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e81d000000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409778702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6747440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e830f56600","size":5,"soft_fail":false,"target_va":5416525680,"va":5409778235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9659866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409778032,"raw_hex":"e8269a6cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409778357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":17100,"parent_record_offset":16840,"ancestor_record_offset":16840,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":17744,"parent_record_offset":16840,"ancestor_record_offset":16840,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":17716,"parent_record_offset":16840,"ancestor_record_offset":16840,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":16840,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":16840,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":16840,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":18112,"parent_record_offset":18024,"ancestor_record_offset":16840,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":17488,"parent_record_offset":17456,"ancestor_record_offset":16840,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":18052,"parent_record_offset":18024,"ancestor_record_offset":16840,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":16840,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":16840,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":17512,"parent_record_offset":17488,"ancestor_record_offset":16840,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":18000,"parent_record_offset":17976,"ancestor_record_offset":16840,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":17456,"parent_record_offset":17408,"ancestor_record_offset":16840,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":17376,"parent_record_offset":17312,"ancestor_record_offset":16840,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":17408,"parent_record_offset":17376,"ancestor_record_offset":16840,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":17800,"parent_record_offset":17772,"ancestor_record_offset":16840,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":17132,"parent_record_offset":17100,"ancestor_record_offset":16840,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":17240,"parent_record_offset":17132,"ancestor_record_offset":16840,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":18080,"parent_record_offset":18052,"ancestor_record_offset":16840,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":17772,"parent_record_offset":17744,"ancestor_record_offset":16840,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":17204,"parent_record_offset":17176,"ancestor_record_offset":16840,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":17576,"parent_record_offset":17408,"ancestor_record_offset":16840,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":17176,"parent_record_offset":17132,"ancestor_record_offset":16840,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":17276,"parent_record_offset":17240,"ancestor_record_offset":16840,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":16840,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":17312,"parent_record_offset":17276,"ancestor_record_offset":16840,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":17612,"parent_record_offset":17312,"ancestor_record_offset":16840,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":260,"rva":41061088,"va":5409770208}
// 编译实例 RVAs: 0x2728ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26543709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8a3f96afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409770264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34091868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8a4ccf7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409770567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9651506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8ceba6cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409770813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271d000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e86944ffff","size":5,"soft_fail":false,"target_va":5409722368,"va":5409770386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8ad1e0000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409770878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6755264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8c0136700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409770411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9652042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409770208,"raw_hex":"e8b6b86cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409770533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":1156,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":1128,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":1524,"parent_record_offset":1436,"ancestor_record_offset":260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":900,"parent_record_offset":868,"ancestor_record_offset":260,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":260,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":868,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":788,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":820,"parent_record_offset":788,"ancestor_record_offset":260,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":260,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":544,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":652,"parent_record_offset":544,"ancestor_record_offset":260,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":1492,"parent_record_offset":1464,"ancestor_record_offset":260,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":1184,"parent_record_offset":1156,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":260,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":988,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":588,"parent_record_offset":544,"ancestor_record_offset":260,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":688,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":724,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":1024,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":15172,"rva":41068208,"va":5409777328}
// 编译实例 RVAs: 0x272a6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26550829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8d3dd6afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409777384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34098988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8d4b0f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409777687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9658626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8fe9e6cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409777933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271d390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-54234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8262cffff","size":5,"soft_fail":false,"target_va":5409723280,"va":5409777509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8dd020000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409777998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6748141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8edf76600","size":5,"soft_fail":false,"target_va":5416525680,"va":5409777534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9659162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409777328,"raw_hex":"e8e69c6cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409777653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":15432,"parent_record_offset":15172,"ancestor_record_offset":15172,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":16076,"parent_record_offset":15172,"ancestor_record_offset":15172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":16048,"parent_record_offset":15172,"ancestor_record_offset":15172,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":15172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":15172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":15172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":16444,"parent_record_offset":16356,"ancestor_record_offset":15172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":15820,"parent_record_offset":15788,"ancestor_record_offset":15172,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":16384,"parent_record_offset":16356,"ancestor_record_offset":15172,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":15172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":15172,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":15172,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":16332,"parent_record_offset":16308,"ancestor_record_offset":15172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":15788,"parent_record_offset":15740,"ancestor_record_offset":15172,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":15708,"parent_record_offset":15644,"ancestor_record_offset":15172,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":15740,"parent_record_offset":15708,"ancestor_record_offset":15172,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":16132,"parent_record_offset":16104,"ancestor_record_offset":15172,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":15464,"parent_record_offset":15432,"ancestor_record_offset":15172,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":15572,"parent_record_offset":15464,"ancestor_record_offset":15172,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":16412,"parent_record_offset":16384,"ancestor_record_offset":15172,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":15172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":15536,"parent_record_offset":15508,"ancestor_record_offset":15172,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":15908,"parent_record_offset":15740,"ancestor_record_offset":15172,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":15508,"parent_record_offset":15464,"ancestor_record_offset":15172,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":15608,"parent_record_offset":15572,"ancestor_record_offset":15172,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":15172,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":15644,"parent_record_offset":15608,"ancestor_record_offset":15172,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":15944,"parent_record_offset":15644,"ancestor_record_offset":15172,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":12664,"rva":41067088,"va":5409776208}
// 编译实例 RVAs: 0x272a250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26549709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e833e26afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409776264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34097868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e834b5f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409776567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9657506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e85ea36cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409776813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271ce30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-54487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e8292bffff","size":5,"soft_fail":false,"target_va":5409721904,"va":5409776386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e83d070000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409776878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6749264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e850fc6600","size":5,"soft_fail":false,"target_va":5416525680,"va":5409776411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9658042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776208,"raw_hex":"e846a16cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409776533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":12912,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":13556,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":13528,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":12664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":13972,"parent_record_offset":13948,"ancestor_record_offset":12664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":13996,"parent_record_offset":13972,"ancestor_record_offset":12664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":13924,"parent_record_offset":13836,"ancestor_record_offset":12664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13300,"parent_record_offset":13268,"ancestor_record_offset":12664,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13864,"parent_record_offset":13836,"ancestor_record_offset":12664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":12664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":12664,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":12664,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":12664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":13268,"parent_record_offset":13220,"ancestor_record_offset":12664,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":13188,"parent_record_offset":13124,"ancestor_record_offset":12664,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":13220,"parent_record_offset":13188,"ancestor_record_offset":12664,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":13612,"parent_record_offset":13584,"ancestor_record_offset":12664,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":12944,"parent_record_offset":12912,"ancestor_record_offset":12664,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":13052,"parent_record_offset":12944,"ancestor_record_offset":12664,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":13892,"parent_record_offset":13864,"ancestor_record_offset":12664,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":13584,"parent_record_offset":13556,"ancestor_record_offset":12664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":13016,"parent_record_offset":12988,"ancestor_record_offset":12664,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":13388,"parent_record_offset":13220,"ancestor_record_offset":12664,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":12988,"parent_record_offset":12944,"ancestor_record_offset":12664,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":13088,"parent_record_offset":13052,"ancestor_record_offset":12664,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":12664,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":13124,"parent_record_offset":13088,"ancestor_record_offset":12664,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":13424,"parent_record_offset":13124,"ancestor_record_offset":12664,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":6140,"rva":41064064,"va":5409773184}
// 编译实例 RVAs: 0x2729680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26545688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e8e8f16afe","size":5,"soft_fail":false,"target_va":5383227792,"va":5409773475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26546687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e801ee6afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409773242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34094972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e884c0f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409773671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9654741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e82bae6cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409774048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271c990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e88e31ffff","size":5,"soft_fail":false,"target_va":5409720720,"va":5409773565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e83b130000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409773808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c0d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e859290000","size":5,"soft_fail":false,"target_va":5409784016,"va":5409773426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6752170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e8aa076700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409773505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9655146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409773184,"raw_hex":"e896ac6cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409773637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":6400,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":7112,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":7084,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":6140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":6140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":7552,"parent_record_offset":7528,"ancestor_record_offset":6140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":7480,"parent_record_offset":7392,"ancestor_record_offset":6140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":6140,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":6140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":6140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":6140,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":6140,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":6140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":6764,"parent_record_offset":6716,"ancestor_record_offset":6140,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":6684,"parent_record_offset":6616,"ancestor_record_offset":6140,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":6716,"parent_record_offset":6684,"ancestor_record_offset":6140,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":7168,"parent_record_offset":7140,"ancestor_record_offset":6140,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":3625,"record_offset":7016,"parent_record_offset":6984,"ancestor_record_offset":6140,"inlinee":98374,"source_line":154,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":6436,"parent_record_offset":6400,"ancestor_record_offset":6140,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":6544,"parent_record_offset":6436,"ancestor_record_offset":6140,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":7448,"parent_record_offset":7420,"ancestor_record_offset":6140,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":7140,"parent_record_offset":7112,"ancestor_record_offset":6140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":6508,"parent_record_offset":6480,"ancestor_record_offset":6140,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":6884,"parent_record_offset":6716,"ancestor_record_offset":6140,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":6480,"parent_record_offset":6436,"ancestor_record_offset":6140,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":6580,"parent_record_offset":6544,"ancestor_record_offset":6140,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":6952,"parent_record_offset":6920,"ancestor_record_offset":6140,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":6616,"parent_record_offset":6580,"ancestor_record_offset":6140,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":6920,"parent_record_offset":6616,"ancestor_record_offset":6140,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":4404,"rva":41063136,"va":5409772256}
// 编译实例 RVAs: 0x27292e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26544799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e861f56afe","size":5,"soft_fail":false,"target_va":5383227792,"va":5409772586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26545759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e8a1f16afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409772314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34094076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e804c4f7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409772775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df3790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9657593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e807a36cff","size":5,"soft_fail":false,"target_va":5400115088,"va":5409772676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9653845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e8abb16cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409773152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e8bb160000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409772912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272e270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e8734e0000","size":5,"soft_fail":false,"target_va":5409792624,"va":5409772536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272e520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e848510000","size":5,"soft_fail":false,"target_va":5409793312,"va":5409772499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6753059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e8230b6700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409772616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9654250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409772256,"raw_hex":"e816b06cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409772741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":4648,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":5376,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":5348,"parent_record_offset":4404,"ancestor_record_offset":4404,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":4404,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":4404,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":4404,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":5744,"parent_record_offset":5656,"ancestor_record_offset":4404,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5048,"parent_record_offset":5016,"ancestor_record_offset":4404,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5684,"parent_record_offset":5656,"ancestor_record_offset":4404,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":4404,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":4404,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4404,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":4404,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":5016,"parent_record_offset":4968,"ancestor_record_offset":4404,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":4932,"parent_record_offset":4864,"ancestor_record_offset":4404,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":4968,"parent_record_offset":4932,"ancestor_record_offset":4404,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":5432,"parent_record_offset":5404,"ancestor_record_offset":4404,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] asset::mesh::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/mesh.rs
//     evidence={"module":3625,"record_offset":5280,"parent_record_offset":5244,"ancestor_record_offset":4404,"inlinee":98367,"source_line":37,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":4684,"parent_record_offset":4648,"ancestor_record_offset":4404,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":4792,"parent_record_offset":4684,"ancestor_record_offset":4404,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":5712,"parent_record_offset":5684,"ancestor_record_offset":4404,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":5404,"parent_record_offset":5376,"ancestor_record_offset":4404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":4756,"parent_record_offset":4728,"ancestor_record_offset":4404,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":5136,"parent_record_offset":4968,"ancestor_record_offset":4404,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":4728,"parent_record_offset":4684,"ancestor_record_offset":4404,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":4828,"parent_record_offset":4792,"ancestor_record_offset":4404,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":5208,"parent_record_offset":5172,"ancestor_record_offset":4404,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":4864,"parent_record_offset":4828,"ancestor_record_offset":4404,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":5172,"parent_record_offset":4864,"ancestor_record_offset":4404,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<f32,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":7876,"rva":41064960,"va":5409774080}
// 编译实例 RVAs: 0x2729a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26547581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e883ea6afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409774136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34095740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e884bdf7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409774439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9655378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e8aeab6cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409774685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e88d0f0000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409774750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6751390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e89e046700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409774285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h8c40ad8fc7445e79 (.llvm.10326317537195889376)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9655914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e896a96cff","size":5,"soft_fail":false,"target_va":5400118496,"va":5409774405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::from::_$LT$impl$u20$core..convert..From$LT$f32$GT$$u20$for$u20$serde_json..value..Value$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-26530777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409774080,"raw_hex":"e8272c6bfe","size":5,"soft_fail":false,"target_va":5383243488,"va":5409774260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":8096,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":8796,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":8768,"parent_record_offset":7876,"ancestor_record_offset":7876,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":7876,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":7876,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":7876,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3625,"record_offset":9164,"parent_record_offset":9076,"ancestor_record_offset":7876,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":8484,"parent_record_offset":8452,"ancestor_record_offset":7876,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":7876,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":7876,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":7876,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":7876,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":7876,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":8452,"parent_record_offset":8404,"ancestor_record_offset":7876,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":8372,"parent_record_offset":8308,"ancestor_record_offset":7876,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":7876,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":7876,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":8128,"parent_record_offset":8096,"ancestor_record_offset":7876,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":8236,"parent_record_offset":8128,"ancestor_record_offset":7876,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3625,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":7876,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":7876,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":7876,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":8572,"parent_record_offset":8404,"ancestor_record_offset":7876,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":8172,"parent_record_offset":8128,"ancestor_record_offset":7876,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":8272,"parent_record_offset":8236,"ancestor_record_offset":7876,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":7876,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$51::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":7876,"inlinee":7136,"source_line":11,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_f32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":7876,"inlinee":13165,"source_line":151,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":8308,"parent_record_offset":8272,"ancestor_record_offset":7876,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":8608,"parent_record_offset":8308,"ancestor_record_offset":7876,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":14320,"rva":41067792,"va":5409776912}
// 编译实例 RVAs: 0x272a510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26550409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776912,"raw_hex":"e877df6afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409776964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9657922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776912,"raw_hex":"e8bea16cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409777229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776912,"raw_hex":"e89e050000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409777293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6748561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409776912,"raw_hex":"e891f96600","size":5,"soft_fail":false,"target_va":5416525680,"va":5409777114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":14536,"parent_record_offset":14320,"ancestor_record_offset":14320,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":15140,"parent_record_offset":14320,"ancestor_record_offset":14320,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":14320,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":14320,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3625,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":14320,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":14888,"parent_record_offset":14836,"ancestor_record_offset":14320,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":14800,"parent_record_offset":14748,"ancestor_record_offset":14320,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":14836,"parent_record_offset":14800,"ancestor_record_offset":14320,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":14564,"parent_record_offset":14536,"ancestor_record_offset":14320,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":14676,"parent_record_offset":14564,"ancestor_record_offset":14320,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":14640,"parent_record_offset":14608,"ancestor_record_offset":14320,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":15000,"parent_record_offset":14836,"ancestor_record_offset":14320,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":14608,"parent_record_offset":14564,"ancestor_record_offset":14320,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":14712,"parent_record_offset":14676,"ancestor_record_offset":14320,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":14320,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":14748,"parent_record_offset":14712,"ancestor_record_offset":14320,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":15036,"parent_record_offset":14748,"ancestor_record_offset":14320,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3625,"record_offset":3668,"rva":41062736,"va":5409771856}
// 编译实例 RVAs: 0x2729150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde_core..ser..Serializer$GT$::serialize_seq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-26545353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409771856,"raw_hex":"e837f36afe","size":5,"soft_fail":false,"target_va":5383226560,"va":5409771908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df4810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9652850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409771856,"raw_hex":"e88eb56cff","size":5,"soft_fail":false,"target_va":5400119312,"va":5409772157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409771856,"raw_hex":"e86e190000","size":5,"soft_fail":false,"target_va":5409778736,"va":5409772221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6753619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409771856,"raw_hex":"e8530d6700","size":5,"soft_fail":false,"target_va":5416525680,"va":5409772056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":3884,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":8810,"source_line":2520,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$2::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":4372,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":13167,"source_line":339,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":4148,"parent_record_offset":4096,"ancestor_record_offset":3668,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3625,"record_offset":4184,"parent_record_offset":4148,"ancestor_record_offset":3668,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":3668,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3625,"record_offset":4024,"parent_record_offset":3912,"ancestor_record_offset":3668,"inlinee":8816,"source_line":2528,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3625,"record_offset":3988,"parent_record_offset":3956,"ancestor_record_offset":3668,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3625,"record_offset":4232,"parent_record_offset":4184,"ancestor_record_offset":3668,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3625,"record_offset":3956,"parent_record_offset":3912,"ancestor_record_offset":3668,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::Serializer::collect_seq::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/mod.rs
//     evidence={"module":3625,"record_offset":4060,"parent_record_offset":4024,"ancestor_record_offset":3668,"inlinee":12744,"source_line":1303,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":3625,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3668,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$2::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":3625,"record_offset":4096,"parent_record_offset":4060,"ancestor_record_offset":3668,"inlinee":13164,"source_line":331,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":3625,"record_offset":4268,"parent_record_offset":4096,"ancestor_record_offset":3668,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/03625.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2150,"record_offset":912,"rva":31413184,"va":5400122304}
// 编译实例 RVAs: 0x1df53c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16894953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e81734fefe","size":5,"soft_fail":false,"target_va":5383227792,"va":5400122740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24443893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e80b048bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400122576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24444204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8d4028bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5400122903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31400374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e84ade20fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400122561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30779565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e853572afe","size":5,"soft_fail":false,"target_va":5369343504,"va":5400123064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f690
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21388983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e849a1b9fe","size":5,"soft_fail":false,"target_va":5378733712,"va":5400122690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409942b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21369821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e823ecb9fe","size":5,"soft_fail":false,"target_va":5378753200,"va":5400123016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271c990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9598164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8d4749200","size":5,"soft_fail":false,"target_va":5409720720,"va":5400122551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1427286d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9646822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8e6329300","size":5,"soft_fail":false,"target_va":5409769168,"va":5400122341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ca80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9664004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e804769300","size":5,"soft_fail":false,"target_va":5409786496,"va":5400122487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272db30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9668234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e88a869300","size":5,"soft_fail":false,"target_va":5409790768,"va":5400122529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16497831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8a7bcfb00","size":5,"soft_fail":false,"target_va":5416620769,"va":5400122933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2150,"record_offset":1564,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1952,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1152,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":2150,"record_offset":1180,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$3::from_archive_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":2150,"record_offset":1088,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":28445,"source_line":92,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":912,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":912,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":912,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":912,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":912,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":912,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":912,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1820,"parent_record_offset":1732,"ancestor_record_offset":912,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":912,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":2150,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":912,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":1360,"parent_record_offset":1328,"ancestor_record_offset":912,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":912,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":912,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":912,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2150,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":912,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":2150,"record_offset":1252,"parent_record_offset":1216,"ancestor_record_offset":912,"inlinee":98374,"source_line":26,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2150,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":912,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2150,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":912,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":2150,"record_offset":1216,"parent_record_offset":1180,"ancestor_record_offset":912,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::to_json_value<asset::mesh::MeshData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2150,"record_offset":260,"rva":31412624,"va":5400121744}
// 编译实例 RVAs: 0x1df5190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16894292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8ac36fefe","size":5,"soft_fail":false,"target_va":5383227792,"va":5400122079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24443303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e859068bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400121986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31399784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e898e020fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400121971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30778780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8645a2afe","size":5,"soft_fail":false,"target_va":5369343504,"va":5400122279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098cd80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21398830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8d27ab9fe","size":5,"soft_fail":false,"target_va":5378723200,"va":5400122025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df3790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e822e5ffff","size":5,"soft_fail":false,"target_va":5400115088,"va":5400121961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142728450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9646746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e89a329300","size":5,"soft_fail":false,"target_va":5409768528,"va":5400121777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9667035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8db819300","size":5,"soft_fail":false,"target_va":5409788944,"va":5400121904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9667604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e814849300","size":5,"soft_fail":false,"target_va":5409789552,"va":5400121943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16498620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8bcbffb00","size":5,"soft_fail":false,"target_va":5416620769,"va":5400122144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":476,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":2150,"record_offset":504,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$3::from_archive_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":2150,"record_offset":412,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":28445,"source_line":92,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":744,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":712,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":2150,"record_offset":640,"parent_record_offset":608,"ancestor_record_offset":260,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":672,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] asset::mesh::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/mesh.rs
//     evidence={"module":2150,"record_offset":568,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":98367,"source_line":10,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":440,"parent_record_offset":412,"ancestor_record_offset":260,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":2150,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":260,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/02150.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 信息 ID: a8dc85559a31570700ffa4f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] hashbrown::raw::RawTableInner::fallible_with_capacity -> ["alloc::alloc::Global"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> > @ 0x2a7b1b2 -> 0x26501f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global>,std::hash::random::RandomState> > @ 0x2e09a0d -> 0x26501f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> > @ 0x2e0a02d -> 0x26501f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> > @ 0x2e0a7dd -> 0x26501f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_any<serde::private::de::content::ContentVisitor>
// 信息 ID: deed3859bdda01af94c31188
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_any -> ["serde::private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$21::visit_some<ref_mut$<ron::de::Deserializer> > @ 0x27223f6 -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$21::visit_map<ron::de::SerdeEnumContent> @ 0x272296c -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabRtConfig> @ 0x2723fe8 -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> > @ 0x2724568 -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabAudioConfig> @ 0x27249da -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_key_seed<serde::private::de::content::ContentVisitor> @ 0x2a7a21e -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_value_seed<serde::private::de::content::ContentVisitor> @ 0x2a7a407 -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$5::next_element_seed<serde::private::de::content::ContentVisitor> @ 0x2a7a6a5 -> 0x2a7a970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_byte_buf<serde::private::de::content::ContentVisitor>
// 信息 ID: 451bbbd66c14319a9d88c631
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_byte_buf -> ["serde::private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 信息 ID: 2e0355b74d1bcdc036b263d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["asset::prefab::config::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::options::Options::from_str_seed<core::marker::PhantomData<asset::prefab::config::PrefabConfig>,asset::prefab::config::PrefabConfig> @ 0x26519c0 -> 0x2a80d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 信息 ID: eb8458da1f53e9578f9b2698
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u8"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$5::next_element_seed -> ["core::marker::PhantomData<u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<u8,ron::de::CommaSeparated> @ 0x2acc915 -> 0x2a7a830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$5::next_element_seed<serde::private::de::content::ContentVisitor>
// 信息 ID: 7445ab29a2461c9c76d8e909
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$5::next_element_seed -> ["serde::private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$21::visit_seq<ron::de::CommaSeparated> @ 0x2722d19 -> 0x2a7a640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<core::marker::PhantomData<alloc::string::String> >
// 信息 ID: 49fd069b1b656282455ae6fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_key_seed -> ["core::marker::PhantomData<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> > @ 0x2a7b1fa -> 0x2a79df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<serde::private::de::content::ContentVisitor>
// 信息 ID: 2871c4d0f8e14cbdcb7fd527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$6::next_key_seed -> ["serde::private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$21::visit_map<ron::de::CommaSeparated> @ 0x2722542 -> 0x2a7a090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<serde::private::de::content::ContentVisitor>
// 信息 ID: f7a8cbf1935d57c3d56d8048
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$6::next_value_seed -> ["serde::private::de::content::ContentVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$21::visit_map<ron::de::CommaSeparated> @ 0x2722584 -> 0x2a7a350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u8>
// 信息 ID: c4494e968954b480d664df55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> > @ 0x2a7a88a -> 0x2cefb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i16>
// 信息 ID: 1aa0e3d19f68f27b88b57a0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cefe07 -> 0x2cee330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i32>
// 信息 ID: 191148a3bf87ae974e26c0cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cefe7b -> 0x2cee450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i64>
// 信息 ID: 3dc40b8b7c21876d308207f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cefeff -> 0x2ceeb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i8>
// 信息 ID: c6fe1fdd4200f999df2e7027
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cefc66 -> 0x2cee690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u16>
// 信息 ID: a9fd414b65c82c9752e27970
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cf0072 -> 0x2cee570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u32>
// 信息 ID: 62b39137c6d3ebb48dd0df82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cf00de -> 0x2cee210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u64>
// 信息 ID: 4560d4831eb33b6b4bbb83bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2cf016e -> 0x2ceea00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u8>
// 信息 ID: 81cf6d95521534a80e43f001
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2ceffad -> 0x2ceec40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<i16>
// 信息 ID: 5f23d5fc2f70b8e62d736733
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["i16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<i16> @ 0x2cee438 -> 0x2cef360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<i32>
// 信息 ID: 0748e13f46ca609b98f7452d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["i32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<i32> @ 0x2cee558 -> 0x2cef170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<i64>
// 信息 ID: 4562ea9e111521c7a532bd3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["i64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<i64> @ 0x2ceec28 -> 0x2ceef70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<u16>
// 信息 ID: a62b832c12ae656f07bdb189
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["u16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<u16> @ 0x2cee678 -> 0x2cef740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<u32>
// 信息 ID: 8079820ddc719deda46de342
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<u32> @ 0x2cee318 -> 0x2cef930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<u64>
// 信息 ID: 9dedd5677c18163111846a86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<u64> @ 0x2ceeb08 -> 0x2ceed60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer_digits<u8>
// 信息 ID: cfdbc3947822f00329f88bc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer_digits -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::parse_integer<u8> @ 0x2ceed48 -> 0x2cef560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>
// 信息 ID: 4247e1e3520f6c37c01841a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::impl$41::deserialize_enum -> ["enum2$<ron::error::Error>", "asset::prefab::config::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > > @ 0x271e422 -> 0x271eb40 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2720abb -> 0x271ebd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2720bf8 -> 0x271ebd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271f6e5 -> 0x271ee10 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271fa91 -> 0x271ee10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > >
// 信息 ID: 58884fa779cd9fe6c7c4222e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["f32", "2"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["array$<f32,2>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > @ 0x272aef1 -> 0x2729cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > >
// 信息 ID: 56517508884c372f6eca6cc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["f32", "3"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["array$<f32,3>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > @ 0x272b791 -> 0x2729f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> > >
// 信息 ID: c57262ce41eddfbc59d328bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::Grass2CardPointPieceGroup", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> > @ 0x272c713 -> 0x2728da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> > >
// 信息 ID: 63577d92356c808cc15cb1d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::Grass2CardPointPositionRange", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> > @ 0x272c4b3 -> 0x272a970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> > >
// 信息 ID: 31983eef4bdaab8e9f113947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::Grass2Lod0PartAsset", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> > @ 0x272da23 -> 0x2728ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> > >
// 信息 ID: a04415247219390d4cb4cf26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::Grass2RuntimeExemplarAsset", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> > @ 0x272c973 -> 0x272a6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> > >
// 信息 ID: 262da06a2992706bbf5af283
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::Grass2SlotAsset", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> > @ 0x272cbd3 -> 0x272a250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> > >
// 信息 ID: 5db750dc9e6acf9022d5af4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::foliage::PackedGrass2CardPointPosition", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> > @ 0x272d093 -> 0x2729680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> > >
// 信息 ID: 002b5d6ab744a93400212bbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["asset::mesh::MeshAttribute", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::mesh::MeshAttribute,alloc::alloc::Global> > @ 0x272d7c3 -> 0x27292e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<f32,alloc::alloc::Global> > >
// 信息 ID: 0d50362055df03b7e7d765c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["f32", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<f32,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<f32,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::vec::Vec<f32,alloc::alloc::Global> > @ 0x272b341 -> 0x2729a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> > >
// 信息 ID: a45c6adc7d80413168f3b221
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u16", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u16,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<u16,alloc::alloc::Global> > @ 0x272e163 -> 0x272a510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> > >
// 信息 ID: 010e30d3446a57873ed6728a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u32", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u32,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_core::ser::Serializer::collect_seq -> ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<u32,alloc::alloc::Global> > @ 0x272d563 -> 0x2729150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>
// 信息 ID: 35d9653aba27cae01554a672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] turbosloth::export::to_json_value -> ["asset::foliage::Grass2RuntimeConfigAsset"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::to_json_value<asset::mesh::MeshData>
// 信息 ID: 473267127f41ddc945855677
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] turbosloth::export::to_json_value -> ["asset::mesh::MeshData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
